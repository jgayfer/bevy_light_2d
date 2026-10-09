mod node;
mod pipeline;
mod prepare;

use bevy::{
    ecs::{component::Component, resource::Resource},
    math::Vec3,
    render::{
        render_resource::{ShaderType, UniformBuffer},
        texture::CachedTexture,
    },
};

pub use node::sdf_pass;
pub use pipeline::SdfPipeline;
pub use prepare::prepare_occluder_meta;
pub use prepare::prepare_sdf_texture;

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
