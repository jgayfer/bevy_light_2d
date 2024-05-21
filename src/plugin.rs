use bevy::{
    core_pipeline::core_2d::graph::{Core2d, Node2d},
    prelude::*,
    render::{
        extract_component::UniformComponentPlugin,
        render_graph::{RenderGraphApp, ViewNodeRunner},
        Render, RenderApp, RenderSet,
    },
};

use crate::render::{
    extract::{extract_ambient_lights, extract_point_lights, ExtractedAmbientLight2d},
    gpu::{prepare_point_lights, LightingPassAssets},
    lighting::{LightingNode, LightingPass, LightingPipeline},
};

pub struct Light2dPlugin;

impl Plugin for Light2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UniformComponentPlugin::<ExtractedAmbientLight2d>::default());

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_systems(
                ExtractSchedule,
                (extract_point_lights, extract_ambient_lights),
            )
            .add_systems(Render, (prepare_point_lights).in_set(RenderSet::Prepare))
            .add_render_graph_node::<ViewNodeRunner<LightingNode>>(Core2d, LightingPass)
            .add_render_graph_edge(Core2d, Node2d::MainPass, LightingPass);
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
