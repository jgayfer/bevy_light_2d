use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::prelude::*;
use bevy::render::render_resource::binding_types::{
    sampler, storage_buffer_read_only, texture_2d, uniform_buffer,
};
use bevy::render::render_resource::{
    BindGroupLayout, BindGroupLayoutEntries, CachedRenderPipelineId, ColorTargetState, ColorWrites,
    FragmentState, MultisampleState, PipelineCache, PrimitiveState, RenderPipelineDescriptor,
    Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, TextureFormat, TextureSampleType,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::BevyDefault;
use bevy::render::view::ViewUniform;

use crate::render::extract::ExtractedAmbientLight2d;
use crate::render::gpu::GpuPointLight2d;

#[derive(Resource)]
pub struct LightingPipeline {
    pub layout: BindGroupLayout,
    pub sampler: Sampler,
    pub pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for LightingPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "lighting_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<ViewUniform>(false),
                    storage_buffer_read_only::<Vec<GpuPointLight2d>>(false),
                    uniform_buffer::<ExtractedAmbientLight2d>(true),
                ),
            ),
        );

        let shader = world.resource::<AssetServer>().load("shaders/light.wgsl");

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("lighting_pipeline".into()),
                    layout: vec![layout.clone()],
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: "fragment".into(),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::bevy_default(),
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}
