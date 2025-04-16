mod node;
mod pipeline;
mod prepare;

use bevy::{
    asset::{Handle, weak_handle},
    ecs::{component::Component, resource::Resource},
    math::Vec3,
    render::{
        render_graph::RenderLabel,
        render_resource::{Shader, ShaderType, UniformBuffer},
        texture::CachedTexture,
    },
};

pub use node::LightMapNode;
pub use pipeline::LightMapPipeline;
pub use prepare::{prepare_light_map_texture, prepare_point_light_count};

pub const LIGHT_MAP_SHADER: Handle<Shader> = weak_handle!("48777bb3-8a37-4b4d-a4f2-f10ff1ee4360");

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct LightMapPass;

#[derive(Component)]
pub struct LightMapTexture {
    pub light_map: CachedTexture,
}

#[derive(Resource, Default)]
pub struct PointLightMetaBuffer {
    pub buffer: UniformBuffer<PointLightMeta>,
}

#[derive(Default, ShaderType)]
pub struct PointLightMeta {
    pub count: u32,
    // WebGL2 structs must be 16 byte aligned.
    _padding: Vec3,
}

impl PointLightMeta {
    pub fn new(count: u32) -> Self {
        Self {
            count,
            _padding: Vec3::ZERO,
        }
    }
}
