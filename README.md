# `bevy_light_2d`

[![Discord](https://img.shields.io/discord/805147867924267018?logo=discord&color=7289DA)](https://discord.gg/yZmJgXnqfv)
[![Crates.io](https://img.shields.io/crates/v/bevy_light_2d)](https://crates.io/crates/bevy_light_2d)
[![docs](https://docs.rs/bevy_light_2d/badge.svg)](https://docs.rs/bevy_light_2d/)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jgayfer/bevy_light_2d/blob/master/LICENSE)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/jgayfer/bevy_light_2d/build.yml)
[![Crates.io](https://img.shields.io/crates/d/bevy_light_2d)](https://crates.io/crates/bevy_light_2d)

General purpose 2D lighting for the [`bevy`](https://bevyengine.org/) game engine.
Designed to be simple to use, yet expressive enough to fit a variety of needs.

<img src="https://github.com/jgayfer/bevy_light_2d/blob/main/static/dungeon.gif?raw=true" width="400">

## Features

- Component driven design
- Configurable [point lights](https://docs.rs/bevy_light_2d/0.2.2/bevy_light_2d/light/struct.PointLight2d.html)
- Camera specific [ambient light](https://docs.rs/bevy_light_2d/0.2.2/bevy_light_2d/light/struct.AmbientLight2d.html)
- Single camera rendering

## Usage

In the [`basic`](https://github.com/jgayfer/bevy_light_2d/blob/main/examples/basic.rs) example, all we need is the plugin, a camera, and a light source.

```toml
# Cargo.toml
[dependencies]
bevy = "0.14"
bevy_light_2d = "0.2"
```

```rust
use bevy::prelude::*;
use bevy_light_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(PointLight2dBundle {
        point_light: PointLight2d {
            radius: 100.0,
            intensity: 3.0,
            ..default()
        },
        ..default()
    });
}
```

To see an in depth example, use `cargo run --example dungeon`.

## Web support

WebGL2 and WebGPU are both supported.

## Motiviation

When I first started experimenting with Bevy, the lack of a first party 2D
lighting implementation left me wanting. While there were some rather impressive
experimental 2D lighting crates out there, there wasn't much in the way
of drop in options available.

My goal with this crate is to fill that void, prioritizing ease of use and
general application over depth of features.

## Future goals

- Light occluders + shadows
- Sprite lights

## Bevy compatibility

| bevy | bevy_light_2d |
|------|---------------|
| 0.14 | 0.2           |
| 0.13 | 0.1           |

## Acknowledgements

I'd like to thank the authors of the below crates; they were a significant source of inspiration.

- [`bevy-magic-light-2d`](https://github.com/zaycev/bevy-magic-light-2d)
- [`bevy_2d_screen_space_lightmaps`](https://github.com/goto64/bevy_2d_screen_space_lightmaps)
- [`bevy_incandescent`](https://github.com/443eb9/bevy_incandescent)

## Asset credits

- [Pixel Dungeon](https://pixel-poem.itch.io/dungeon-assetpuck) by [Pixel Poem](https://pixel-poem.itch.io/)
