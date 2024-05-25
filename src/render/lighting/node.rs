use bevy::prelude::*;
use bevy::render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy::render::render_graph::ViewNode;

use bevy::render::render_resource::{
    BindGroupEntries, Operations, PipelineCache, RenderPassColorAttachment, RenderPassDescriptor,
};
use bevy::render::view::{ViewTarget, ViewUniformOffset, ViewUniforms};

use crate::render::extract::ExtractedAmbientLight2d;
use crate::render::gpu::GpuPointLights;

use super::LightingPipeline;

const LIGHTING_PASS: &str = "lighting_pass";
const LIGHTING_BIND_GROUP: &str = "lighting_bind_group";

#[derive(Default)]
pub struct LightingNode;

impl ViewNode for LightingNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static DynamicUniformIndex<ExtractedAmbientLight2d>,
        &'static ViewUniformOffset,
    );

    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        (view_target, ambient_index, view_offset): bevy::ecs::query::QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let lighting_pipeline = world.resource::<LightingPipeline>();

        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(lighting_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let Some(view_uniform_binding) = world.resource::<ViewUniforms>().uniforms.binding() else {
            return Ok(());
        };

        let Some(point_light_buffer) = world.resource::<GpuPointLights>().buffer.binding() else {
            return Ok(());
        };

        let Some(ambient_light_uniform) = world
            .resource::<ComponentUniforms<ExtractedAmbientLight2d>>()
            .uniforms()
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
                point_light_buffer,
                ambient_light_uniform,
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

        // Setup fullscreen triangle.
        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[view_offset.offset, ambient_index.index()]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
