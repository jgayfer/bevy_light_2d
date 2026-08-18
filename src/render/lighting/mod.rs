mod node;
mod pipeline;
mod prepare;

use bevy::{
    asset::{Handle, weak_handle},
    ecs::component::Component,
    prelude::Shader,
    render::render_resource::{CachedRenderPipelineId, TextureFormat},
};

pub use node::lighting_pass;
pub use pipeline::*;
pub use prepare::*;

pub const LIGHTING_SHADER: Handle<Shader> = weak_handle!("22ed6ffe-b47d-4b88-b986-5b0e87b3a240");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LightingPipelineKey {
    pub target_format: TextureFormat,
}

#[derive(Component)]
pub struct LightingPipelineId(pub CachedRenderPipelineId);
