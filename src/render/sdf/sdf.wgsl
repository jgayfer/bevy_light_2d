#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View
#import bevy_light_2d::types::{LightOccluder2d, OccluderMeta};
#import bevy_light_2d::view_transformations::{frag_coord_to_ndc, ndc_to_world};

// We're currently only using a single uniform binding for occluders in
// WebGL2, which is limited to 4kb in BatchedUniformBuffer, so we need to
// ensure our occluders can fit in 4kb.
//
// As each occluder is 24 bytes (with padding), we can fit 4096 / 24 = 170 occluders.
const MAX_OCCLUDERS: u32 = 170u;

@group(0) @binding(0)
var<uniform> view: View;

// WebGL2 does not support storage buffers, so we fall back to a fixed length
// array in a uniform buffer.
#if AVAILABLE_STORAGE_BUFFER_BINDINGS >= 6
    @group(0) @binding(1)
    var<storage> occluders: array<LightOccluder2d>;
#else
    @group(0) @binding(1)
    var<uniform> occluders: array<LightOccluder2d, MAX_OCCLUDERS>;
#endif

@group(0) @binding(2)
var<uniform> occluder_meta: OccluderMeta;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let pos = ndc_to_world(frag_coord_to_ndc(in.position.xy));

    // WebGL2 does not support storage buffers (or runtime sized arrays), so we
    // need to use a fixed number of occluders.
#if AVAILABLE_STORAGE_BUFFER_BINDINGS >= 6
    let occluder_count = occluder_meta.count;
#else
    let occluder_count = min(MAX_OCCLUDERS, occluder_meta.count);
#endif

    // If there aren't any occluders, use the max value for the texture.
    if (occluder_count == 0) {
        return vec4(255.0, 0.0, 0.0, 1.0);
    }

    var sdf = occluder_sd(pos, occluders[0]);

    for (var i = 1u; i < occluder_count; i++) {
        sdf = min(sdf, occluder_sd(pos, occluders[i]));
    }

    return vec4(sdf, 0.0, 0.0, 1.0);
}

fn occluder_sd(p: vec2f, occluder: LightOccluder2d) -> f32 {
  let local_pos = occluder.center - p;

  // Rotate into occluder's local space (negate rotation to go world→local)
  let cos_r = cos(-occluder.rotation);
  let sin_r = sin(-occluder.rotation);
  let rotated_pos = vec2f(
    local_pos.x * cos_r - local_pos.y * sin_r,
    local_pos.x * sin_r + local_pos.y * cos_r
  );

  let d = abs(rotated_pos) - occluder.half_size;
  return length(max(d, vec2f(0.))) + min(max(d.x, d.y), 0.);
}
