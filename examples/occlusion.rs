use bevy::{
    color::palettes::css::{BLUE, GREEN, RED, YELLOW},
    input::mouse::MouseWheel,
    prelude::*,
};
use bevy_light_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_lights,
                rotate_occluder,
                control_camera_movement,
                control_camera_zoom,
            ),
        )
        .run();
}

#[derive(Component)]
struct YellowLight;

#[derive(Component)]
struct BlueLight;

#[derive(Component)]
struct RedLight;

#[derive(Component)]
struct GreenLight;

#[derive(Component)]
struct RotatingOccluder;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Light2d::default()));

    commands.spawn((
        PointLight2d {
            intensity: 20.0,
            radius: 500.0,
            falloff: 10.0,
            cast_shadows: true,
            color: Color::Srgba(YELLOW),
        },
        Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
        YellowLight,
    ));

    commands.spawn((
        PointLight2d {
            intensity: 20.0,
            radius: 500.0,
            falloff: 10.0,
            cast_shadows: true,
            color: Color::Srgba(BLUE),
        },
        Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
        BlueLight,
    ));

    commands.spawn((
        SpotLight2d {
            intensity: 20.0,
            radius: 500.0,
            falloff: 10.0,
            direction: 90.0,
            inner_angle: 180.0,
            outer_angle: 120.0,
            source_width: 10.0,
            cast_shadows: true,
            color: Color::Srgba(RED),
        },
        Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
        RedLight,
    ));

    commands.spawn((
        SpotLight2d {
            intensity: 20.0,
            radius: 500.0,
            falloff: 10.0,
            direction: 90.0,
            inner_angle: 180.0,
            outer_angle: 120.0,
            source_width: 10.0,
            cast_shadows: true,
            color: Color::Srgba(GREEN),
        },
        Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
        GreenLight,
    ));

    commands.spawn((
        LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        Transform::from_xyz(-400.0, 0., 0.0),
    ));

    commands.spawn((
        LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        Transform::from_xyz(-200.0, 0.0, 0.0),
    ));

    commands.spawn((
        LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        RotatingOccluder,
    ));

    commands.spawn((
        LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));

    commands.spawn((
        LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::splat(25.0),
            },
        },
        Transform::from_xyz(400.0, 0.0, 0.0),
    ));
}

fn move_lights(
    mut yellow_query: Query<
        &mut Transform,
        (
            With<YellowLight>,
            Without<BlueLight>,
            Without<RedLight>,
            Without<GreenLight>,
        ),
    >,
    mut blue_query: Query<
        &mut Transform,
        (
            With<BlueLight>,
            Without<YellowLight>,
            Without<RedLight>,
            Without<GreenLight>,
        ),
    >,
    mut red_query: Query<
        &mut Transform,
        (
            With<RedLight>,
            Without<GreenLight>,
            Without<BlueLight>,
            Without<YellowLight>,
        ),
    >,
    mut green_query: Query<
        &mut Transform,
        (
            With<GreenLight>,
            Without<RedLight>,
            Without<BlueLight>,
            Without<YellowLight>,
        ),
    >,
    time: Res<Time>,
) {
    for mut light_transform in &mut yellow_query {
        light_transform.translation.x = time.elapsed_secs().sin() * 500.
    }
    for mut light_transform in &mut blue_query {
        light_transform.translation.x = time.elapsed_secs().cos() * 500.
    }
    for mut light_transform in &mut red_query {
        light_transform.translation.x = time.elapsed_secs().cos() * 300.
    }
    for mut light_transform in &mut green_query {
        light_transform.translation.x = time.elapsed_secs().cos() * 750.
    }
}

fn rotate_occluder(mut query: Query<&mut Transform, With<RotatingOccluder>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_secs());
    }
}

const CAMERA_SPEED: f32 = 10.0;

fn control_camera_movement(
    mut camera_current: Local<Vec2>,
    mut camera_target: Local<Vec2>,
    mut query_cameras: Query<&mut Transform, With<Camera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::KeyW) {
        camera_target.y += CAMERA_SPEED;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        camera_target.y -= CAMERA_SPEED;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        camera_target.x -= CAMERA_SPEED;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        camera_target.x += CAMERA_SPEED;
    }

    // Smooth camera.
    let blend_ratio = 0.2;
    let movement = *camera_target - *camera_current;
    *camera_current += movement * blend_ratio;

    // Update all sprite cameras.
    for mut camera_transform in query_cameras.iter_mut() {
        camera_transform.translation.x = camera_current.x;
        camera_transform.translation.y = camera_current.y;
    }
}

const MIN_CAMERA_SCALE: f32 = 1.;
const MAX_CAMERA_SCALE: f32 = 20.;

fn control_camera_zoom(
    projections: Query<&mut Projection, With<Camera>>,
    time: Res<Time>,
    mut scroll_event_reader: MessageReader<MouseWheel>,
) {
    let mut projection_delta = 0.;

    for event in scroll_event_reader.read() {
        projection_delta += event.y * 3.;
    }

    if projection_delta == 0. {
        return;
    }

    for mut projection in projections {
        if let Projection::Orthographic(ref mut camera) = *projection {
            camera.scale = (camera.scale - projection_delta * time.delta_secs())
                .clamp(MIN_CAMERA_SCALE, MAX_CAMERA_SCALE);
        }
    }
}
