use bevy::{color::palettes::css::YELLOW, prelude::*};
use bevy_light_2d::prelude::*;

const TILE_INDEX: f32 = 0.0;
const ENTITY_INDEX: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            Light2dPlugin,
        ))
        .init_resource::<DungeonTileset>()
        .init_resource::<CandleSpritesheet>()
        .add_systems(Startup, (setup_camera, set_clear_color))
        .add_systems(Startup, (setup_dungeon_tileset, spawn_tiles).chain())
        .add_systems(Startup, (setup_candle_spritesheet, spawn_candles).chain())
        .add_systems(Update, animate_candles)
        .run();
}

#[derive(Resource, Default)]
struct DungeonTileset {
    layout: Handle<TextureAtlasLayout>,
    texture: Handle<Image>,
}

#[derive(Resource, Default)]
struct CandleSpritesheet {
    layout: Handle<TextureAtlasLayout>,
    texture: Handle<Image>,
}

#[derive(Component)]
struct Candle;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.25;
    commands.spawn((
        camera,
        AmbientLight2d {
            brightness: 0.1,
            ..default()
        },
    ));
}

fn set_clear_color(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::srgb_u8(37, 19, 26);
}

fn animate_candles(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas), With<Candle>>,
) {
    for (mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = (atlas.index + 1) % 4;
        }
    }
}

fn spawn_candles(mut commands: Commands, spritesheet: Res<CandleSpritesheet>) {
    let light = commands
        .spawn((PointLight2dBundle {
            transform: Transform::from_xyz(0.0, 4.0, ENTITY_INDEX),
            point_light: PointLight2d {
                radius: 48.0,
                color: Color::Srgba(YELLOW),
                intensity: 25.0,
                falloff: 4.0,
            },
            ..default()
        },))
        .id();

    commands
        .spawn((
            Candle,
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            SpriteBundle {
                transform: Transform::from_xyz(0., 2., ENTITY_INDEX),
                texture: spritesheet.texture.clone(),
                ..default()
            },
            TextureAtlas {
                layout: spritesheet.layout.clone(),
                ..default()
            },
        ))
        .add_child(light);
}

fn spawn_tiles(mut commands: Commands, tileset: Res<DungeonTileset>) {
    let mut spawn_wall_tile = |position: (i32, i32), index: usize| {
        spawn_from_atlas(
            &mut commands,
            tile_translation(position.0, position.1).extend(TILE_INDEX),
            index,
            tileset.layout.clone(),
            tileset.texture.clone(),
        );
    };

    // First row
    spawn_wall_tile((-3, 2), LEFT_WALL_A);
    spawn_wall_tile((-2, 2), TOP_WALL_A);
    spawn_wall_tile((-1, 2), TOP_WALL_B);
    spawn_wall_tile((0, 2), TOP_WALL_C);
    spawn_wall_tile((1, 2), TOP_WALL_A);
    spawn_wall_tile((2, 2), TOP_WALL_D);
    spawn_wall_tile((3, 2), RIGHT_WALL_A);

    // Second row
    spawn_wall_tile((-3, 1), LEFT_WALL_B);
    spawn_wall_tile((-2, 1), TOP_LEFT_FLOOR);
    spawn_wall_tile((-1, 1), TOP_FLOOR_A);
    spawn_wall_tile((0, 1), TOP_FLOOR_B);
    spawn_wall_tile((1, 1), TOP_FLOOR_A);
    spawn_wall_tile((2, 1), TOP_RIGHT_FLOOR);
    spawn_wall_tile((3, 1), RIGHT_WALL_B);

    // Third row
    spawn_wall_tile((-3, 0), LEFT_WALL_C);
    spawn_wall_tile((-2, 0), LEFT_FLOOR);
    spawn_wall_tile((-1, 0), FLOOR_A);
    spawn_wall_tile((0, 0), FLOOR_B);
    spawn_wall_tile((1, 0), FLOOR_A);
    spawn_wall_tile((2, 0), RIGHT_FLOOR);
    spawn_wall_tile((3, 0), RIGHT_WALL_C);

    // Fourth row
    spawn_wall_tile((-3, -1), LEFT_WALL_D);
    spawn_wall_tile((-2, -1), BOTTOM_LEFT_FLOOR);
    spawn_wall_tile((-1, -1), BOTTOM_FLOOR_A);
    spawn_wall_tile((0, -1), BOTTOM_FLOOR_B);
    spawn_wall_tile((1, -1), BOTTOM_FLOOR_B);
    spawn_wall_tile((2, -1), BOTTOM_RIGHT_FLOOR);
    spawn_wall_tile((3, -1), RIGHT_WALL_D);

    // Bottom row
    spawn_wall_tile((-3, -2), BOTTOM_LEFT_WALL);
    spawn_wall_tile((-2, -2), BOTTOM_WALL_A);
    spawn_wall_tile((-1, -2), BOTTOM_WALL_B);
    spawn_wall_tile((0, -2), BOTTOM_WALL_C);
    spawn_wall_tile((1, -2), BOTTOM_WALL_A);
    spawn_wall_tile((2, -2), BOTTOM_WALL_D);
    spawn_wall_tile((3, -2), BOTTOM_RIGHT_WALL);
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
        SpriteBundle {
            transform: Transform {
                translation,
                ..default()
            },
            texture,
            ..default()
        },
        TextureAtlas {
            index: sprite_index,
            layout: atlas_handle,
        },
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

fn setup_candle_spritesheet(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut candle_spritesheet: ResMut<CandleSpritesheet>,
) {
    candle_spritesheet.texture = asset_server.load("candle.png");
    candle_spritesheet.layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(16, 16),
        4,
        1,
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
