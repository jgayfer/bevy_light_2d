mod node;
mod pipeline;
mod prepare;

use bevy::{
    asset::Handle,
    ecs::component::Component,
    ecs::resource::Resource,
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

pub const LIGHT_MAP_SHADER: Handle<Shader> =
    Handle::weak_from_u128(320609826414128764415270070474935914193);

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
