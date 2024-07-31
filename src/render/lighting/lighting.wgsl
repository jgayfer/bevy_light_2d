#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;

@group(0) @binding(1)
var light_map_texture: texture_2d<f32>;

@group(0) @binding(2)
var texture_sampler: sampler;

@fragment
fn fragment(vo: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let light_frag = textureSample(light_map_texture, texture_sampler, vo.uv);
    let scene_frag = textureSample(screen_texture, texture_sampler, vo.uv);
    return scene_frag * light_frag;
}
