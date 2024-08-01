#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View
#import bevy_light_2d::types::LightOccluder2d

fn frag_coord_to_uv(frag_coord: vec2<f32>) -> vec2<f32> {
    return (frag_coord - view.viewport.xy) / view.viewport.zw;
}

fn frag_coord_to_ndc(frag_coord: vec2<f32>) -> vec2<f32> {
    return uv_to_ndc(frag_coord_to_uv(frag_coord.xy));
}

fn uv_to_ndc(uv: vec2<f32>) -> vec2<f32> {
    return uv * vec2(2.0, -2.0) + vec2(-1.0, 1.0);
}

fn ndc_to_world(ndc_position: vec2<f32>) -> vec2<f32> {
    return (view.world_from_clip * vec4(ndc_position, 0.0, 1.0)).xy;
}

// We're currently only using a single uniform binding for occluders in
// WebGL2, which is limited to 4kb in BatchedUniformBuffer, so we need to
// ensure our occluders can fit in 4kb.
//
// As each occluder is 16 bytes, we can fit 4096 / 16 = 256 occluders.
const MAX_OCCLUDERS: u32 = 256u;

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

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let pos = ndc_to_world(frag_coord_to_ndc(in.position.xy));

    var sdf = occluder_sd(pos, occluders[0]);

    // WebGL2 does not support storage buffers (or runtime sized arrays), so we
    // need to use a fixed number of occluders.
#if AVAILABLE_STORAGE_BUFFER_BINDINGS >= 6
    let occluder_count = arrayLength(&occluders);
#else
    let occluder_count = MAX_OCCLUDERS;
#endif

    for (var i = 1u; i < occluder_count; i++) {
        sdf = min(sdf, occluder_sd(pos, occluders[i]));
    }

    return vec4(sdf, 0.0, 0.0, 1.0);
}

fn occluder_sd(p: vec2f, occluder: LightOccluder2d) -> f32 {
  let local_pos = occluder.center - p;
  let d = abs(local_pos) - occluder.half_size;

  return length(max(d, vec2f(0.))) + min(max(d.x, d.y), 0.);
}
