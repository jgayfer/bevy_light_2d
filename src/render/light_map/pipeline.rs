use bevy::core_pipeline::FullscreenShader;
use bevy::ecs::resource::Resource;
use bevy::ecs::world::{FromWorld, World};
use bevy::render::render_resource::binding_types::{sampler, texture_2d, uniform_buffer};
use bevy::render::render_resource::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntries, CachedRenderPipelineId, ColorTargetState,
    ColorWrites, FragmentState, GpuArrayBuffer, MultisampleState, PipelineCache, PrimitiveState,
    RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
    TextureFormat, TextureSampleType,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::ViewUniform;

use crate::render::extract::{
    ExtractedAmbientLight2d, ExtractedPointLight2d, ExtractedSpotLight2d,
};

use super::{LIGHT_MAP_SHADER, PointLightMeta, SpotLightMeta};

const LIGHT_MAP_BIND_GROUP_LAYOUT: &str = "light_map_group_layout";
const LIGHT_MAP_PIPELINE: &str = "light_map_pipeline";

#[derive(Resource)]
pub struct LightMapPipeline {
    pub layout_descriptor: BindGroupLayoutDescriptor,
    pub sdf_sampler: Sampler,
    pub pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for LightMapPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let limits = &render_device.limits();
        let layout_descriptor = BindGroupLayoutDescriptor::new(
            LIGHT_MAP_BIND_GROUP_LAYOUT,
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    uniform_buffer::<ViewUniform>(true),
                    uniform_buffer::<ExtractedAmbientLight2d>(true),
                    GpuArrayBuffer::<ExtractedPointLight2d>::binding_layout(limits),
                    uniform_buffer::<PointLightMeta>(false),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    GpuArrayBuffer::<ExtractedSpotLight2d>::binding_layout(limits),
                    uniform_buffer::<SpotLightMeta>(false),
                ),
            ),
        );

        let sdf_sampler = render_device.create_sampler(&SamplerDescriptor::default());
        let fullscreen_shader = world.resource::<FullscreenShader>().clone();
        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some(LIGHT_MAP_PIPELINE.into()),
                    layout: vec![layout_descriptor.clone()],
                    vertex: fullscreen_shader.to_vertex_state(),
                    fragment: Some(FragmentState {
                        shader: LIGHT_MAP_SHADER,
                        shader_defs: vec![],
                        entry_point: Some("fragment".into()),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::Rgba16Float,
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                    zero_initialize_workgroup_memory: false,
                });

        Self {
            layout_descriptor,
            sdf_sampler,
            pipeline_id,
        }
    }
}
