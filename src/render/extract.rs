use bevy::{
    color::{Color, LinearRgba},
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query},
    },
    render::{render_resource::ShaderType, view::ViewVisibility, Extract},
    transform::components::GlobalTransform,
};

use crate::light::{AmbientLight2d, PointLight2d};

#[derive(Component, Default, Clone)]
pub struct ExtractedPointLight2d {
    pub transform: GlobalTransform,
    pub radius: f32,
    pub color: LinearRgba,
    pub intensity: f32,
    pub falloff: f32,
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
            color: point_light.color.linear(),
            transform: *global_transform,
            radius: point_light.radius,
            intensity: point_light.intensity,
            falloff: point_light.falloff,
        });
    }
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
                color: ambient_light.color.linear() * ambient_light.brightness,
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
