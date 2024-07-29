use bevy::{
    prelude::*,
    render::{render_resource::ShaderType, Extract},
};

use crate::light::{AmbientLight2d, LightOccluder2d, PointLight2d};

#[derive(Component, Default, Clone, ShaderType)]
pub struct ExtractedPointLight2d {
    pub transform: Vec2,
    pub radius: f32,
    pub color: LinearRgba,
    pub intensity: f32,
    pub falloff: f32,
}

#[derive(Component, Default, Clone, ShaderType)]
pub struct ExtractedLightOccluder2d {
    pub half_size: Vec2,
    pub center: Vec2,
}

#[derive(Component, Default, Clone, ShaderType)]
pub struct ExtractedAmbientLight2d {
    pub color: LinearRgba,
}

pub fn extract_point_lights(
    mut commands: Commands,
    point_light_query: Extract<Query<(Entity, &PointLight2d, &GlobalTransform, &ViewVisibility)>>,
) {
    for (entity, point_light, global_transform, view_visibility) in &point_light_query {
        if !view_visibility.get() {
            continue;
        }
        commands.get_or_spawn(entity).insert(ExtractedPointLight2d {
            color: point_light.color.to_linear(),
            transform: global_transform.translation().xy(),
            radius: point_light.radius,
            intensity: point_light.intensity,
            falloff: point_light.falloff,
        });
    }

    // BufferVec won't write to the GPU if there aren't any point lights.
    // For now we can spawn an empty point light to get around this.
    commands.spawn(ExtractedPointLight2d {
        transform: Vec2::ZERO,
        intensity: 0.0,
        radius: 0.0,
        falloff: 0.0,
        color: LinearRgba::BLACK,
    });
}

pub fn extract_light_occluders(
    mut commands: Commands,
    light_occluders_query: Extract<
        Query<(Entity, &LightOccluder2d, &GlobalTransform, &ViewVisibility)>,
    >,
) {
    for (entity, light_occluder, global_transform, view_visibility) in &light_occluders_query {
        if !view_visibility.get() {
            continue;
        }

        commands
            .get_or_spawn(entity)
            .insert(ExtractedLightOccluder2d {
                half_size: light_occluder.half_size,
                center: global_transform.translation().xy(),
            });
    }

    // BufferVec won't write to the GPU if there aren't any point lights.
    // For now we can spawn an empty point light to get around this.
    commands.spawn(ExtractedLightOccluder2d::default());
}

pub fn extract_ambient_lights(
    mut commands: Commands,
    ambient_light_query: Extract<Query<(Entity, &AmbientLight2d)>>,
    camera_query: Extract<Query<Entity, (With<Camera2d>, Without<AmbientLight2d>)>>,
) {
    for (entity, ambient_light) in &ambient_light_query {
        commands
            .get_or_spawn(entity)
            .insert(ExtractedAmbientLight2d {
                color: ambient_light.color.to_linear() * ambient_light.brightness,
            });
    }

    // Our lighting pass only runs on views with an ambient light component,
    // so let's add a no-op ambient light to any 2d cameras don't have one.
    for entity in &camera_query {
        commands
            .get_or_spawn(entity)
            .insert(ExtractedAmbientLight2d {
                color: Color::WHITE.into(),
            });
    }
}
