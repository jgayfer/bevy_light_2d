use bevy::{
    ecs::{bundle::Bundle, component::Component},
    render::{
        color::Color,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    transform::components::{GlobalTransform, Transform},
};

#[derive(Component, Default, Clone, Copy)]
pub struct PointLight2d {
    pub color: Color,
    pub energy: f32,
    pub radius: f32,
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
            brightness: 0.0,
        }
    }
}
