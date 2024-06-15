use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::prelude::*;
use bevy::render::render_resource::binding_types::{sampler, texture_2d, uniform_buffer};
use bevy::render::render_resource::{
    BindGroupLayout, BindGroupLayoutEntries, ColorTargetState, ColorWrites, FragmentState,
    GpuArrayBuffer, MultisampleState, PrimitiveState, RenderPipelineDescriptor, Sampler,
    SamplerBindingType, SamplerDescriptor, ShaderStages, SpecializedRenderPipeline, TextureFormat,
    TextureSampleType,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::BevyDefault;
use bevy::render::view::{ViewTarget, ViewUniform};

use crate::render::extract::{ExtractedAmbientLight2d, ExtractedPointLight2d};

use super::{LightingPipelineKey, LIGHTING_SHADER};

const LIGHTING_PIPELINE: &str = "lighting_pipeline";
const LIGHTING_BIND_GROUP_LAYOUT: &str = "lighting_bind_group_layout";

#[derive(Resource)]
pub struct LightingPipeline {
    pub layout: BindGroupLayout,
    pub sampler: Sampler,
}

impl FromWorld for LightingPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            LIGHTING_BIND_GROUP_LAYOUT,
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<ViewUniform>(true),
                    uniform_buffer::<ExtractedAmbientLight2d>(true),
                    GpuArrayBuffer::<ExtractedPointLight2d>::binding_layout(render_device),
                ),
            ),
        );

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        Self { layout, sampler }
    }
}

impl SpecializedRenderPipeline for LightingPipeline {
    type Key = LightingPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some(LIGHTING_PIPELINE.into()),
            layout: vec![self.layout.clone()],
            vertex: fullscreen_shader_vertex_state(),
            fragment: Some(FragmentState {
                shader: LIGHTING_SHADER,
                shader_defs: vec![],
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: if key.hdr {
                        ViewTarget::TEXTURE_FORMAT_HDR
                    } else {
                        TextureFormat::bevy_default()
                    },
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
        }
    }
}
