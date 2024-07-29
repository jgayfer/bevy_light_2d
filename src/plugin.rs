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
        view::{check_visibility, prepare_view_targets, VisibilitySystems},
        Render, RenderApp, RenderSet,
    },
};

use crate::{
    light::{AmbientLight2d, PointLight2d},
    render::{
        extract::{
            extract_ambient_lights, extract_light_occluders, extract_point_lights,
            ExtractedAmbientLight2d, ExtractedLightOccluder2d, ExtractedPointLight2d,
        },
        lighting::{
            prepare_lighting_auxiliary_textures, prepare_lighting_pipelines, LightingNode,
            LightingPass, LightingPipeline, SdfPipeline, LIGHTING_SHADER, SDF_SHADER, TYPES_SHADER,
        },
    },
};

/// A plugin that provides 2d lighting for an app.
pub struct Light2dPlugin;

impl Plugin for Light2dPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            TYPES_SHADER,
            "render/lighting/types.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SDF_SHADER,
            "render/lighting/sdf.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            LIGHTING_SHADER,
            "render/lighting/lighting.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins((
            UniformComponentPlugin::<ExtractedAmbientLight2d>::default(),
            GpuComponentArrayBufferPlugin::<ExtractedPointLight2d>::default(),
            GpuComponentArrayBufferPlugin::<ExtractedLightOccluder2d>::default(),
        ))
        .register_type::<AmbientLight2d>()
        .register_type::<PointLight2d>()
        .add_systems(
            PostUpdate,
            check_visibility::<With<PointLight2d>>.in_set(VisibilitySystems::CheckVisibility),
        );

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<SpecializedRenderPipelines<LightingPipeline>>()
            .add_systems(
                ExtractSchedule,
                (
                    extract_point_lights,
                    extract_light_occluders,
                    extract_ambient_lights,
                ),
            )
            .add_systems(
                Render,
                (
                    prepare_lighting_pipelines.in_set(RenderSet::Prepare),
                    prepare_lighting_auxiliary_textures
                        .after(prepare_view_targets)
                        .in_set(RenderSet::ManageViews),
                ),
            )
            .add_render_graph_node::<ViewNodeRunner<LightingNode>>(Core2d, LightingPass)
            .add_render_graph_edge(Core2d, Node2d::EndMainPass, LightingPass);
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<LightingPipeline>()
            .init_resource::<SdfPipeline>();
    }
}
