mod node;
mod pipeline;
mod prepare;

use bevy::{
    asset::Handle,
    ecs::component::Component,
    render::{render_graph::RenderLabel, render_resource::Shader, texture::CachedTexture},
};

pub use node::LightMapNode;
pub use pipeline::LightMapPipeline;
pub use prepare::prepare_light_map_texture;

pub const LIGHT_MAP_SHADER: Handle<Shader> =
    Handle::weak_from_u128(320609826414128764415270070474935914193);

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct LightMapPass;

#[derive(Component)]
pub struct LightMapTexture {
    pub light_map: CachedTexture,
}
