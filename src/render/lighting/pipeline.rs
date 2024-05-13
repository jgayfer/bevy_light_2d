use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::prelude::*;
use bevy::render::render_resource::binding_types::{storage_buffer_read_only, uniform_buffer};
use bevy::render::render_resource::{
    BindGroupLayout, BindGroupLayoutEntries, BlendComponent, BlendFactor, BlendOperation,
    BlendState, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState,
    MultisampleState, PipelineCache, PrimitiveState, RenderPipelineDescriptor, Sampler,
    SamplerDescriptor, ShaderStages, TextureFormat,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::BevyDefault;
use bevy::render::view::ViewUniform;

use crate::GpuPointLight2dBuffer;

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
                    uniform_buffer::<ViewUniform>(false),
                    storage_buffer_read_only::<GpuPointLight2dBuffer>(false),
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
                            blend: Some(BLEND),
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState {
                        count: 4,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    push_constant_ranges: vec![],
                });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}

const BLEND: BlendState = BlendState {
    color: BlendComponent {
        src_factor: BlendFactor::SrcAlpha,
        dst_factor: BlendFactor::One,
        operation: BlendOperation::Add,
    },

    alpha: BlendComponent {
        src_factor: BlendFactor::OneMinusSrcAlpha,
        dst_factor: BlendFactor::One,
        operation: BlendOperation::Add,
    },
};
