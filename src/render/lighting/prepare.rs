use bevy::{
    prelude::*,
    render::{
        render_resource::{PipelineCache, SpecializedRenderPipelines},
        view::ExtractedView,
    },
};
use bevy::render::render_resource::{TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::TextureCache;
use bevy::render::view::ViewTarget;
use crate::light::PointLight2dImageTexture;
use crate::render::extract::ExtractedAmbientLight2d;
use super::{LightImageTexture, LightingPipeline, LightingPipelineId, LightingPipelineKey};

const LIGHT_IMAGE_TEXTURE_LABEL: &str = "light_image_texture";

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

//TODO: figure out how to "insert image or identity"
pub fn prepare_light_image_textures(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut texture_cache: ResMut<TextureCache>,
    view_targets: Query<(Entity, &ViewTarget, Option<&PointLight2dImageTexture>)>,
){
    for (entity, view_target, maybe_image_texture) in &view_targets{
        let texture = texture_cache.get(
            &render_device,
            TextureDescriptor {
                label: Some(LIGHT_IMAGE_TEXTURE_LABEL),
                size: view_target.main_texture().size(),
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba16Float,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
        );

        commands.entity(entity).insert(LightImageTexture {
            texture
        });
    }
}