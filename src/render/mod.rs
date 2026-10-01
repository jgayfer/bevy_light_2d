use bevy::{
    camera::{Camera2d, CameraDepthTexture},
    ecs::{query::With, system::Query},
    render::render_resource::TextureUsages,
};

pub mod empty_buffer;
pub mod extract;
pub mod gpu_array_buffer;
pub mod light_map;
pub mod lighting;
pub mod sdf;

pub fn configure_depth_texture(mut view_targets: Query<&mut CameraDepthTexture, With<Camera2d>>) {
    for mut depth_texture in &mut view_targets {
        let mut depth_texture_usages = TextureUsages::from(depth_texture.texture_usages);
        depth_texture_usages |= TextureUsages::TEXTURE_BINDING;
        depth_texture.texture_usages = depth_texture_usages.into();
    }
}
