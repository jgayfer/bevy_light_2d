//! A module which contains the main [`Light2dPlugin`].

use bevy::{
    asset::load_internal_asset,
    core_pipeline::core_2d::graph::{Core2d, Node2d},
    prelude::*,
    render::{
        extract_component::UniformComponentPlugin,
        render_graph::{RenderGraphApp, ViewNodeRunner},
        Render, RenderApp, RenderSet,
    },
};

use crate::{
    component::{AmbientLight2d, PointLight2d},
    render::{
        extract::{extract_ambient_lights, extract_point_lights, ExtractedAmbientLight2d},
        gpu::{prepare_point_lights, GpuPointLights},
        lighting::{LightingNode, LightingPass, LightingPipeline, LIGHTING_SHADER},
    },
};

/// A plugin that provides 2d lighting for an app.
pub struct Light2dPlugin;

impl Plugin for Light2dPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            LIGHTING_SHADER,
            "render/lighting/lighting.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(UniformComponentPlugin::<ExtractedAmbientLight2d>::default())
            .register_type::<AmbientLight2d>()
            .register_type::<PointLight2d>();

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
            .init_resource::<GpuPointLights>();
    }
}
