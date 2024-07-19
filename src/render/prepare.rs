use super::extract::ExtractedPointLight2d;
use bevy::{
    prelude::*,
    render::{
        render_resource::{BindingResource, ShaderType, StorageBuffer, UniformBuffer},
        renderer::{RenderDevice, RenderQueue},
    },
};

const MAX_UNIFORM_POINT_LIGHTS: usize = 256;

#[derive(Resource)]
pub enum GpuPointLights {
    Uniform(UniformBuffer<GpuPointLightsUniform>),
    Storage(StorageBuffer<GpuPointLightsStorage>),
}

impl GpuPointLights {
    pub fn new(device: &RenderDevice) -> Self {
        let limits = device.limits();
        if limits.max_storage_buffers_per_shader_stage == 0 {
            GpuPointLights::Uniform(UniformBuffer::from(GpuPointLightsUniform::default()))
        } else {
            GpuPointLights::Storage(StorageBuffer::from(GpuPointLightsStorage::default()))
        }
    }

    fn set(&mut self, mut point_lights: Vec<ExtractedPointLight2d>) {
        match self {
            GpuPointLights::Uniform(buffer) => {
                let len = point_lights.len().min(MAX_UNIFORM_POINT_LIGHTS);
                let src = &point_lights[..len];
                let dst = &mut buffer.get_mut().data[..len];
                dst.copy_from_slice(src);
            }
            GpuPointLights::Storage(buffer) => {
                buffer.get_mut().data.clear();
                buffer.get_mut().data.append(&mut point_lights);
            }
        }
    }

    fn write_buffer(&mut self, render_device: &RenderDevice, render_queue: &RenderQueue) {
        match self {
            GpuPointLights::Uniform(buffer) => {
                buffer.write_buffer(render_device, render_queue);
            }
            GpuPointLights::Storage(buffer) => {
                buffer.write_buffer(render_device, render_queue);
            }
        }
    }

    pub fn binding(&self) -> Option<BindingResource> {
        match self {
            GpuPointLights::Uniform(buffer) => buffer.binding(),
            GpuPointLights::Storage(buffer) => buffer.binding(),
        }
    }
}

#[derive(ShaderType)]
pub struct GpuPointLightsUniform {
    data: Box<[ExtractedPointLight2d; MAX_UNIFORM_POINT_LIGHTS]>,
}

impl Default for GpuPointLightsUniform {
    fn default() -> Self {
        Self {
            data: Box::new([ExtractedPointLight2d::default(); MAX_UNIFORM_POINT_LIGHTS]),
        }
    }
}

#[derive(Default, ShaderType)]
pub struct GpuPointLightsStorage {
    #[size(runtime)]
    pub data: Vec<ExtractedPointLight2d>,
}

pub fn prepare_point_lights(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    point_light_query: Query<&ExtractedPointLight2d>,
    mut gpu_point_lights: ResMut<GpuPointLights>,
) {
    let point_lights = point_light_query
        .iter()
        .cloned()
        .collect::<Vec<ExtractedPointLight2d>>();

    gpu_point_lights.set(point_lights.clone());
    gpu_point_lights.write_buffer(&render_device, &render_queue);
}
