pub mod component;
pub mod plugin;
mod render;

pub mod prelude {
    pub use crate::component::{AmbientLight2d, PointLight2d, PointLight2dBundle};
    pub use crate::plugin::Light2dPlugin;
}
