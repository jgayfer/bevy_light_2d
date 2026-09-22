//! A module which contains the main [`Light2dPlugin`].

use bevy::{
    asset::embedded_asset,
    camera::visibility::VisibilitySystems,
    core_pipeline::{Core2d, Core2dSystems},
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems, extract_component::UniformComponentPlugin,
        render_resource::SpecializedRenderPipelines, view::prepare_view_targets,
    },
    shader::load_shader_library,
};

use crate::{
    light::{AmbientLight2d, PointLight2d, SpotLight2d},
    occluder::LightOccluder2dVisibility,
    render::{
        empty_buffer::{EmptyBuffer, prepare_empty_buffer},
        extract::{
            ExtractedAmbientLight2d, ExtractedLightOccluder2d, ExtractedPointLight2d,
            ExtractedSpotLight2d, extract_ambient_lights, extract_light_occluders,
            extract_point_lights, extract_spot_lights,
        },
        gpu_array_buffer::GpuArrayBufferPlugin,
        light_map::{
            LightMapPipeline, PointLightMetaBuffer, SpotLightMetaBuffer, light_map_pass,
            prepare_light_map_texture, prepare_point_light_count, prepare_spot_light_count,
        },
        lighting::{LightingPipeline, lighting_pass, prepare_lighting_pipelines},
        sdf::{
            OccluderMetaBuffer, SdfPipeline, prepare_occluder_meta, prepare_sdf_texture, sdf_pass,
        },
    },
    visibility::{calculate_light_bounds, check_occluder_visibility},
};

/// A plugin that provides 2d lighting for an app.
pub struct Light2dPlugin;

impl Plugin for Light2dPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "render/types.wesl");
        load_shader_library!(app, "render/view_transformations.wesl");

        embedded_asset!(app, "render/sdf/sdf.wesl");
        embedded_asset!(app, "render/lighting/lighting.wesl");
        embedded_asset!(app, "render/light_map/light_map.wesl");

        app.add_plugins((
            UniformComponentPlugin::<ExtractedAmbientLight2d>::default(),
            GpuArrayBufferPlugin::<ExtractedPointLight2d>::default(),
            GpuArrayBufferPlugin::<ExtractedLightOccluder2d>::default(),
            GpuArrayBufferPlugin::<ExtractedSpotLight2d>::default(),
        ))
        .register_type::<AmbientLight2d>()
        .register_type::<PointLight2d>()
        .register_type::<SpotLight2d>()
        .register_type::<LightOccluder2dVisibility>();

        app.add_systems(
            PostUpdate,
            calculate_light_bounds.in_set(VisibilitySystems::CalculateBounds),
        );

        app.add_systems(
            PostUpdate,
            check_occluder_visibility.after(VisibilitySystems::CheckVisibility),
        );

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
                    extract_spot_lights,
                    extract_light_occluders,
                    extract_ambient_lights,
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
