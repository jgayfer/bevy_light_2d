use bevy::{
    core_pipeline::core_2d::CORE_2D_DEPTH_FORMAT,
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
        view::{Msaa, ViewDepthStencilTexture, ViewTarget},
    },
};

use crate::render::extract::{ExtractedLight2d, ExtractedPointLight2d, ExtractedSpotLight2d};

use super::{
    LightMapPipeline, LightMapPipelineId, LightMapPipelineKey, LightMapTexture, PointLightMeta,
    PointLightMetaBuffer, SpotLightMeta, SpotLightMetaBuffer,
};

const LIGHT_MAP_TEXTURE: &str = "light_map_texture";
const Z_SORTING_DEPTH_TEXTURE: &str = "light_2d_z_sorting_depth_texture";

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

// Bevy doesn't use TEXTURE_BINDING for the 2d depth map, so it can't be used
// in a shader. We can hack that by overwriting ViewDepthStencilTexture with the texture
// binding flag. Hoping to get this changed upstream so we can remove this.
pub fn prepare_z_sorting_depth_texture(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut texture_cache: ResMut<TextureCache>,
    views: Query<(Entity, &ViewDepthStencilTexture, &ExtractedLight2d)>,
) {
    for (entity, depth, light_2d) in &views {
        if !light_2d.z_sorting {
            continue;
        }
        let texture = texture_cache.get(
            &render_device,
            TextureDescriptor {
                label: Some(Z_SORTING_DEPTH_TEXTURE),
                size: depth.texture().size(),
                mip_level_count: 1,
                sample_count: depth.texture().sample_count(),
                dimension: TextureDimension::D2,
                format: CORE_2D_DEPTH_FORMAT,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
        );

        commands
            .entity(entity)
            .insert(ViewDepthStencilTexture::new(texture, Some(0.0), None));
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
