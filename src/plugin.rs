//! A module which contains the main [`Light2dPlugin`].

use bevy::{
    asset::load_internal_asset,
    core_pipeline::{Core2d, Core2dSystems},
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems, extract_component::UniformComponentPlugin,
        gpu_component_array_buffer::GpuComponentArrayBufferPlugin,
        render_resource::SpecializedRenderPipelines, view::prepare_view_targets,
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
            LIGHT_MAP_SHADER, LightMapPipeline, PointLightMetaBuffer, SpotLightMetaBuffer,
            light_map_pass, prepare_light_map_texture, prepare_point_light_count,
            prepare_spot_light_count,
        },
        lighting::{LIGHTING_SHADER, LightingPipeline, lighting_pass, prepare_lighting_pipelines},
        sdf::{
            OccluderMetaBuffer, SDF_SHADER, SdfPipeline, prepare_occluder_meta,
            prepare_sdf_texture, sdf_pass,
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
                        .in_set(RenderSystems::PrepareViews),
                    prepare_light_map_texture
                        .after(prepare_view_targets)
                        .in_set(RenderSystems::PrepareViews),
                ),
            )
            .add_systems(
                Core2d,
                (sdf_pass, light_map_pass, lighting_pass)
                    .chain()
                    .after(Core2dSystems::MainPass)
                    .before(Core2dSystems::EarlyPostProcess),
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
