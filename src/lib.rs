use bevy::app::Plugin;
use bevy::core_pipeline::core_2d::graph::{Core2d, Node2d};
use bevy::prelude::*;
use bevy::render::extract_component::UniformComponentPlugin;
use bevy::render::render_graph::{RenderGraphApp, ViewNodeRunner};
use bevy::render::render_resource::{ShaderType, StorageBuffer};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::{Render, RenderApp, RenderSet};
use render::extract::{
    extract_ambient_lights, extract_point_lights, ExtractedAmbientLight2d, ExtractedPointLight2d,
};
use render::lighting::{LightingNode, LightingPass, LightingPipeline};

mod component;
mod render;

pub use component::{AmbientLight2d, PointLight2d, PointLight2dBundle};

pub struct Light2dPlugin;

impl Plugin for Light2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UniformComponentPlugin::<ExtractedAmbientLight2d>::default());

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(
            ExtractSchedule,
            (extract_point_lights, extract_ambient_lights),
        );

        render_app.add_systems(Render, (prepare_lights).in_set(RenderSet::Prepare));

        render_app.add_render_graph_node::<ViewNodeRunner<LightingNode>>(Core2d, LightingPass);

        render_app.add_render_graph_edge(Core2d, Node2d::MainPass, LightingPass);
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<LightingPipeline>()
            .init_resource::<LightingPassAssets>();
    }
}

fn prepare_lights(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    point_light_query: Query<&ExtractedPointLight2d>,
    mut lighting_pass_assets: ResMut<LightingPassAssets>,
) {
    let point_light_buffer = lighting_pass_assets.point_lights.get_mut();

    // Resources are global state, so we need to clear the data from the previous frame.
    point_light_buffer.clear();

    for point_light in &point_light_query {
        point_light_buffer.push(GpuPointLight2d {
            center: point_light.transform.translation().xy(),
            radius: point_light.radius,
            color: point_light.color,
            energy: point_light.energy,
        });
    }

    lighting_pass_assets
        .point_lights
        .write_buffer(&render_device, &render_queue);
}

#[derive(Default, Resource)]
pub struct LightingPassAssets {
    pub point_lights: StorageBuffer<Vec<GpuPointLight2d>>,
}

#[derive(Default, Clone, ShaderType)]
pub struct GpuPointLight2d {
    pub center: Vec2,
    pub radius: f32,
    pub color: Vec3,
    pub energy: f32,
}
