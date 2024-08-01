use bevy::{
    prelude::*,
    render::{
        render_resource::{PipelineCache, SpecializedRenderPipelines},
        view::ExtractedView,
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
