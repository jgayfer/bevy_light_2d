//! A module which contains lighting components.

use bevy::{
    color::Color,
    ecs::{bundle::Bundle, component::Component},
    reflect::Reflect,
    render::{
        sync_world::SyncToRenderWorld,
        view::{self, InheritedVisibility, ViewVisibility, Visibility, VisibilityClass},
    },
    transform::components::{GlobalTransform, Transform},
};

/// A light that provides illumination in all directions.
///
/// This is commonly used as a component within [`PointLight2dBundle`].
///
/// # Attentuation
///
/// Light attenuation is based on a variation of inverse square falloff, where a light source will
/// only provide illumination from within its radius.
///
/// For more information on the formula used, see the blog post
/// [A better point light attenutation function](https://lisyarus.github.io/blog/posts/point-light-attenuation.html#section-the-solution)
/// by [lisyarus](https://lisyarus.github.io/blog/).
#[derive(Component, Clone, Reflect)]
#[require(SyncToRenderWorld, Transform, Visibility, VisibilityClass)]
#[component(on_add = view::add_visibility_class::<PointLight2d>)]
pub struct PointLight2d {
    /// The light's color tint.
    pub color: Color,
    /// The intensity of the light. The light's attenutation is multiplied by this value.
    /// The higher the intensity, the brighter the light.
    pub intensity: f32,
    /// The radius of the light. Illumination will only occur within the light's radius.
    pub radius: f32,
    /// How quickly illumination from the light should deteriorate over distance.
    /// A higher falloff value will result in less illumination at the light's maximum radius.
    pub falloff: f32,
    /// Whether the light should cast shadows.
    pub cast_shadows: bool,
}

impl Default for PointLight2d {
    /// Returns a 1x1 white [`PointLight2d`].
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 1.0,
            radius: 0.5,
            falloff: 0.0,
            cast_shadows: false,
        }
    }
}

/// A bundle of components for rendering a [`PointLight2d`] entity.
#[derive(Bundle, Default)]
#[deprecated(
    since = "0.5.0",
    note = "Use the `PointLight2d` component instead. Inserting `PointLight2d` will also insert the other components required automatically."
)]
pub struct PointLight2dBundle {
    /// Specifies the rendering properties of the point light, such as color and radius.
    pub point_light: PointLight2d,
    /// The local transform of the point light, relative to its parent.
    pub transform: Transform,
    /// The absolute transform of the point light. This should generally not be written to directly.
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible.
    pub visibility: Visibility,
    /// Inherited visibility of an entity.
    pub inherited_visibility: InheritedVisibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering.
    pub view_visibility: ViewVisibility,
}

/// A component representing how much ambient light to apply to a `Camera2d`.
///
/// - For a darker scene, use a brightness value between `0.0` and `1.0`.
/// - For a brighter scene, use a brightness greater than `1.0`.
/// - A brightness value of `0.0` will result in a completely black scene.
#[derive(Component, Clone, Reflect)]
pub struct AmbientLight2d {
    /// The ambient light's color tint.
    pub color: Color,
    /// The brightness of the ambient light. This value is multiplied against the linear RGB
    /// representation of the ambient light's color.
    pub brightness: f32,
}

impl Default for AmbientLight2d {
    /// Return a white ambient light with a brightness of `1.0` (which is effectively
    /// the same as not having any ambient light).
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            brightness: 1.0,
        }
    }
}
