//! A module which contains the main [`Light2dPlugin`].

use bevy::{
    asset::load_internal_asset,
    core_pipeline::core_2d::graph::{Core2d, Node2d},
    prelude::*,
    render::{
        extract_component::UniformComponentPlugin,
        gpu_component_array_buffer::GpuComponentArrayBufferPlugin,
        render_graph::{RenderGraphApp, ViewNodeRunner},
        render_resource::SpecializedRenderPipelines,
        Render, RenderApp, RenderSet,
    },
};

use crate::{
    light::{AmbientLight2d, PointLight2d},
    render::{
        extract::{
            extract_ambient_lights, extract_point_lights, ExtractedAmbientLight2d,
            ExtractedPointLight2d,
        },
        lighting::{
            prepare_lighting_pipelines, LightingNode, LightingPass, LightingPipeline,
            LIGHTING_SHADER,
        },
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

        app.add_plugins((
            UniformComponentPlugin::<ExtractedAmbientLight2d>::default(),
            GpuComponentArrayBufferPlugin::<ExtractedPointLight2d>::default(),
        ))
        .register_type::<AmbientLight2d>()
        .register_type::<PointLight2d>();

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<SpecializedRenderPipelines<LightingPipeline>>()
            .add_systems(
                ExtractSchedule,
                (extract_point_lights, extract_ambient_lights),
            )
            .add_systems(
                Render,
                prepare_lighting_pipelines.in_set(RenderSet::Prepare),
            )
            .add_render_graph_node::<ViewNodeRunner<LightingNode>>(Core2d, LightingPass)
            .add_render_graph_edge(Core2d, Node2d::MainPass, LightingPass);
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<LightingPipeline>();
    }
}
