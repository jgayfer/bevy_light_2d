use bevy::{
    camera::{
        primitives::Aabb,
        visibility::{InheritedVisibility, NoAutoAabb, NoFrustumCulling, ViewVisibility},
    },
    ecs::{
        change_detection::DetectChangesMut,
        entity::Entity,
        query::{AnyOf, Changed, Or, Without},
        system::{Commands, Query},
    },
    math::{
        Vec3A, Vec3Swizzles,
        bounding::{BoundingCircle, IntersectsVolume},
    },
    transform::components::GlobalTransform,
};

use crate::{
    light::{PointLight2d, SpotLight2d},
    occluder::{LightOccluder2d, LightOccluder2dVisibility},
};

/// Add (or update) an [`Aabb`] for each light source, so that Bevy's
/// [`VisibilityPlugin`](bevy::camera::visibility::VisibilityPlugin) can perform
/// CPU culling.
pub fn calculate_light_bounds(
    mut commands: Commands,
    point_lights: Query<
        (Entity, &PointLight2d),
        (
            Or<(Without<Aabb>, Changed<PointLight2d>)>,
            Without<NoFrustumCulling>,
            Without<NoAutoAabb>,
        ),
    >,
    spot_lights: Query<
        (Entity, &SpotLight2d),
        (
            Or<(Without<Aabb>, Changed<SpotLight2d>)>,
            Without<NoFrustumCulling>,
            Without<NoAutoAabb>,
        ),
    >,
) {
    for (entity, point_light) in point_lights {
        commands
            .entity(entity)
            .try_insert(point_light_aabb(point_light));
    }

    for (entity, spot_light) in spot_lights {
        commands
            .entity(entity)
            .try_insert(spot_light_aabb(spot_light));
    }
}

/// Determine which [`LightOccluder2d`] entities intersect with at least one
/// visible light source.
///
/// Because off screen occluders can affect the view (despite not intersecting
/// with it), we can't rely on Bevy's Aabb based culling.
///
/// Instead we mark occluder "visibility" after light source visibility has been
/// computed, checking intersection only on visible light sources.
pub fn check_occluder_visibility(
    visible_lights: Query<(
        &GlobalTransform,
        &ViewVisibility,
        AnyOf<(&PointLight2d, &SpotLight2d)>,
    )>,
    mut occluders: Query<(
        &LightOccluder2d,
        &GlobalTransform,
        &InheritedVisibility,
        &mut LightOccluder2dVisibility,
    )>,
) {
    let light_bounds: Vec<BoundingCircle> = visible_lights
        .iter()
        .filter_map(|(transform, visibility, (point, spot))| {
            let (radius, cast_shadows) = match (point, spot) {
                (Some(p), _) => (p.radius, p.cast_shadows),
                (_, Some(s)) => (s.radius, s.cast_shadows),
                _ => return None,
            };

            (visibility.get() && cast_shadows)
                .then(|| BoundingCircle::new(transform.translation().xy(), radius))
        })
        .collect();

    occluders.par_iter_mut().for_each(
        |(occluder, transform, inherited_visibility, mut occluder_visibility)| {
            let aabb = occluder.shape.aabb(transform.translation().xy());

            let visible = inherited_visibility.get()
                && light_bounds.iter().any(|bounds| aabb.intersects(bounds));

            occluder_visibility.set_if_neq(LightOccluder2dVisibility(visible));
        },
    );
}

fn point_light_aabb(point_light: &PointLight2d) -> Aabb {
    Aabb {
        center: Vec3A::ZERO,
        half_extents: Vec3A::new(point_light.radius, point_light.radius, 0.0),
    }
}

fn spot_light_aabb(spot_light: &SpotLight2d) -> Aabb {
    Aabb {
        center: Vec3A::ZERO,
        // A conservative estimate (ignores light angle and direction).
        half_extents: Vec3A::new(spot_light.radius, spot_light.radius, 0.0),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::occluder::LightOccluder2dShape;
    use bevy::{
        app::{App, Update},
        math::Vec2,
    };

    #[test]
    fn calculate_light_bounds_creates_aabb_for_point_light_entity() {
        let mut app = App::new();
        app.add_systems(Update, calculate_light_bounds);

        let entity = app
            .world_mut()
            .spawn(PointLight2d {
                radius: 3.0,
                ..PointLight2d::default()
            })
            .id();

        assert!(app.world().get::<Aabb>(entity).is_none());

        app.update();

        let aabb = app.world().get::<Aabb>(entity).unwrap();

        assert_eq!(aabb.center, Vec3A::ZERO);
        assert_eq!(aabb.half_extents, Vec3A::new(3.0, 3.0, 0.0));
    }

    #[test]
    fn calculate_light_bounds_creates_aabb_for_spot_light_entity() {
        let mut app = App::new();
        app.add_systems(Update, calculate_light_bounds);

        let entity = app
            .world_mut()
            .spawn(SpotLight2d {
                radius: 3.0,
                ..SpotLight2d::default()
            })
            .id();

        assert!(app.world().get::<Aabb>(entity).is_none());

        app.update();

        let aabb = app.world().get::<Aabb>(entity).unwrap();

        assert_eq!(aabb.center, Vec3A::ZERO);
        assert_eq!(aabb.half_extents, Vec3A::new(3.0, 3.0, 0.0));
    }

    #[test]
    fn calculate_light_bounds_updates_aabb_for_changed_point_light() {
        let mut app = App::new();
        app.add_systems(Update, calculate_light_bounds);

        let entity = app
            .world_mut()
            .spawn(PointLight2d {
                radius: 1.0,
                ..PointLight2d::default()
            })
            .id();

        app.update();

        let first_aabb = *app.world().get::<Aabb>(entity).unwrap();

        app.world_mut()
            .get_mut::<PointLight2d>(entity)
            .unwrap()
            .radius = 2.0;

        app.update();

        let second_aabb = *app.world().get::<Aabb>(entity).unwrap();

        assert_ne!(first_aabb, second_aabb);
    }

    #[test]
    fn calculate_light_bounds_updates_aabb_for_changed_spot_light() {
        let mut app = App::new();
        app.add_systems(Update, calculate_light_bounds);

        let entity = app
            .world_mut()
            .spawn(SpotLight2d {
                radius: 1.0,
                ..SpotLight2d::default()
            })
            .id();

        app.update();

        let first_aabb = *app.world().get::<Aabb>(entity).unwrap();

        app.world_mut()
            .get_mut::<SpotLight2d>(entity)
            .unwrap()
            .radius = 2.0;

        app.update();

        let second_aabb = *app.world().get::<Aabb>(entity).unwrap();

        assert_ne!(first_aabb, second_aabb);
    }

    #[test]
    fn check_occluder_visibility_marks_occluder_in_range_of_visible_light() {
        let mut app = App::new();
        app.add_systems(Update, check_occluder_visibility);

        app.world_mut().spawn((
            PointLight2d {
                radius: 10.0,
                cast_shadows: true,
                ..PointLight2d::default()
            },
            ViewVisibility::VISIBLE,
        ));

        let entity = app
            .world_mut()
            .spawn((
                LightOccluder2d {
                    shape: LightOccluder2dShape::Rectangle {
                        half_size: Vec2::ONE,
                    },
                },
                GlobalTransform::from_xyz(5.0, 0.0, 0.0),
                InheritedVisibility::VISIBLE,
            ))
            .id();

        app.update();

        assert_eq!(
            *app.world()
                .get::<LightOccluder2dVisibility>(entity)
                .unwrap(),
            LightOccluder2dVisibility::VISIBLE
        );
    }

    #[test]
    fn check_occluder_visibility_hides_occluder_out_of_range_of_visible_light() {
        let mut app = App::new();
        app.add_systems(Update, check_occluder_visibility);

        app.world_mut().spawn((
            PointLight2d {
                radius: 10.0,
                cast_shadows: true,
                ..PointLight2d::default()
            },
            ViewVisibility::VISIBLE,
        ));

        let entity = app
            .world_mut()
            .spawn((
                LightOccluder2d {
                    shape: LightOccluder2dShape::Rectangle {
                        half_size: Vec2::ONE,
                    },
                },
                GlobalTransform::from_xyz(20.0, 0.0, 0.0),
                InheritedVisibility::VISIBLE,
            ))
            .id();

        app.update();

        assert_eq!(
            *app.world()
                .get::<LightOccluder2dVisibility>(entity)
                .unwrap(),
            LightOccluder2dVisibility::HIDDEN
        );
    }

    #[test]
    fn check_occluder_visibility_hides_occluder_in_range_of_hidden_light() {
        let mut app = App::new();
        app.add_systems(Update, check_occluder_visibility);

        app.world_mut().spawn((
            PointLight2d {
                radius: 10.0,
                cast_shadows: true,
                ..PointLight2d::default()
            },
            ViewVisibility::HIDDEN,
        ));

        let entity = app
            .world_mut()
            .spawn((
                LightOccluder2d {
                    shape: LightOccluder2dShape::Rectangle {
                        half_size: Vec2::ONE,
                    },
                },
                GlobalTransform::from_xyz(5.0, 0.0, 0.0),
                InheritedVisibility::VISIBLE,
            ))
            .id();

        app.update();

        assert_eq!(
            *app.world()
                .get::<LightOccluder2dVisibility>(entity)
                .unwrap(),
            LightOccluder2dVisibility::HIDDEN
        );
    }

    #[test]
    fn check_occluder_visibility_hides_hidden_occluder_in_range_of_visible_light() {
        let mut app = App::new();
        app.add_systems(Update, check_occluder_visibility);

        app.world_mut().spawn((
            PointLight2d {
                radius: 10.0,
                cast_shadows: true,
                ..PointLight2d::default()
            },
            ViewVisibility::VISIBLE,
        ));

        let entity = app
            .world_mut()
            .spawn((
                LightOccluder2d {
                    shape: LightOccluder2dShape::Rectangle {
                        half_size: Vec2::ONE,
                    },
                },
                GlobalTransform::from_xyz(5.0, 0.0, 0.0),
                InheritedVisibility::HIDDEN,
            ))
            .id();

        app.update();

        assert_eq!(
            *app.world()
                .get::<LightOccluder2dVisibility>(entity)
                .unwrap(),
            LightOccluder2dVisibility::HIDDEN
        );
    }
}
