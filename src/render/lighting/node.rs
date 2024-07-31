use bevy::ecs::system::lifetimeless::Read;
use bevy::prelude::*;
use bevy::render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy::render::render_graph::ViewNode;

use bevy::render::render_resource::{
    BindGroupEntries, GpuArrayBuffer, Operations, PipelineCache, RenderPassColorAttachment,
    RenderPassDescriptor,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::{ViewTarget, ViewUniformOffset, ViewUniforms};
use smallvec::{smallvec, SmallVec};

use crate::render::extract::{
    ExtractedAmbientLight2d, ExtractedLightOccluder2d, ExtractedPointLight2d,
};

use super::{
    LightMapPipeline, Lighting2dAuxiliaryTextures, LightingPipeline, LightingPipelineId,
    SdfPipeline,
};

const SDF_PASS: &str = "sdf_pass";
const SDF_BIND_GROUP: &str = "sdf_bind_group";
const LIGHT_MAP_PASS: &str = "light_map_pass";
const LIGHT_MAP_BIND_GROUP: &str = "light_map_bind_group";
const LIGHTING_PASS: &str = "lighting_pass";
const LIGHTING_BIND_GROUP: &str = "lighting_bind_group";

#[derive(Default)]
pub struct LightingNode;

impl ViewNode for LightingNode {
    type ViewQuery = (
        Read<ViewTarget>,
        Read<DynamicUniformIndex<ExtractedAmbientLight2d>>,
        Read<ViewUniformOffset>,
        Read<LightingPipelineId>,
        Read<Lighting2dAuxiliaryTextures>,
    );

    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        (view_target, ambient_index, view_offset, pipeline_id, aux_textures): bevy::ecs::query::QueryItem<
            'w,
            Self::ViewQuery,
        >,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let sdf_pipeline_resource = world.resource::<SdfPipeline>();
        let light_map_pipeline_resource = world.resource::<LightMapPipeline>();
        let pipeline = world.resource::<LightingPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let (
            Some(sdf_pipeline),
            Some(lighting_pipeline),
            Some(light_map_pipeline),
            Some(view_uniform_binding),
            Some(ambient_light_uniform),
            Some(point_light_binding),
            Some(light_occluders_binding),
        ) = (
            pipeline_cache.get_render_pipeline(sdf_pipeline_resource.pipeline_id),
            pipeline_cache.get_render_pipeline(pipeline_id.0),
            pipeline_cache.get_render_pipeline(light_map_pipeline_resource.pipeline_id),
            world.resource::<ViewUniforms>().uniforms.binding(),
            world
                .resource::<ComponentUniforms<ExtractedAmbientLight2d>>()
                .uniforms()
                .binding(),
            world
                .resource::<GpuArrayBuffer<ExtractedPointLight2d>>()
                .binding(),
            world
                .resource::<GpuArrayBuffer<ExtractedLightOccluder2d>>()
                .binding(),
        )
        else {
            return Ok(());
        };

        // SDF
        let bind_group = render_context.render_device().create_bind_group(
            SDF_BIND_GROUP,
            &sdf_pipeline_resource.layout,
            &BindGroupEntries::sequential((view_uniform_binding.clone(), light_occluders_binding)),
        );

        let mut sdf_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some(SDF_PASS),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &aux_textures.sdf.default_view,
                resolve_target: None,
                ops: Operations::default(),
            })],
            ..default()
        });

        let mut dynamic_offsets: SmallVec<[u32; 3]> = smallvec![view_offset.offset];

        // Storage buffers aren't available in WebGL2. We fall back to a
        // dynamic uniform buffer, and therefore need to provide the offset.
        // We're providing a value of 0 here as we're limiting the number of
        // point lights to only those that can reasonably fit in a single binding.
        if world
            .resource::<RenderDevice>()
            .limits()
            .max_storage_buffers_per_shader_stage
            == 0
        {
            dynamic_offsets.push(0);
        }

        sdf_pass.set_render_pipeline(sdf_pipeline);
        sdf_pass.set_bind_group(0, &bind_group, &dynamic_offsets);
        sdf_pass.draw(0..3, 0..1);

        drop(sdf_pass);

        // Light map
        let light_map_bind_group = render_context.render_device().create_bind_group(
            LIGHT_MAP_BIND_GROUP,
            &light_map_pipeline_resource.layout,
            &BindGroupEntries::sequential((
                view_uniform_binding.clone(),
                ambient_light_uniform.clone(),
                point_light_binding.clone(),
                &aux_textures.sdf.default_view,
                &light_map_pipeline_resource.sdf_sampler,
            )),
        );

        let mut light_map_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some(LIGHT_MAP_PASS),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &aux_textures.light_map.default_view,
                resolve_target: None,
                ops: Operations::default(),
            })],
            ..default()
        });

        let mut light_map_offsets: SmallVec<[u32; 3]> =
            smallvec![view_offset.offset, ambient_index.index()];

        // Storage buffers aren't available in WebGL2. We fall back to a
        // dynamic uniform buffer, and therefore need to provide the offset.
        // We're providing a value of 0 here as we're limiting the number of
        // point lights to only those that can reasonably fit in a single binding.
        if world
            .resource::<RenderDevice>()
            .limits()
            .max_storage_buffers_per_shader_stage
            == 0
        {
            light_map_offsets.push(0);
        }

        light_map_pass.set_render_pipeline(light_map_pipeline);
        light_map_pass.set_bind_group(0, &light_map_bind_group, &light_map_offsets);
        light_map_pass.draw(0..3, 0..1);

        drop(light_map_pass);

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            LIGHTING_BIND_GROUP,
            &pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &aux_textures.light_map.default_view,
                &pipeline.sampler,
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some(LIGHTING_PASS),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(lighting_pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
