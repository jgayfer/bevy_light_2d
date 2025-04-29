mod node;
mod pipeline;
mod prepare;

use bevy::{
    asset::{Handle, weak_handle},
    ecs::component::Component,
    render::{
        render_graph::RenderLabel,
        render_resource::{CachedRenderPipelineId, Shader},
    },
};

pub use node::LightingNode;
pub use pipeline::*;
pub use prepare::*;

pub const LIGHTING_SHADER: Handle<Shader> = weak_handle!("22ed6ffe-b47d-4b88-b986-5b0e87b3a240");

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct LightingPass;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LightingPipelineKey {
    pub hdr: bool,
}

#[derive(Component)]
pub struct LightingPipelineId(pub CachedRenderPipelineId);
