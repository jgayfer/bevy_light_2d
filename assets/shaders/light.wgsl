#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View

const LIGHT_SOURCE_MULTIPLIER: f32 = 0.5;

struct PointLight2d {
    center: vec2f,
    radius: f32,
    color: vec3f,
    energy: f32,
}

struct PointLight2dBuffer {
    data: array<PointLight2d>
}

struct AmbientLight2d {
    color: vec3f,
    brightness: f32
}


@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;
@group(0) @binding(2)
var<uniform> view: View;
@group(0) @binding(3)
var<storage> point_light_buffer: PointLight2dBuffer;
@group(0) @binding(4)
var<uniform> ambient_light: AmbientLight2d;

@fragment
fn fragment(vo: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let texture = textureSample(screen_texture, texture_sampler, vo.uv);

    // The color of the main texture, before applying any lighting effects.
    var color = texture.rgb;

    // Blend in the ambient light, dividing by 100 as the input value is
    // interpreted as a percentage (e.g. 100 brightness is full brightness).
    color *= ambient_light.color * ambient_light.brightness / 100;

    // For each light, determine its illumination if we're within range of it.
    for (var i = 0u; i < arrayLength(&point_light_buffer.data); i++) {

        let point_light = point_light_buffer.data[i];

        // Compute the distance between the current position and the current
        // light's center.
        let distance = distance(point_light.center, vo.position.xy);

        // If we're within the light's radius, it should provide some level
        // of illumination.
        if distance < point_light.radius {

            // Compute light color falloff (a value between 0.0 and 1.0).
            // The closer to the light we are, the higher the multiplier.
            let distance_multiplier = (point_light.radius - distance) / 100;

            // Add in the color from the light, taking into account the light's
            // energy and how far away it is.
            color +=
                point_light.color
                * point_light.energy
                * distance_multiplier
                * LIGHT_SOURCE_MULTIPLIER;
        }
    }

    return vec4(color, texture.a);
}
