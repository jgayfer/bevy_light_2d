use bevy::prelude::*;

use bevy::render::render_resource::{
    BindGroupEntries, Operations, PipelineCache, RenderPassColorAttachment, RenderPassDescriptor,
};
use bevy::render::renderer::{RenderContext, ViewQuery};
use bevy::render::view::ViewTarget;

use crate::render::light_map::LightMapTexture;

use super::{LightingPipeline, LightingPipelineId};

const LIGHTING_PASS: &str = "lighting_pass";
const LIGHTING_BIND_GROUP: &str = "lighting_bind_group";

pub fn lighting_pass(
    world: &World,
    view: ViewQuery<(&ViewTarget, &LightingPipelineId, &LightMapTexture)>,
    mut ctx: RenderContext,
) {
    let (view_target, pipeline_id, light_map_texture) = view.into_inner();

    let pipeline = world.resource::<LightingPipeline>();
    let pipeline_cache = world.resource::<PipelineCache>();

    let Some(lighting_pipeline) = pipeline_cache.get_render_pipeline(pipeline_id.0) else {
        return;
    };

    let post_process = view_target.post_process_write();

    let bind_group = ctx.render_device().create_bind_group(
        LIGHTING_BIND_GROUP,
        &pipeline_cache.get_bind_group_layout(&pipeline.layout_descriptor),
        &BindGroupEntries::sequential((
            post_process.source,
            &light_map_texture.light_map.default_view,
            &pipeline.sampler,
        )),
    );

    let mut render_pass = ctx.begin_tracked_render_pass(RenderPassDescriptor {
        label: Some(LIGHTING_PASS),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: post_process.destination,
            resolve_target: None,
            ops: Operations::default(),
            depth_slice: None,
        })],
        ..default()
    });

    render_pass.set_render_pipeline(lighting_pipeline);
    render_pass.set_bind_group(0, &bind_group, &[]);
    render_pass.draw(0..3, 0..1);
}
