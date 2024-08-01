use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    render::{
        render_resource::{TextureDescriptor, TextureDimension, TextureFormat, TextureUsages},
        renderer::RenderDevice,
        texture::TextureCache,
        view::ViewTarget,
    },
};

use super::SdfTexture;

const SDF_TEXTURE: &str = "sdf_texture";

pub fn prepare_sdf_texture(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut texture_cache: ResMut<TextureCache>,
    view_targets: Query<(Entity, &ViewTarget)>,
) {
    for (entity, view_target) in &view_targets {
        let sdf_texture = texture_cache.get(
            &render_device,
            TextureDescriptor {
                label: Some(SDF_TEXTURE),
                size: view_target.main_texture().size(),
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba16Float,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
        );

        commands
            .entity(entity)
            .insert(SdfTexture { sdf: sdf_texture });
    }
}
