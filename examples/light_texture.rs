use bevy::prelude::*;
use bevy_light_2d::light::PointLight2dImageTexture;
use bevy_light_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>,) {
    commands.spawn((Camera2d, Light2d::default()));

    commands.spawn((
        PointLight2d {
            intensity: 3.0,
            radius: 100.0,
            ..default()
        },
        PointLight2dImageTexture {
            texture: asset_server.load("blast_circle.png"),
        }
    ));
}
