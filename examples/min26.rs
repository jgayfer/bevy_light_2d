use bevy::prelude::*;
use bevy_light_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, pan_camera)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Light2d {
            ambient_light: AmbientLight2d {
                brightness: 0.02,
                ..default()
            },
        },
    ));

    // A light attached to a sprite, with a radius far larger than the sprite.
    commands.spawn((
        Sprite::from_color(Color::srgb(0.8, 0.1, 0.1), Vec2::splat(32.0)),
        PointLight2d {
            radius: 300.0,
            intensity: 10.0,
            ..default()
        },
    ));
}

// Pan right until the sprite leaves the frustum, but the light radius doesn't.
fn pan_camera(camera: Single<&mut Transform, With<Camera2d>>, time: Res<Time>) {
    camera.into_inner().translation.x += 100.0 * time.delta_secs();
}
