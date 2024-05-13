use bevy::prelude::*;
use bevy::render::render_graph::ViewNode;

use bevy::render::render_resource::{BindGroupEntries, PipelineCache, RenderPassDescriptor};
use bevy::render::view::{ViewTarget, ViewUniforms};

use crate::LightingPassAssets;

use super::LightingPipeline;

#[derive(Default)]
pub struct LightingNode;

impl ViewNode for LightingNode {
    type ViewQuery = &'static ViewTarget;

    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        view_target: bevy::ecs::query::QueryItem<'w, Self::ViewQuery>,
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

        let Some(point_light_buffer) = world
            .resource::<LightingPassAssets>()
            .point_lights
            .binding()
        else {
            return Ok(());
        };

        let bind_group = render_context.render_device().create_bind_group(
            "lighting_bind_group",
            &lighting_pipeline.layout,
            &BindGroupEntries::sequential((view_uniform_binding, point_light_buffer)),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("lighting_pass"),
            color_attachments: &[Some(view_target.get_color_attachment())],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Setup fullscreen triangle.
        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
