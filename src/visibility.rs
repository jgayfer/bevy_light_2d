use bevy::{
    camera::{
        primitives::Aabb,
        visibility::{NoAutoAabb, NoFrustumCulling},
    },
    ecs::{
        entity::Entity,
        query::{Changed, Or, Without},
        system::{Commands, Query},
    },
    math::Vec3A,
};

use crate::light::{PointLight2d, SpotLight2d};

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
    use bevy::app::{App, Update};

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
}
