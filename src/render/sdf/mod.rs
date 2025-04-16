mod node;
mod pipeline;
mod prepare;

use bevy::{
    asset::{Handle, weak_handle},
    ecs::component::Component,
    render::{render_graph::RenderLabel, render_resource::Shader, texture::CachedTexture},
};

pub use node::SdfNode;
pub use pipeline::SdfPipeline;
pub use prepare::prepare_sdf_texture;

pub const SDF_SHADER: Handle<Shader> = weak_handle!("16251728-6dd9-481e-95a7-7c2e0ff8d920");

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct SdfPass;

#[derive(Component)]
pub struct SdfTexture {
    pub sdf: CachedTexture,
}
