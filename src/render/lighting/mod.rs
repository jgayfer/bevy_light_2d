mod node;
mod pipeline;
mod prepare;

use bevy::{
    asset::Handle,
    ecs::component::Component,
    render::{
        render_graph::RenderLabel,
        render_resource::{CachedRenderPipelineId, Shader},
    },
};

pub use node::LightingNode;
pub use pipeline::LightingPipeline;
pub use prepare::prepare_lighting_pipelines;

pub const LIGHTING_SHADER: Handle<Shader> =
    Handle::weak_from_u128(111120241052143214281687226997564407636);

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct LightingPass;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LightingPipelineKey {
    pub hdr: bool,
}

#[derive(Component)]
pub struct LightingPipelineId(pub CachedRenderPipelineId);
