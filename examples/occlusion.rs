use bevy::{
    color::palettes::css::{BLUE, YELLOW},
    prelude::*,
};
use bevy_light_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, move_lights)
        .run();
}

#[derive(Component)]
struct YellowLight;

#[derive(Component)]
struct BlueLight;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        PointLight2dBundle {
            point_light: PointLight2d {
                intensity: 20.0,
                radius: 1000.0,
                falloff: 10.0,
                cast_shadows: true,
                color: Color::Srgba(YELLOW),
            },
            transform: Transform {
                translation: Vec3::new(0.0, 200.0, 0.0),
                ..default()
            },
            ..default()
        },
        YellowLight,
    ));

    commands.spawn((
        PointLight2dBundle {
            point_light: PointLight2d {
                intensity: 20.0,
                radius: 1000.0,
                falloff: 10.0,
                cast_shadows: true,
                color: Color::Srgba(BLUE),
            },
            transform: Transform {
                translation: Vec3::new(0.0, -200.0, 0.0),
                ..default()
            },
            ..default()
        },
        BlueLight,
    ));

    commands.spawn(LightOccluder2dBundle {
        light_occluder: LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        transform: Transform::from_xyz(-400.0, 0., 0.0),
        ..default()
    });

    commands.spawn(LightOccluder2dBundle {
        light_occluder: LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        transform: Transform::from_xyz(-200.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(LightOccluder2dBundle {
        light_occluder: LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(LightOccluder2dBundle {
        light_occluder: LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        transform: Transform::from_xyz(200.0, 0.0, 0.0),
        ..default()
    });

    commands.spawn(LightOccluder2dBundle {
        light_occluder: LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        transform: Transform::from_xyz(400.0, 0.0, 0.0),
        ..default()
    });
}

fn move_lights(
    mut yellow_query: Query<&mut Transform, (With<YellowLight>, Without<BlueLight>)>,
    mut blue_query: Query<&mut Transform, (With<BlueLight>, Without<YellowLight>)>,
    time: Res<Time>,
) {
    for mut light_transform in &mut yellow_query {
        light_transform.translation.x = time.elapsed_seconds().sin() * 500.
    }
    for mut light_transform in &mut blue_query {
        light_transform.translation.x = time.elapsed_seconds().cos() * 500.
    }
}
