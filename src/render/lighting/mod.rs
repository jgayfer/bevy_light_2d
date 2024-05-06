mod node;
mod pipeline;

use bevy::render::render_graph::RenderLabel;

pub use node::LightingNode;
pub use pipeline::LightingPipeline;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct LightingPass;
