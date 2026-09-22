use bevy::{
    asset::asset_value, color::palettes::css::YELLOW, image::TextureAtlasTemplate, prelude::*,
};
use bevy_light_2d::prelude::*;

pub struct CandlePlugin;

impl Plugin for CandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_candles);
    }
}

#[derive(SceneComponent, Clone)]
pub struct Candle {
    frame_timer: Timer,
}

impl Default for Candle {
    fn default() -> Self {
        Self {
            frame_timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        }
    }
}

impl Candle {
    fn scene() -> impl Scene {
        bsn! {
            Sprite {
                image: "candle.png",
                texture_atlas: {
                    TextureAtlasTemplate {
                        layout: asset_value(TextureAtlasLayout::from_grid(
                            UVec2::new(16, 16),
                            4,
                            1,
                            None,
                            None,
                        )),
                        index: 0,
                    }
                },
            }
            Children [
                PointLight2d {
                    radius: 48.0,
                    color: Color::Srgba(YELLOW),
                    intensity: 2.0,
                    falloff: 4.0,
                }
                Transform::from_xyz(0.0, 4.0, 0.0)
            ]
        }
    }
}

fn animate_candles(time: Res<Time>, mut query: Query<(&mut Candle, &mut Sprite)>) {
    for (mut candle, mut sprite) in &mut query {
        candle.frame_timer.tick(time.delta());
        if candle.frame_timer.just_finished()
            && let Some(ref mut texture_atlas) = sprite.texture_atlas
        {
            texture_atlas.index = (texture_atlas.index + 1) % 4;
        }
    }
}
