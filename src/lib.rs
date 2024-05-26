#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod component;
pub mod plugin;
mod render;

/// A module which exports commonly used dependencies.
pub mod prelude {
    pub use crate::component::{AmbientLight2d, PointLight2d, PointLight2dBundle};
    pub use crate::plugin::Light2dPlugin;
}
