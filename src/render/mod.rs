use bevy::{
    asset::{Handle, weak_handle},
    render::render_resource::Shader,
};

pub mod empty_buffer;
pub mod extract;
pub mod light_map;
pub mod lighting;
pub mod sdf;

pub const TYPES_SHADER: Handle<Shader> = weak_handle!("606bf813-c0cc-40c8-9fd6-ffcb6a5d66d8");

pub const VIEW_TRANSFORMATIONS_SHADER: Handle<Shader> =
    weak_handle!("16d31d1e-b859-4c6b-90ed-a66b93e0b86f");
