use bevy::{
    prelude::*,
    render::{
        render_resource::{
            PipelineCache, SpecializedRenderPipelines, TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages,
        },
        renderer::RenderDevice,
        texture::{CachedTexture, TextureCache},
        view::{ExtractedView, ViewTarget},
    },
};

use crate::render::extract::ExtractedAmbientLight2d;

use super::{LightingPipeline, LightingPipelineId, LightingPipelineKey};

pub fn prepare_lighting_pipelines(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    mut pipelines: ResMut<SpecializedRenderPipelines<LightingPipeline>>,
    lighting_pipeline: Res<LightingPipeline>,
    view_targets: Query<(Entity, &ExtractedView), With<ExtractedAmbientLight2d>>,
) {
    for (entity, view) in view_targets.iter() {
        let pipeline_id = pipelines.specialize(
            &pipeline_cache,
            &lighting_pipeline,
            LightingPipelineKey { hdr: view.hdr },
        );

        commands
            .entity(entity)
            .insert(LightingPipelineId(pipeline_id));
    }
}

fn create_aux_texture(
    view_target: &ViewTarget,
    texture_cache: &mut TextureCache,
    render_device: &RenderDevice,
    label: &'static str,
) -> CachedTexture {
    texture_cache.get(
        render_device,
        TextureDescriptor {
            label: Some(label),
            size: view_target.main_texture().size(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
    )
}

#[derive(Component)]
pub struct Lighting2dAuxiliaryTextures {
    pub sdf: CachedTexture,
    pub light_map: CachedTexture,
}

pub fn prepare_lighting_auxiliary_textures(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut texture_cache: ResMut<TextureCache>,
    view_targets: Query<(Entity, &ViewTarget)>,
) {
    for (entity, view_target) in &view_targets {
        commands.entity(entity).insert(Lighting2dAuxiliaryTextures {
            sdf: create_aux_texture(view_target, &mut texture_cache, &render_device, "sdf"),
            light_map: create_aux_texture(
                view_target,
                &mut texture_cache,
                &render_device,
                "light_map",
            ),
        });
    }
}
