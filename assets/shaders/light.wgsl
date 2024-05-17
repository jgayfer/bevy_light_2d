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

fn world_to_ndc(world_position: vec2<f32>, view_projection: mat4x4<f32>) -> vec2<f32> {
    return (view_projection * vec4<f32>(world_position, 0.0, 1.0)).xy;
}

fn ndc_to_screen(ndc: vec2<f32>, screen_size: vec2<f32>) -> vec2<f32> {
    let screen_position: vec2<f32> = (ndc + 1.0) * 0.5 * screen_size;
    return vec2(screen_position.x, (screen_size.y - screen_position.y));
}

fn world_to_screen(
    world_position: vec2<f32>,
    screen_size: vec2<f32>,
    view_projection: mat4x4<f32>
) -> vec2<f32> {
    return ndc_to_screen(world_to_ndc(world_position, view_projection), screen_size);
}

fn scale_factor(view: View) -> f32 {
    let screen_size =
        2.0 * vec2f(view.inverse_projection[0][0], view.inverse_projection[1][1]);
    return screen_size.y / view.viewport.w;
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
    // Setup the color to add to this position from lights sources.
    var light_color = vec3(0.0);

    // For each light, determine its illumination if we're within range of it.
    for (var i = 0u; i < arrayLength(&point_light_buffer.data); i++) {

        let point_light = point_light_buffer.data[i];

        // Our point light position is still in world space. We need to convert
        // it to screen space in order to do things like compute distances (let
        // alone render it in the correct place).
        let point_light_screen_center =
            world_to_screen(point_light.center, view.viewport.zw, view.projection);

        // Compute the distance between the current position and the light's center.
        // We multiply by the scale factor as otherwise our distance will always be
        // represented in actual pixels.
        let distance =
            distance(point_light_screen_center, vo.position.xy) * scale_factor(view);

        // If we're within the light's radius, it should provide some level
        // of illumination.
        if distance < point_light.radius {

            // Compute light color falloff (a value between 0.0 and 1.0).
            // The closer to the light we are, the higher the multiplier.
            let distance_multiplier = (point_light.radius - distance) / 100;

            // Add in the color from the light, taking into account the light's
            // energy and how far away it is.
            light_color +=
                point_light.color
                * point_light.energy
                * distance_multiplier
                * LIGHT_SOURCE_MULTIPLIER;
        }
    }

    // Compute ambient light, dividing by 100 as the input value is
    // interpreted as a percentage (e.g. 100 brightness is full brightness).
    let ambient_color = ambient_light.color * ambient_light.brightness / 100;

    return textureSample(screen_texture, texture_sampler, vo.uv)
        * vec4(ambient_color, 1.0)
        + vec4(light_color, 0.0);
}
