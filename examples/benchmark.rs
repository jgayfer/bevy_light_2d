use bevy::{
    camera::primitives::{Frustum, Sphere},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseWheel,
    prelude::*,
};
use bevy_light_2d::prelude::*;

const GRID: usize = 100;
const SPACING: f32 = 100.0;
const RADIUS: f32 = 80.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Light2dPlugin,
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (zoom, update_readout))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Light2d {
            ambient_light: AmbientLight2d {
                brightness: 0.1,
                ..default()
            },
        },
    ));

    let offset = GRID as f32 * SPACING / 2.0;
    for i in 0..GRID * GRID {
        let x = (i % GRID) as f32 * SPACING - offset;
        let y = (i / GRID) as f32 * SPACING - offset;
        commands.spawn((
            PointLight2d {
                radius: RADIUS,
                intensity: 2.0,
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
        ));
    }

    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn zoom(
    mut wheel: MessageReader<MouseWheel>,
    mut projection: Single<&mut Projection, With<Camera2d>>,
) {
    let Projection::Orthographic(orthographic) = &mut **projection else {
        return;
    };

    for scroll in wheel.read() {
        orthographic.scale = (orthographic.scale * (1.0 - scroll.y * 0.1)).clamp(0.1, 100.0);
    }
}

fn update_readout(
    diagnostics: Res<DiagnosticsStore>,
    frustum: Single<&Frustum, With<Camera2d>>,
    lights: Query<(&GlobalTransform, &PointLight2d)>,
    mut text: Single<&mut Text>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
        .unwrap_or_default();

    let in_view = lights
        .iter()
        .filter(|(transform, light)| {
            frustum.intersects_sphere(
                &Sphere {
                    center: transform.translation().into(),
                    radius: light.radius,
                },
                false,
            )
        })
        .count();

    text.0 = format!(
        "{fps:.0} fps\n{in_view} of {} lights in view\nscroll to zoom",
        GRID * GRID
    );
}
