mod node;
mod pipeline;

use bevy::{
    asset::Handle,
    render::{render_graph::RenderLabel, render_resource::Shader},
};

pub use node::LightingNode;
pub use pipeline::LightingPipeline;

pub const LIGHTING_SHADER: Handle<Shader> =
    Handle::weak_from_u128(111120241052143214281687226997564407636);

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct LightingPass;
