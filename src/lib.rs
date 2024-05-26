#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod light;
pub mod plugin;
mod render;

/// A module which exports commonly used dependencies.
pub mod prelude {
    pub use crate::light::{AmbientLight2d, PointLight2d, PointLight2dBundle};
    pub use crate::plugin::Light2dPlugin;
}
