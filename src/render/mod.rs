use bevy::{asset::Handle, render::render_resource::Shader};

pub mod extract;
pub mod light_map;
pub mod lighting;
pub mod sdf;

pub const TYPES_SHADER: Handle<Shader> =
    Handle::weak_from_u128(134542958402584092759402858489640143033);

pub const VIEW_TRANSFORMATIONS_SHADER: Handle<Shader> =
    Handle::weak_from_u128(134542958402584092759402858489640143039);
