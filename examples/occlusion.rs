use bevy::prelude::*;
use bevy_light_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, move_lights)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            intensity: 20.0,
            radius: 1000.0,
            falloff: 10.0,
            cast_shadows: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 100.0, 0.0),
            ..default()
        },
        ..default()
    });

    commands.spawn(LightOccluder2dBundle {
        light_occluder: LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        transform: Transform::from_xyz(-400.0, -50.0, 0.0),
        ..default()
    });

    commands.spawn(LightOccluder2dBundle {
        light_occluder: LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        transform: Transform::from_xyz(-200.0, -50.0, 0.0),
        ..default()
    });

    commands.spawn(LightOccluder2dBundle {
        light_occluder: LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        transform: Transform::from_xyz(0.0, -50.0, 0.0),
        ..default()
    });

    commands.spawn(LightOccluder2dBundle {
        light_occluder: LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        transform: Transform::from_xyz(200.0, -50.0, 0.0),
        ..default()
    });

    commands.spawn(LightOccluder2dBundle {
        light_occluder: LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        transform: Transform::from_xyz(400.0, -50.0, 0.0),
        ..default()
    });
}

fn move_lights(mut query: Query<&mut Transform, With<PointLight2d>>, time: Res<Time>) {
    for mut light_transform in &mut query {
        light_transform.translation.x = time.elapsed_seconds().sin() * 500.
    }
}
