#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View

struct PointLight2d {
    center: vec2f,
    radius: f32,
    color: vec3f,
    energy: f32,
}

struct PointLight2dBuffer {
    data: array<PointLight2d>
}

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;
@group(0) @binding(2)
var<uniform> view: View;
@group(0) @binding(3)
var<storage> point_light_buffer: PointLight2dBuffer;

@fragment
fn fragment(vo: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    for (var i = 0u; i < arrayLength(&point_light_buffer.data); i++) {

        let point_light = point_light_buffer.data[i];
        let distance = distance(point_light.center * 2., vo.position.xy);

        if distance < point_light.radius {
            // TODO: Blend in multiple light sources, take color into account, etc.
            return vec4(1., 1., 1., 1.);
        }
    }

    return textureSample(screen_texture, texture_sampler, vo.uv);
}
