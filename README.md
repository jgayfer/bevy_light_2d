# `bevy_light_2d`

[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jgayfer/bevy_light_2d/blob/master/LICENSE)
![build](https://github.com/jgayfer/bevy_light_2d/actions/workflows/build.yml/badge.svg)

A general purpose 2d lighting plugin for [`bevy`](https://bevyengine.org/).
Designed to be simple to use, yet expressive enough to fit a variety of needs.

## Features

- Component driven design
- Configurable point lights
- Camera specific ambient light

## Usage

In the [`basic`](./examples/basic.rs) example, all we need is the plugin, a camera, and a light source.

```toml
# Cargo.toml
[dependencies]
bevy = "0.13"
bevy_light_2d = "0.1"
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
            color: Color::YELLOW,
            radius: 50.,
            ..default()
        },
        ..default()
    });
}
```

## Bevy compatibility

| bevy | bevy_light_2d |
|------|---------------|
| 0.13 | 0.1           |
