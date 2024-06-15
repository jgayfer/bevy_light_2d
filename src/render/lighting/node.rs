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

use crate::render::extract::{ExtractedAmbientLight2d, ExtractedPointLight2d};

use super::{LightingPipeline, LightingPipelineId};

const LIGHTING_PASS: &str = "lighting_pass";
const LIGHTING_BIND_GROUP: &str = "lighting_bind_group";

#[derive(Default)]
pub struct LightingNode;

impl ViewNode for LightingNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static DynamicUniformIndex<ExtractedAmbientLight2d>,
        &'static ViewUniformOffset,
        &'static LightingPipelineId,
    );

    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        (view_target, ambient_index, view_offset, pipeline_id): bevy::ecs::query::QueryItem<
            'w,
            Self::ViewQuery,
        >,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let lighting_pipeline = world.resource::<LightingPipeline>();

        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(pipeline_id.0) else {
            return Ok(());
        };

        let Some(view_uniform_binding) = world.resource::<ViewUniforms>().uniforms.binding() else {
            return Ok(());
        };

        let Some(ambient_light_uniform) = world
            .resource::<ComponentUniforms<ExtractedAmbientLight2d>>()
            .uniforms()
            .binding()
        else {
            return Ok(());
        };

        let Some(point_light_binding) = world
            .resource::<GpuArrayBuffer<ExtractedPointLight2d>>()
            .binding()
        else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            LIGHTING_BIND_GROUP,
            &lighting_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &lighting_pipeline.sampler,
                view_uniform_binding,
                ambient_light_uniform,
                point_light_binding,
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

        let mut dynamic_offsets: SmallVec<[u32; 3]> =
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
            dynamic_offsets.push(0);
        }

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &dynamic_offsets);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
