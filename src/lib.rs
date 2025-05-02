#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![expect(deprecated)]

pub mod light;
pub mod occluder;
pub mod plugin;
mod render;

/// A module which exports commonly used dependencies.
pub mod prelude {
    pub use crate::light::{AmbientLight2d, Light2d, PointLight2d, PointLight2dBundle};
    pub use crate::occluder::{LightOccluder2d, LightOccluder2dBundle, LightOccluder2dShape};
    pub use crate::plugin::Light2dPlugin;
}
