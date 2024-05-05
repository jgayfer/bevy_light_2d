use bevy::prelude::*;
use bevy_light_2d::Light2dPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Light2dPlugin))
        .run()
}
