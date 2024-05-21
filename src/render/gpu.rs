use super::extract::ExtractedPointLight2d;
use bevy::{
    prelude::*,
    render::{
        render_resource::{ShaderType, StorageBuffer},
        renderer::{RenderDevice, RenderQueue},
    },
};

#[derive(Default, Resource)]
pub struct LightingPassAssets {
    pub point_lights: StorageBuffer<Vec<GpuPointLight2d>>,
}

#[derive(Default, Clone, ShaderType)]
pub struct GpuPointLight2d {
    pub center: Vec2,
    pub radius: f32,
    pub color: Vec3,
    pub intensity: f32,
}

pub fn prepare_point_lights(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    point_light_query: Query<&ExtractedPointLight2d>,
    mut lighting_pass_assets: ResMut<LightingPassAssets>,
) {
    let point_light_buffer = lighting_pass_assets.point_lights.get_mut();

    // Resources are global state, so we need to clear the data from the previous frame.
    point_light_buffer.clear();

    for point_light in &point_light_query {
        point_light_buffer.push(GpuPointLight2d {
            center: point_light.transform.translation().xy(),
            radius: point_light.radius,
            color: point_light.color,
            intensity: point_light.intensity,
        });
    }

    lighting_pass_assets
        .point_lights
        .write_buffer(&render_device, &render_queue);
}
