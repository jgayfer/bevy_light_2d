//! A module which contains occluder components.

use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::Vec2,
    render::{
        sync_world::SyncToRenderWorld,
        view::{self, InheritedVisibility, ViewVisibility, Visibility, VisibilityClass},
    },
    transform::components::{GlobalTransform, Transform},
};

/// A light occluder that prevents light passing through it, casting shadows.
///
/// This is commonly used as a component within [`LightOcluder2dBundle`].
#[derive(Default, Component)]
#[require(SyncToRenderWorld, Transform, Visibility, VisibilityClass)]
#[component(on_add = view::add_visibility_class::<LightOccluder2d>)]
pub struct LightOccluder2d {
    /// The shape of the light occluder.
    pub shape: LightOccluder2dShape,
}

/// Shape data for a light occluder.
pub enum LightOccluder2dShape {
    /// A rectangular light occluder.
    Rectangle {
        /// Half of the width and height of the rectangle.
        half_size: Vec2,
    },
}

impl Default for LightOccluder2dShape {
    fn default() -> Self {
        Self::Rectangle {
            half_size: Vec2::splat(0.0),
        }
    }
}

/// A bundle of components for rendering a [`LightOccluder2d`] entity.
#[derive(Bundle, Default)]
#[deprecated(
    since = "0.5.0",
    note = "Use the `LightOccluder2d` component instead. Inserting `LightOccluder2d` will also insert the other components required automatically."
)]
pub struct LightOccluder2dBundle {
    /// Specifies the rendering properties of the light occluder
    pub light_occluder: LightOccluder2d,
    /// The local transform of the light occluder, relative to its parent.
    pub transform: Transform,
    /// The absolute transform of the light occluder. This should generally not be written to directly.
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible.
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering.
    pub view_visibility: ViewVisibility,
}
