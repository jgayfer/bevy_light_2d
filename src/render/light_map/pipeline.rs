use bevy::asset::{Handle, load_embedded_asset};
use bevy::core_pipeline::FullscreenShader;
use bevy::ecs::resource::Resource;
use bevy::ecs::world::{FromWorld, World};
use bevy::render::render_resource::binding_types::{
    sampler, texture_2d, texture_depth_2d, texture_depth_2d_multisampled, uniform_buffer,
};
use bevy::render::render_resource::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntries, BindGroupLayoutEntry, ColorTargetState,
    ColorWrites, FragmentState, GpuArrayBuffer, MultisampleState, PrimitiveState,
    RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
    SpecializedRenderPipeline, TextureFormat, TextureSampleType,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::ViewUniform;
use bevy::shader::Shader;

use crate::render::extract::{
    ExtractedAmbientLight2d, ExtractedPointLight2d, ExtractedSpotLight2d,
};

use super::{LightMapPipelineKey, PointLightMeta, SpotLightMeta};

const LIGHT_MAP_BIND_GROUP_LAYOUT: &str = "light_map_group_layout";
const LIGHT_MAP_PIPELINE: &str = "light_map_pipeline";

pub const DEPTH_TEXTURE_BINDING: u32 = 8;

#[derive(Resource)]
pub struct LightMapPipeline {
    pub base_entries: Vec<BindGroupLayoutEntry>,
    pub sdf_sampler: Sampler,
    pub fullscreen_shader: FullscreenShader,
    pub shader: Handle<Shader>,
}

impl LightMapPipeline {
    pub fn layout_descriptor(&self, key: LightMapPipelineKey) -> BindGroupLayoutDescriptor {
        let mut entries = self.base_entries.clone();
        if key.z_sorting {
            let depth_texture_binding = if key.multisampled {
                texture_depth_2d_multisampled()
            } else {
                texture_depth_2d()
            };
            entries
                .push(depth_texture_binding.build(DEPTH_TEXTURE_BINDING, ShaderStages::FRAGMENT));
        }
        BindGroupLayoutDescriptor::new(LIGHT_MAP_BIND_GROUP_LAYOUT, &entries)
    }
}

impl FromWorld for LightMapPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let limits = &render_device.limits();
        let base_entries = BindGroupLayoutEntries::sequential(
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
        )
        .to_vec();

        let sdf_sampler = render_device.create_sampler(&SamplerDescriptor::default());
        let fullscreen_shader = world.resource::<FullscreenShader>().clone();
        let shader = load_embedded_asset!(world, "light_map.wesl");

        Self {
            base_entries,
            sdf_sampler,
            fullscreen_shader,
            shader,
        }
    }
}

impl SpecializedRenderPipeline for LightMapPipeline {
    type Key = LightMapPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut shader_defs = vec![];
        if key.z_sorting {
            shader_defs.push("Z_SORTING".into());
        }
        if key.multisampled {
            shader_defs.push("MULTISAMPLED".into());
        }

        RenderPipelineDescriptor {
            label: Some(LIGHT_MAP_PIPELINE.into()),
            layout: vec![self.layout_descriptor(key)],
            vertex: self.fullscreen_shader.to_vertex_state(),
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                shader_defs,
                entry_point: Some("fragment".into()),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba16Float,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
                constants: Vec::new(),
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            immediate_size: 0,
            zero_initialize_workgroup_memory: false,
        }
    }
}
