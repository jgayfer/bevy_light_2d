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
    occluder::LightOccluder2d,
    render::{
        extract::{
            extract_ambient_lights, extract_light_occluders, extract_point_lights,
            ExtractedAmbientLight2d, ExtractedLightOccluder2d, ExtractedPointLight2d,
        },
        light_map::{
            prepare_light_map_texture, LightMapNode, LightMapPass, LightMapPipeline,
            LIGHT_MAP_SHADER,
        },
        lighting::{
            prepare_lighting_pipelines, LightingNode, LightingPass, LightingPipeline,
            LIGHTING_SHADER,
        },
        sdf::{prepare_sdf_texture, SdfNode, SdfPass, SdfPipeline, SDF_SHADER},
        TYPES_SHADER, VIEW_TRANSFORMATIONS_SHADER,
    },
};

/// A plugin that provides 2d lighting for an app.
pub struct Light2dPlugin;

impl Plugin for Light2dPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, TYPES_SHADER, "render/types.wgsl", Shader::from_wgsl);
        load_internal_asset!(
            app,
            VIEW_TRANSFORMATIONS_SHADER,
            "render/view_transformations.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(app, SDF_SHADER, "render/sdf/sdf.wgsl", Shader::from_wgsl);
        load_internal_asset!(
            app,
            LIGHTING_SHADER,
            "render/lighting/lighting.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            LIGHT_MAP_SHADER,
            "render/light_map/light_map.wgsl",
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
            (
                check_visibility::<With<PointLight2d>>,
                check_visibility::<With<LightOccluder2d>>,
            )
                .in_set(VisibilitySystems::CheckVisibility),
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
                    prepare_sdf_texture
                        .after(prepare_view_targets)
                        .in_set(RenderSet::ManageViews),
                    prepare_light_map_texture
                        .after(prepare_view_targets)
                        .in_set(RenderSet::ManageViews),
                ),
            )
            .add_render_graph_node::<ViewNodeRunner<LightingNode>>(Core2d, LightingPass)
            .add_render_graph_node::<ViewNodeRunner<SdfNode>>(Core2d, SdfPass)
            .add_render_graph_node::<ViewNodeRunner<LightMapNode>>(Core2d, LightMapPass)
            .add_render_graph_edges(
                Core2d,
                (Node2d::EndMainPass, SdfPass, LightMapPass, LightingPass),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<LightingPipeline>()
            .init_resource::<SdfPipeline>()
            .init_resource::<LightMapPipeline>();
    }
}
