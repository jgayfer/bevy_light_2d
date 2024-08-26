use bevy::ecs::system::lifetimeless::Read;
use bevy::prelude::*;
use bevy::render::render_graph::ViewNode;

use bevy::render::render_resource::{
    BindGroupEntries, GpuArrayBuffer, Operations, PipelineCache, RenderPassColorAttachment,
    RenderPassDescriptor,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::{ViewUniformOffset, ViewUniforms};
use smallvec::{smallvec, SmallVec};

use crate::render::empty_buffer::EmptyBuffer;
use crate::render::extract::ExtractedLightOccluder2d;

use super::pipeline::SdfPipeline;
use super::SdfTexture;

const SDF_PASS: &str = "sdf_pass";
const SDF_BIND_GROUP: &str = "sdf_bind_group";

#[derive(Default)]
pub struct SdfNode;

impl ViewNode for SdfNode {
    type ViewQuery = (Read<ViewUniformOffset>, Read<SdfTexture>);

    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        (view_offset, sdf_texture): bevy::ecs::query::QueryItem<'w, Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let sdf_pipeline = world.resource::<SdfPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let (Some(pipeline), Some(view_uniform_binding), Some(light_occluders_binding)) = (
            pipeline_cache.get_render_pipeline(sdf_pipeline.pipeline_id),
            world.resource::<ViewUniforms>().uniforms.binding(),
            world
                .resource::<GpuArrayBuffer<ExtractedLightOccluder2d>>()
                .binding()
                .or(world.resource::<EmptyBuffer>().binding()),
        ) else {
            return Ok(());
        };

        let bind_group = render_context.render_device().create_bind_group(
            SDF_BIND_GROUP,
            &sdf_pipeline.layout,
            &BindGroupEntries::sequential((view_uniform_binding.clone(), light_occluders_binding)),
        );

        let mut sdf_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some(SDF_PASS),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &sdf_texture.sdf.default_view,
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

        sdf_pass.set_render_pipeline(pipeline);
        sdf_pass.set_bind_group(0, &bind_group, &dynamic_offsets);
        sdf_pass.draw(0..3, 0..1);

        Ok(())
    }
}
