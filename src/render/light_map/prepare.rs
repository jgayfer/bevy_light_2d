use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    render::{
        render_resource::{TextureDescriptor, TextureDimension, TextureFormat, TextureUsages},
        renderer::{RenderDevice, RenderQueue},
        texture::TextureCache,
        view::ViewTarget,
    },
};

use crate::render::extract::ExtractedPointLight2d;

use super::{LightMapTexture, PointLightMeta, PointLightMetaBuffer};

const LIGHT_MAP_TEXTURE: &str = "light_map_texture";

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
