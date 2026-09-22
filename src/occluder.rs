//! A module which contains occluder components.

use bevy::{
    camera::visibility::{self, InheritedVisibility, ViewVisibility, Visibility, VisibilityClass},
    ecs::{bundle::Bundle, component::Component},
    math::Vec2,
    prelude::{Deref, ReflectComponent, ReflectDefault},
    reflect::Reflect,
    render::sync_world::SyncToRenderWorld,
    shape::Aabb2d,
    transform::components::{GlobalTransform, Transform},
};

/// A light occluder that prevents light passing through it, casting shadows.
///
/// This is commonly used as a component within [`LightOcluder2dBundle`].
#[derive(Default, Component)]
#[require(
    SyncToRenderWorld,
    Transform,
    Visibility,
    VisibilityClass,
    LightOccluder2dVisibility
)]
#[component(on_add = visibility::add_visibility_class::<LightOccluder2d>)]
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

impl LightOccluder2dShape {
    pub(crate) fn aabb(&self, center: Vec2) -> Aabb2d {
        match self {
            Self::Rectangle { half_size } => Aabb2d::new(center, *half_size),
        }
    }
}

/// Whether an occluder is "visible". That is, whether an occluder is in range of
/// any visible light sources.
///
/// Unlike light sources, occluders can't be culled by checking if they intersect
/// the view, as off screen occluders can still occlude visible light sources that
/// originate outside the view.
///
/// This computed component keeps track of which occluders should be considered
/// for rendering. It should not be set manually.
///
/// To change the "visibility" of an occluder, use [`Visibility`] instead.
#[derive(Component, Deref, Debug, Default, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
pub struct LightOccluder2dVisibility(pub(crate) bool);

impl LightOccluder2dVisibility {
    /// An occluder that does not intersect any visible light sources.
    pub const HIDDEN: Self = Self(false);
    /// An occluder that intersects with at least one visible light source.
    pub const VISIBLE: Self = Self(true);

    /// Returns `true` if the occluder intersects with at least one visible light
    /// source. Otherwise, returns `false`.
    #[inline]
    pub fn get(self) -> bool {
        self.0
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
