# `bevy_light_2d`

[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jgayfer/bevy_light_2d/blob/master/LICENSE)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/jgayfer/bevy_light_2d/build.yml)

A general purpose 2d lighting plugin for [`bevy`](https://bevyengine.org/).
Designed to be simple to use, yet expressive enough to fit a variety of needs.

## Features

- Component driven design
- Configurable point lights
- Camera specific ambient light
- Single camera rendering

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
            radius: 100.0,
            intensity: 3.0,
            ..default()
        },
        ..default()
    });
}
```

## Motiviation

When I first started experimenting with Bevy, the lack of a first party 2d
lighting implementation left me wanting. While there were some rather impressive
experimental 2d lighting crates out there, there wasn't much in the way
of drop in options available.

My goal with this crate is to fill that void, prioritizing ease of use and
general application over depth of features.

## Future goals

- Light occluders + shadows
- Sprite lights

## Bevy compatibility

| bevy | bevy_light_2d |
|------|---------------|
| 0.13 | 0.1           |
