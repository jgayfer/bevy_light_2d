mod node;
mod pipeline;
mod prepare;

use bevy::{
    ecs::component::Component,
    render::render_resource::{CachedRenderPipelineId, TextureFormat},
};

pub use node::lighting_pass;
pub use pipeline::*;
pub use prepare::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LightingPipelineKey {
    pub target_format: TextureFormat,
}

#[derive(Component)]
pub struct LightingPipelineId(pub CachedRenderPipelineId);
