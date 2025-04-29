use bevy::{
    prelude::*,
    render::{Extract, render_resource::ShaderType, sync_world::RenderEntity},
};

use crate::{
    light::{AmbientLight2d, PointLight2d},
    occluder::{LightOccluder2d, LightOccluder2dShape},
};

#[derive(Component, Default, Clone, ShaderType)]
pub struct ExtractedPointLight2d {
    pub transform: Vec2,
    pub radius: f32,
    pub color: LinearRgba,
    pub intensity: f32,
    pub falloff: f32,
    pub cast_shadows: u32,
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
    point_light_query: Extract<
        Query<(
            &RenderEntity,
            &PointLight2d,
            &GlobalTransform,
            &ViewVisibility,
        )>,
    >,
) {
    for (render_entity, point_light, global_transform, view_visibility) in &point_light_query {
        if !view_visibility.get() {
            continue;
        }
        commands
            .entity(render_entity.id())
            .insert(ExtractedPointLight2d {
                color: point_light.color.to_linear(),
                transform: global_transform.translation().xy(),
                radius: point_light.radius,
                intensity: point_light.intensity,
                falloff: point_light.falloff,
                cast_shadows: if point_light.cast_shadows { 1 } else { 0 },
            });
    }
}

pub fn extract_light_occluders(
    mut commands: Commands,
    light_occluders_query: Extract<
        Query<(
            &RenderEntity,
            &LightOccluder2d,
            &GlobalTransform,
            &ViewVisibility,
        )>,
    >,
) {
    for (render_entity, light_occluder, global_transform, view_visibility) in &light_occluders_query
    {
        if !view_visibility.get() {
            continue;
        }

        let extracted_occluder = match light_occluder.shape {
            LightOccluder2dShape::Rectangle { half_size } => ExtractedLightOccluder2d {
                half_size,
                center: global_transform.translation().xy(),
            },
        };

        commands
            .entity(render_entity.id())
            .insert(extracted_occluder);
    }
}

pub fn extract_ambient_lights(
    mut commands: Commands,
    ambient_light_query: Extract<Query<(&RenderEntity, &AmbientLight2d)>>,
    camera_query: Extract<Query<&RenderEntity, (With<Camera2d>, Without<AmbientLight2d>)>>,
) {
    for (render_entity, ambient_light) in &ambient_light_query {
        commands
            .entity(render_entity.id())
            .insert(ExtractedAmbientLight2d {
                color: ambient_light.color.to_linear() * ambient_light.brightness,
            });
    }

    // Our lighting pass only runs on views with an ambient light component,
    // so let's add a no-op ambient light to any 2d cameras don't have one.
    for render_entity in &camera_query {
        commands
            .entity(render_entity.id())
            .insert(ExtractedAmbientLight2d {
                color: Color::WHITE.into(),
            });
    }
}
