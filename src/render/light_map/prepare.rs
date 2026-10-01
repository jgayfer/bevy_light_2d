use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    render::{
        render_resource::{
            PipelineCache, SpecializedRenderPipelines, TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages,
        },
        renderer::{RenderDevice, RenderQueue},
        texture::TextureCache,
        view::{Msaa, ViewTarget},
    },
};

use crate::render::extract::{ExtractedLight2d, ExtractedPointLight2d, ExtractedSpotLight2d};

use super::{
    LightMapPipeline, LightMapPipelineId, LightMapPipelineKey, LightMapTexture, PointLightMeta,
    PointLightMetaBuffer, SpotLightMeta, SpotLightMetaBuffer,
};

const LIGHT_MAP_TEXTURE: &str = "light_map_texture";

pub fn prepare_light_map_pipelines(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    mut pipelines: ResMut<SpecializedRenderPipelines<LightMapPipeline>>,
    light_map_pipeline: Res<LightMapPipeline>,
    views: Query<(Entity, &Msaa, &ExtractedLight2d)>,
) {
    for (entity, msaa, light_2d) in &views {
        let key = LightMapPipelineKey {
            z_sorting: light_2d.z_sorting,
            multisampled: msaa.samples() > 1,
        };
        let id = pipelines.specialize(&pipeline_cache, &light_map_pipeline, key);
        commands
            .entity(entity)
            .insert(LightMapPipelineId { id, key });
    }
}

pub fn prepare_light_map_texture(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut texture_cache: ResMut<TextureCache>,
    view_targets: Query<(Entity, &ViewTarget)>,
) {
    for (entity, view_target) in &view_targets {
        let light_map_texture = texture_cache.get(
            &render_device,
            TextureDescriptor {
                label: Some(LIGHT_MAP_TEXTURE),
                size: view_target.main_texture().size(),
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba16Float,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
        );

        commands.entity(entity).insert(LightMapTexture {
            light_map: light_map_texture,
        });
    }
}

pub fn prepare_point_light_count(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    point_lights: Query<&ExtractedPointLight2d>,
    mut point_light_count: ResMut<PointLightMetaBuffer>,
) {
    let meta = PointLightMeta::new(point_lights.iter().len() as u32);
    point_light_count.buffer.set(meta);
    point_light_count
        .buffer
        .write_buffer(&render_device, &render_queue);
}

pub fn prepare_spot_light_count(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    spot_lights: Query<&ExtractedSpotLight2d>,
    mut spot_light_count: ResMut<SpotLightMetaBuffer>,
) {
    let meta = SpotLightMeta::new(spot_lights.iter().len() as u32);
    spot_light_count.buffer.set(meta);
    spot_light_count
        .buffer
        .write_buffer(&render_device, &render_queue);
}
