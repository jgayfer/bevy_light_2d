mod node;
mod pipeline;
mod prepare;

use bevy::{
    asset::Handle,
    ecs::component::Component,
    render::{render_graph::RenderLabel, render_resource::Shader, texture::CachedTexture},
};

pub use node::SdfNode;
pub use pipeline::SdfPipeline;
pub use prepare::prepare_sdf_texture;

pub const SDF_SHADER: Handle<Shader> =
    Handle::weak_from_u128(231804371047309214783091483091843019281);

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct SdfPass;

#[derive(Component)]
pub struct SdfTexture {
    pub sdf: CachedTexture,
}
