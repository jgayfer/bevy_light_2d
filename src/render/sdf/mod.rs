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

pub use node::SdfNode;
pub use pipeline::SdfPipeline;
pub use prepare::prepare_occluder_meta;
pub use prepare::prepare_sdf_texture;

pub const SDF_SHADER: Handle<Shader> = weak_handle!("16251728-6dd9-481e-95a7-7c2e0ff8d920");

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct SdfPass;

#[derive(Component)]
pub struct SdfTexture {
    pub sdf: CachedTexture,
}
#[derive(Resource, Default)]
pub struct OccluderMetaBuffer {
    pub buffer: UniformBuffer<OccluderMeta>,
}

#[derive(Default, ShaderType)]
pub struct OccluderMeta {
    pub count: u32,
    // WebGL2 structs must be 16 byte aligned.
    _padding: Vec3,
}

impl OccluderMeta {
    pub fn new(count: u32) -> Self {
        Self {
            count,
            _padding: Vec3::ZERO,
        }
    }
}
