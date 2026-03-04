//! A module which contains the main [`Light2dPlugin`].

use bevy::{
    asset::load_internal_asset,
    core_pipeline::core_2d::graph::{Core2d, Node2d},
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems, extract_component::UniformComponentPlugin,
        gpu_component_array_buffer::GpuComponentArrayBufferPlugin, render_graph::RenderGraphExt,
        render_graph::ViewNodeRunner, render_resource::SpecializedRenderPipelines,
        view::prepare_view_targets,
    },
};

use crate::{
    light::{AmbientLight2d, PointLight2d, SpotLight2d},
    render::{
        TYPES_SHADER, VIEW_TRANSFORMATIONS_SHADER,
        empty_buffer::{EmptyBuffer, prepare_empty_buffer},
        extract::{
            ExtractedAmbientLight2d, ExtractedLightOccluder2d, ExtractedPointLight2d,
            ExtractedSpotLight2d, extract_ambient_lights, extract_light_occluders,
            extract_point_lights, extract_spot_lights,
        },
        light_map::{
            LIGHT_MAP_SHADER, LightMapNode, LightMapPass, LightMapPipeline, PointLightMetaBuffer,
            SpotLightMetaBuffer, prepare_light_map_texture, prepare_point_light_count,
            prepare_spot_light_count,
        },
        lighting::{
            LIGHTING_SHADER, LightingNode, LightingPass, LightingPipeline,
            prepare_lighting_pipelines,
        },
        sdf::{
            OccluderMetaBuffer, SDF_SHADER, SdfNode, SdfPass, SdfPipeline, prepare_occluder_meta,
            prepare_sdf_texture,
        },
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
            GpuComponentArrayBufferPlugin::<ExtractedSpotLight2d>::default(),
        ))
        .register_type::<AmbientLight2d>()
        .register_type::<PointLight2d>()
        .register_type::<SpotLight2d>();

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<SpecializedRenderPipelines<LightingPipeline>>()
            .init_resource::<PointLightMetaBuffer>()
            .init_resource::<SpotLightMetaBuffer>()
            .init_resource::<OccluderMetaBuffer>()
            .init_resource::<EmptyBuffer>()
            .add_systems(
                ExtractSchedule,
                (
                    extract_point_lights,
                    extract_light_occluders,
                    extract_ambient_lights,
                    extract_spot_lights,
                ),
            )
            .add_systems(
                Render,
                (
                    prepare_lighting_pipelines.in_set(RenderSystems::Prepare),
                    prepare_point_light_count.in_set(RenderSystems::Prepare),
                    prepare_spot_light_count.in_set(RenderSystems::Prepare),
                    prepare_occluder_meta.in_set(RenderSystems::Prepare),
                    prepare_empty_buffer.in_set(RenderSystems::Prepare),
                    prepare_sdf_texture
                        .after(prepare_view_targets)
                        .in_set(RenderSystems::ManageViews),
                    prepare_light_map_texture
                        .after(prepare_view_targets)
                        .in_set(RenderSystems::ManageViews),
                ),
            )
            .add_render_graph_node::<ViewNodeRunner<LightingNode>>(Core2d, LightingPass)
            .add_render_graph_node::<ViewNodeRunner<SdfNode>>(Core2d, SdfPass)
            .add_render_graph_node::<ViewNodeRunner<LightMapNode>>(Core2d, LightMapPass)
            .add_render_graph_edges(
                Core2d,
                (
                    Node2d::EndMainPass,
                    SdfPass,
                    LightMapPass,
                    LightingPass,
                    Node2d::StartMainPassPostProcessing,
                ),
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
