use bevy::ecs::system::lifetimeless::Read;
use bevy::prelude::*;
use bevy::render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy::render::render_graph::ViewNode;

use bevy::render::render_resource::{
    BindGroupEntries, GpuArrayBuffer, Operations, PipelineCache, RenderPassColorAttachment,
    RenderPassDescriptor,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::{ViewUniformOffset, ViewUniforms};
use smallvec::{smallvec, SmallVec};

use crate::render::empty_buffer::EmptyBuffer;
use crate::render::extract::{ExtractedAmbientLight2d, ExtractedPointLight2d};
use crate::render::sdf::SdfTexture;

use super::{LightMapPipeline, LightMapTexture, PointLightMetaBuffer};

const LIGHT_MAP_PASS: &str = "light_map_pass";
const LIGHT_MAP_BIND_GROUP: &str = "light_map_bind_group";

#[derive(Default)]
pub struct LightMapNode;

impl ViewNode for LightMapNode {
    type ViewQuery = (
        Read<DynamicUniformIndex<ExtractedAmbientLight2d>>,
        Read<ViewUniformOffset>,
        Read<LightMapTexture>,
        Read<SdfTexture>,
    );

    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        (ambient_index, view_offset, light_map_texture, sdf_texture): bevy::ecs::query::QueryItem<
            'w,
            Self::ViewQuery,
        >,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let light_map_pipeline = world.resource::<LightMapPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let (
            Some(pipeline),
            Some(view_uniform_binding),
            Some(ambient_light_uniform),
            Some(point_light_binding),
            Some(point_light_count_binding),
        ) = (
            pipeline_cache.get_render_pipeline(light_map_pipeline.pipeline_id),
            world.resource::<ViewUniforms>().uniforms.binding(),
            world
                .resource::<ComponentUniforms<ExtractedAmbientLight2d>>()
                .uniforms()
                .binding(),
            world
                .resource::<GpuArrayBuffer<ExtractedPointLight2d>>()
                .binding()
                .or(world.resource::<EmptyBuffer>().binding()),
            world.resource::<PointLightMetaBuffer>().buffer.binding(),
        )
        else {
            return Ok(());
        };

        let light_map_bind_group = render_context.render_device().create_bind_group(
            LIGHT_MAP_BIND_GROUP,
            &light_map_pipeline.layout,
            &BindGroupEntries::sequential((
                view_uniform_binding.clone(),
                ambient_light_uniform.clone(),
                point_light_binding.clone(),
                point_light_count_binding.clone(),
                &sdf_texture.sdf.default_view,
                &light_map_pipeline.sdf_sampler,
            )),
        );

        let mut light_map_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some(LIGHT_MAP_PASS),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &light_map_texture.light_map.default_view,
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

        light_map_pass.set_render_pipeline(pipeline);
        light_map_pass.set_bind_group(0, &light_map_bind_group, &light_map_offsets);
        light_map_pass.draw(0..3, 0..1);

        Ok(())
    }
}
