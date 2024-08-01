mod node;
mod pipeline;

use bevy::{
    asset::Handle,
    render::{render_graph::RenderLabel, render_resource::Shader},
};

pub use node::SdfNode;
pub use pipeline::SdfPipeline;

pub const SDF_SHADER: Handle<Shader> =
    Handle::weak_from_u128(231804371047309214783091483091843019281);

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct SdfPass;
