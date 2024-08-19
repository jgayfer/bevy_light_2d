use bevy::prelude::*;
use bevy_light_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        AmbientLight2d {
            brightness: 0.1,
            ..default()
        },
    ));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::splat(150.)),
            color: Color::WHITE,
            ..default()
        },
        ..default()
    });

    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            color: Color::Srgba(Srgba::RED),
            radius: 50.,
            intensity: 1.0,
            ..default()
        },
        transform: Transform::from_xyz(-50., 25., 0.),
        ..default()
    });

    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            color: Color::WHITE,
            radius: 50.,
            intensity: 1.0,
            falloff: 5.0,
            ..default()
        },
        transform: Transform::from_xyz(25., 50., 0.),
        ..default()
    });

    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            color: Color::Srgba(Srgba::GREEN),
            radius: 75.,
            intensity: 1.0,
            ..default()
        },
        transform: Transform::from_xyz(-10., -25., 0.),
        ..default()
    });
}
