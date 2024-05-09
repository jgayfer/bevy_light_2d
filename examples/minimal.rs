use bevy::prelude::*;
use bevy_light_2d::{Light2dPlugin, PointLight2d, PointLight2dBundle};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(100.)),
            ..default()
        },
        ..default()
    });

    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            color: Color::RED,
            radius: 200.,
            ..default()
        },
        transform: Transform::from_xyz(150., 0., 0.),
        ..default()
    });
}
