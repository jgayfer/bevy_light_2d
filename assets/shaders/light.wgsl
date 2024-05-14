#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View

const FALLOFF_FACTOR: f32 = 0.5;

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
var<uniform> view: View;
@group(0) @binding(1)
var<storage> point_light_buffer: PointLight2dBuffer;

@fragment
fn fragment(vo: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var color = vec3(0.0);
    var alpha = 0.0;

    // For each light, check if we're within range of it, modifying the color and alpha if we are.
    for (var i = 0u; i < arrayLength(&point_light_buffer.data); i++) {

        let point_light = point_light_buffer.data[i];

        // Compute the distance between the current position and the current light's center.
        let distance = distance(point_light.center * 2., vo.position.xy);

        if distance < point_light.radius {

            // Compute light falloff. A value between 0.0 and 1.0, using our falloff factor to normalize it.
            let falloff = (point_light.radius - distance) / 100 * FALLOFF_FACTOR;

            alpha += falloff;
            color += point_light.color * vec3(point_light.energy) * falloff;
        }
    }

    return vec4(color, max(alpha, 0.5));
}
