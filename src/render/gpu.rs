use super::extract::ExtractedPointLight2d;
use bevy::{
    prelude::*,
    render::{
        render_resource::{ShaderType, StorageBuffer},
        renderer::{RenderDevice, RenderQueue},
    },
};

#[derive(Default, Resource)]
pub struct GpuPointLights {
    pub buffer: StorageBuffer<Vec<GpuPointLight2d>>,
}

#[derive(Default, Clone, ShaderType)]
pub struct GpuPointLight2d {
    pub center: Vec2,
    pub radius: f32,
    pub color: LinearRgba,
    pub intensity: f32,
    pub falloff: f32,
}

pub fn prepare_point_lights(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    point_light_query: Query<&ExtractedPointLight2d>,
    mut gpu_point_lights: ResMut<GpuPointLights>,
) {
    let point_light_buffer = gpu_point_lights.buffer.get_mut();

    // Resources are global state, so we need to clear the data from the previous frame.
    point_light_buffer.clear();

    for point_light in &point_light_query {
        point_light_buffer.push(GpuPointLight2d {
            center: point_light.transform.translation().xy(),
            radius: point_light.radius,
            color: point_light.color,
            intensity: point_light.intensity,
            falloff: point_light.falloff,
        });
    }

    gpu_point_lights
        .buffer
        .write_buffer(&render_device, &render_queue);
}
