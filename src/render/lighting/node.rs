use bevy::ecs::system::lifetimeless::Read;
use bevy::prelude::*;
use bevy::render::render_graph::ViewNode;

use bevy::render::render_resource::{
    BindGroupEntries, Operations, PipelineCache, RenderPassColorAttachment, RenderPassDescriptor,
};
use bevy::render::view::ViewTarget;

use crate::render::light_map::LightMapTexture;

use super::{LightingPipeline, LightingPipelineId};

const LIGHTING_PASS: &str = "lighting_pass";
const LIGHTING_BIND_GROUP: &str = "lighting_bind_group";

#[derive(Default)]
pub struct LightingNode;

impl ViewNode for LightingNode {
    type ViewQuery = (
        Read<ViewTarget>,
        Read<LightingPipelineId>,
        Read<LightMapTexture>,
    );

    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        (view_target, pipeline_id, light_map_texture): bevy::ecs::query::QueryItem<
            'w,
            Self::ViewQuery,
        >,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let pipeline = world.resource::<LightingPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(lighting_pipeline) = pipeline_cache.get_render_pipeline(pipeline_id.0) else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();

        let bind_group = render_context.render_device().create_bind_group(
            LIGHTING_BIND_GROUP,
            &pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &light_map_texture.light_map.default_view,
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
