use bevy::{prelude::*, render::camera::Viewport};
use bevy_light_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Main camera
    commands.spawn((Camera2d, Light2d::default()));

    // Minimap camera, without a Light2d marker (disabling the lighting pipeline)
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::Default,
            viewport: Some(Viewport {
                physical_position: UVec2::new(10, 10),
                physical_size: UVec2::new(200, 200),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(3.0)),
        Light2d::default(),
    ));

    // The "player"
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(25.0, 30.0)),
            ..Default::default()
        },
        PointLight2d {
            radius: 375.0,
            ..default()
        },
    ));
}
