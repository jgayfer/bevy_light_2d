use bevy::{
    ecs::{bundle::Bundle, component::Component},
    render::{
        color::Color,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    transform::components::{GlobalTransform, Transform},
};

#[derive(Component, Clone, Copy)]
pub struct PointLight2d {
    pub color: Color,
    pub intensity: f32,
    pub radius: f32,
    pub falloff: f32,
}

impl Default for PointLight2d {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 1.0,
            radius: 0.5,
            falloff: 0.0,
        }
    }
}

#[derive(Bundle, Default)]
pub struct PointLight2dBundle {
    pub point_light: PointLight2d,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

#[derive(Component, Clone, Copy)]
pub struct AmbientLight2d {
    pub color: Color,
    pub brightness: f32,
}

impl Default for AmbientLight2d {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            brightness: 1.0,
        }
    }
}
