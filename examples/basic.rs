use bevy::prelude::*;
use bevy_light_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            intensity: 3.0,
            radius: 100.0,
            ..default()
        },
        ..default()
    });
}
