//! Repro for https://github.com/jgayfer/bevy_light_2d/issues/26
//!
//! A `PointLight2d` is attached directly to a sprite entity, with a light
//! radius much larger than the sprite. The camera pans right until the sprite
//! leaves the frustum.
//!
//! Before the fix, extraction filtered on `ViewVisibility`, so the light was
//! dropped the moment the *sprite's* `Aabb` left the frustum, even though the
//! light's radius still reached well into view. Watch for the glow on the left
//! edge popping out at once.
//!
//! After the fix, the glow fades out smoothly as the light's own radius clears
//! the edge.

use bevy::prelude::*;
use bevy_light_2d::prelude::*;

const LIGHT_RADIUS: f32 = 300.0;
const CAMERA_SPEED: f32 = 100.0;
const CAMERA_RESET_X: f32 = 1200.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, pan_camera)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Light2d {
            ambient_light: AmbientLight2d {
                brightness: 0.02,
                ..default()
            },
        },
    ));

    // Something for the light to fall on.
    commands.spawn((
        Sprite {
            color: Color::srgb(0.7, 0.7, 0.75),
            custom_size: Some(Vec2::new(4000.0, 1000.0)),
            ..default()
        },
        Transform::from_xyz(1000.0, 0.0, -1.0),
    ));

    // The light rides on the sprite entity. This is what issue #26 is about.
    commands.spawn((
        Sprite {
            image: asset_server.load("candle.png"),
            ..default()
        },
        PointLight2d {
            radius: LIGHT_RADIUS,
            intensity: 3.0,
            falloff: 4.0,
            color: Color::srgb(1.0, 0.85, 0.4),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn pan_camera(
    time: Res<Time>,
    mut last_zone: Local<Option<&'static str>>,
    camera: Single<&mut Transform, With<Camera2d>>,
    window: Single<&Window>,
) {
    let mut transform = camera.into_inner();
    transform.translation.x += CAMERA_SPEED * time.delta_secs();
    if transform.translation.x > CAMERA_RESET_X {
        transform.translation.x = 0.0;
    }

    // The sprite sits at x = 0, so the camera's x is also the distance between
    // them. Report which side of each boundary we're on.
    let half_width = window.width() / 2.0;
    let sprite_gone = transform.translation.x > half_width + 32.0;
    let light_gone = transform.translation.x > half_width + LIGHT_RADIUS;

    let zone = match (sprite_gone, light_gone) {
        (false, _) => "sprite on screen -> lit",
        (true, false) => "sprite off screen, light radius still reaches -> MUST STILL BE LIT",
        (true, true) => "light radius cleared the edge -> dark is correct",
    };

    if *last_zone != Some(zone) {
        *last_zone = Some(zone);
        println!("camera x {:>7.1}  |  {zone}", transform.translation.x);
    }
}
