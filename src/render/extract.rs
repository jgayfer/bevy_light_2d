use bevy::{
    prelude::*,
    render::{Extract, render_resource::ShaderType, sync_world::RenderEntity},
};

use crate::{
    light::{Light2d, PointLight2d, SpotLight2d},
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
pub struct ExtractedSpotLight2d {
    pub center: Vec2,
    pub radius: f32,
    pub color: LinearRgba,
    pub intensity: f32,
    pub falloff: f32,
    pub direction: Vec2,
    pub cos_inner_angle: f32,
    pub cos_outer_angle: f32,
    pub source_width: f32,
    pub cast_shadows: u32,
}

pub fn extract_spot_lights(
    mut commands: Commands,
    q: Extract<
        Query<(
            &RenderEntity,
            &SpotLight2d,
            &GlobalTransform,
            &ViewVisibility,
        )>,
    >,
) {
    for (render_entity, spot_light, global_transform, view_visibility) in &q {
        if !view_visibility.get() {
            continue;
        }
        commands
            .entity(render_entity.id())
            .insert(ExtractedSpotLight2d {
                center: global_transform.translation().xy(),
                radius: spot_light.radius,
                color: spot_light.color.to_linear(),
                intensity: spot_light.intensity,
                falloff: spot_light.falloff,
                direction: Vec2::from_angle(spot_light.direction).normalize_or_zero(),
                cos_inner_angle: spot_light.cos_inner_angle,
                cos_outer_angle: spot_light.cos_outer_angle,
                source_width: spot_light.source_width,
                cast_shadows: if spot_light.cast_shadows { 1 } else { 0 },
            });
    }
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
    light_2d_query: Extract<Query<(&RenderEntity, &Light2d)>>,
) {
    for (render_entity, light_2d) in &light_2d_query {
        commands
            .entity(render_entity.id())
            .insert(ExtractedAmbientLight2d {
                color: light_2d.ambient_light.color.to_linear() * light_2d.ambient_light.brightness,
            });
    }
}
