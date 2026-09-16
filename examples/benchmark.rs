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
// A quarter as many occluders as lights, sitting between the lights.
const OCCLUDER_GRID: usize = GRID / 2;
const OCCLUDER_SPACING: f32 = SPACING * 2.0;
const OCCLUDER_HALF_SIZE: f32 = 10.0;

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
                cast_shadows: true,
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
        ));
    }

    // Offset by half a light cell so occluders fall between lights rather than on top of them.
    let occluder_offset = offset - SPACING / 2.0;
    for i in 0..OCCLUDER_GRID * OCCLUDER_GRID {
        let x = (i % OCCLUDER_GRID) as f32 * OCCLUDER_SPACING - occluder_offset;
        let y = (i / OCCLUDER_GRID) as f32 * OCCLUDER_SPACING - occluder_offset;
        commands.spawn((
            LightOccluder2d {
                shape: LightOccluder2dShape::Rectangle {
                    half_size: Vec2::splat(OCCLUDER_HALF_SIZE),
                },
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
    occluders: Query<(&GlobalTransform, &LightOccluder2d)>,
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

    let occluders_in_view = occluders
        .iter()
        .filter(|(transform, occluder)| {
            let LightOccluder2dShape::Rectangle { half_size } = occluder.shape;
            // A bounding sphere around the rectangle is a cheap conservative test.
            frustum.intersects_sphere(
                &Sphere {
                    center: transform.translation().into(),
                    radius: half_size.length(),
                },
                false,
            )
        })
        .count();

    text.0 = format!(
        "{fps:.0} fps\n{in_view} of {} lights in view\n{occluders_in_view} of {} occluders in view\nscroll to zoom",
        GRID * GRID,
        OCCLUDER_GRID * OCCLUDER_GRID
    );
}
