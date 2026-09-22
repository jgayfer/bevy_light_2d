use bevy::{color::palettes::css::YELLOW, prelude::*, sprite::SpriteMesh, window::PrimaryWindow};
use bevy_light_2d::prelude::*;

mod candle;
use candle::{Candle, CandlePlugin};

const TILE_INDEX: f32 = 0.0;
const ENTITY_INDEX: f32 = 1.0;

const ROOM_WIDTH: i32 = 11;
const ROOM_HEIGHT: i32 = 7;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            Light2dPlugin,
            CandlePlugin,
        ))
        .init_resource::<DungeonTileset>()
        .add_systems(Startup, (setup_camera, set_clear_color))
        .add_systems(
            Startup,
            (setup_dungeon_tileset, (spawn_tiles, spawn_chests)).chain(),
        )
        .add_systems(Startup, (candles.spawn(), spawn_cursor_light))
        .add_systems(Update, follow_cursor)
        .run();
}

#[derive(Resource, Default)]
struct DungeonTileset {
    layout: Handle<TextureAtlasLayout>,
    texture: Handle<Image>,
}

fn setup_camera(mut commands: Commands) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 0.25;
    commands.spawn((
        Camera2d,
        Projection::Orthographic(projection),
        Light2d {
            ambient_light: AmbientLight2d {
                brightness: 0.1,
                ..default()
            },
            z_sorting: true,
        },
    ));
}

#[derive(Component)]
struct CursorLight;

fn spawn_cursor_light(mut commands: Commands) {
    commands.spawn((
        PointLight2d {
            radius: 80.0,
            intensity: 1.0,
            falloff: 0.1,
            cast_shadows: true,
            color: Color::Srgba(YELLOW),
        },
        Transform::from_xyz(0.0, 0.0, ENTITY_INDEX),
        Visibility::Hidden,
        CursorLight,
    ));
}

fn follow_cursor(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    light: Single<(&mut Transform, &mut Visibility), With<CursorLight>>,
) {
    let (camera, camera_transform) = *camera;
    let (mut transform, mut visibility) = light.into_inner();
    let cursor = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok());
    match cursor {
        Some(world) => {
            transform.translation.x = world.x;
            transform.translation.y = world.y;
            *visibility = Visibility::Visible;
        }
        None => *visibility = Visibility::Hidden,
    }
}

fn spawn_chests(mut commands: Commands, tileset: Res<DungeonTileset>) {
    for (x, y) in [(2, 0), (-2, 1), (-2, -1)] {
        commands.spawn((
            SpriteMesh {
                image: tileset.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: CHEST,
                    layout: tileset.layout.clone(),
                }),
                ..default()
            },
            Transform::from_translation(tile_translation(x, y).extend(ENTITY_INDEX)),
            children![(
                LightOccluder2d {
                    shape: LightOccluder2dShape::Rectangle {
                        half_size: Vec2::new(4.5, 1.5),
                    },
                },
                Transform::from_xyz(0.0, -4.5, 0.0),
            )],
        ));
    }
}

fn set_clear_color(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::srgb_u8(37, 19, 26);
}

fn candles() -> impl SceneList {
    bsn_list! {
        @Candle Transform::from_xyz(0., 2., ENTITY_INDEX)
    }
}

fn spawn_tiles(mut commands: Commands, tileset: Res<DungeonTileset>) {
    let half_width = ROOM_WIDTH / 2;
    let half_height = ROOM_HEIGHT / 2;

    for y in -half_height..=half_height {
        for x in -half_width..=half_width {
            let column = (x + half_width) as usize;
            let index = match (x, y) {
                (x, y) if x == -half_width && y == half_height => LEFT_WALL_A,
                (x, y) if x == half_width && y == half_height => RIGHT_WALL_A,
                (x, y) if x == -half_width && y == -half_height => BOTTOM_LEFT_WALL,
                (x, y) if x == half_width && y == -half_height => BOTTOM_RIGHT_WALL,
                (_, y) if y == half_height => TOP_WALLS[column % TOP_WALLS.len()],
                (_, y) if y == -half_height => BOTTOM_WALLS[column % BOTTOM_WALLS.len()],
                (x, y) if x == -half_width && y == half_height - 1 => LEFT_WALL_B,
                (x, y) if x == half_width && y == half_height - 1 => RIGHT_WALL_B,
                (x, y) if x == -half_width && y == -half_height + 1 => LEFT_WALL_D,
                (x, y) if x == half_width && y == -half_height + 1 => RIGHT_WALL_D,
                (x, _) if x == -half_width => LEFT_WALL_C,
                (x, _) if x == half_width => RIGHT_WALL_C,
                (x, y) if x == -half_width + 1 && y == half_height - 1 => TOP_LEFT_FLOOR,
                (x, y) if x == half_width - 1 && y == half_height - 1 => TOP_RIGHT_FLOOR,
                (x, y) if x == -half_width + 1 && y == -half_height + 1 => BOTTOM_LEFT_FLOOR,
                (x, y) if x == half_width - 1 && y == -half_height + 1 => BOTTOM_RIGHT_FLOOR,
                (_, y) if y == half_height - 1 => TOP_FLOORS[column % TOP_FLOORS.len()],
                (_, y) if y == -half_height + 1 => BOTTOM_FLOORS[column % BOTTOM_FLOORS.len()],
                (x, _) if x == -half_width + 1 => LEFT_FLOOR,
                (x, _) if x == half_width - 1 => RIGHT_FLOOR,
                _ => FLOORS[column % FLOORS.len()],
            };
            spawn_from_atlas(
                &mut commands,
                tile_translation(x, y).extend(TILE_INDEX),
                index,
                tileset.layout.clone(),
                tileset.texture.clone(),
            );
        }
    }
}

fn tile_translation(x: i32, y: i32) -> Vec2 {
    Vec2::new(x as f32 * 16.0, y as f32 * 16.0)
}

fn spawn_from_atlas(
    commands: &mut Commands,
    translation: Vec3,
    sprite_index: usize,
    atlas_handle: Handle<TextureAtlasLayout>,
    texture: Handle<Image>,
) {
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                index: sprite_index,
                layout: atlas_handle,
            },
        ),
        Transform::from_translation(translation),
    ));
}

fn setup_dungeon_tileset(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut dungeon_tileset: ResMut<DungeonTileset>,
) {
    dungeon_tileset.texture = asset_server.load("dungeon_tiles.png");
    dungeon_tileset.layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        10,
        10,
        None,
        None,
    ));
}

const TOP_WALL_A: usize = 1;
const TOP_WALL_B: usize = 2;
const TOP_WALL_C: usize = 3;
const TOP_WALL_D: usize = 4;

const LEFT_WALL_A: usize = 0;
const LEFT_WALL_B: usize = 10;
const LEFT_WALL_C: usize = 20;
const LEFT_WALL_D: usize = 20;

const RIGHT_WALL_A: usize = 5;
const RIGHT_WALL_B: usize = 15;
const RIGHT_WALL_C: usize = 25;
const RIGHT_WALL_D: usize = 35;

const BOTTOM_LEFT_WALL: usize = 40;
const BOTTOM_RIGHT_WALL: usize = 45;

const BOTTOM_WALL_A: usize = 41;
const BOTTOM_WALL_B: usize = 42;
const BOTTOM_WALL_C: usize = 43;
const BOTTOM_WALL_D: usize = 44;

const TOP_LEFT_FLOOR: usize = 11;
const TOP_RIGHT_FLOOR: usize = 14;

const TOP_FLOOR_A: usize = 12;
const TOP_FLOOR_B: usize = 13;

const LEFT_FLOOR: usize = 21;
const RIGHT_FLOOR: usize = 24;

const BOTTOM_LEFT_FLOOR: usize = 31;
const BOTTOM_RIGHT_FLOOR: usize = 34;

const BOTTOM_FLOOR_A: usize = 32;
const BOTTOM_FLOOR_B: usize = 33;

const FLOOR_A: usize = 22;
const FLOOR_B: usize = 23;

const CHEST: usize = 80;

const TOP_WALLS: [usize; 5] = [TOP_WALL_A, TOP_WALL_B, TOP_WALL_C, TOP_WALL_A, TOP_WALL_D];
const BOTTOM_WALLS: [usize; 5] = [
    BOTTOM_WALL_A,
    BOTTOM_WALL_B,
    BOTTOM_WALL_C,
    BOTTOM_WALL_A,
    BOTTOM_WALL_D,
];
const TOP_FLOORS: [usize; 2] = [TOP_FLOOR_A, TOP_FLOOR_B];
const BOTTOM_FLOORS: [usize; 2] = [BOTTOM_FLOOR_A, BOTTOM_FLOOR_B];
const FLOORS: [usize; 2] = [FLOOR_A, FLOOR_B];
