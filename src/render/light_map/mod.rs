mod node;
mod pipeline;

use bevy::{
    asset::Handle,
    render::{render_graph::RenderLabel, render_resource::Shader},
};

pub use node::LightMapNode;
pub use pipeline::LightMapPipeline;

pub const LIGHT_MAP_SHADER: Handle<Shader> =
    Handle::weak_from_u128(320609826414128764415270070474935914193);

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct LightMapPass;
