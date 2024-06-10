#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View

// We're currently only using a single uniform binding for point lights in 
// WebGL2, which is limited to 4kb in BatchedUniformBuffer, so we need to
// ensure our point lights can fit in 4kb.
const MAX_POINT_LIGHTS: u32 = 82u;

struct PointLight2d {
    center: vec2f,
    radius: f32,
    color: vec4<f32>,
    intensity: f32,
    falloff: f32
}

struct AmbientLight2d {
    color: vec3<f32>
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
var<uniform> ambient_light: AmbientLight2d;

// WebGL2 does not support storage buffers, so we fall back to a fixed length
// array in a uniform buffer.
#if AVAILABLE_STORAGE_BUFFER_BINDINGS >= 6
    @group(0) @binding(4)
    var<storage> point_lights: array<PointLight2d>;
#else
    @group(0) @binding(4)
    var<uniform> point_lights: array<PointLight2d, MAX_POINT_LIGHTS>;
#endif

@fragment
fn fragment(vo: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Setup aggregate color from light sources to multiply the main texture by.
    var light_color = vec3(1.0);

    // WebGL2 does not support storage buffers (or runtime sized arrays), so we
    // need to use a fixed number of point lights.
#if AVAILABLE_STORAGE_BUFFER_BINDINGS >= 6
    let point_light_count = arrayLength(&point_lights);
#else
    let point_light_count = MAX_POINT_LIGHTS;
#endif

    // For each light, determine its illumination if we're within range of it.
    for (var i = 0u; i < point_light_count; i++) {

        let point_light = point_lights[i];

        // Our point light position is still in world space. We need to convert
        // it to screen space in order to do things like compute distances (let
        // alone render it in the correct place).
        let point_light_screen_center =
            world_to_screen(point_light.center, view.viewport.zw, view.view_proj);

        // Compute the distance between the current position and the light's center.
        // We multiply by the scale factor as otherwise our distance will always be
        // represented in actual pixels.
        let distance =
            distance(point_light_screen_center, vo.position.xy) * scale_factor(view);

        // If we're within the light's radius, it should provide some level
        // of illumination.
        if distance < point_light.radius {

            // Compute light color falloff (a value between 0.0 and 1.0).
            let attenuation = attenuation(
                distance,
                point_light.radius,
                point_light.intensity,
                point_light.falloff
            );

            // Add in the color from the light, taking into account its attenuation.
            light_color += point_light.color.rgb * attenuation;
        }
    }

    return textureSample(screen_texture, texture_sampler, vo.uv)
        * vec4(ambient_light.color.rgb, 1.0)
        * vec4(light_color, 1.0);
}

fn square(x: f32) -> f32 {
    return x * x;
}

// Compute light attenutation.
// See https://lisyarus.github.io/blog/posts/point-light-attenuation.html
fn attenuation(distance: f32, radius: f32, intensity: f32, falloff: f32) -> f32 {
    let s = distance / radius;
    if (s > 1.0) {
        return 0.0;
    }
    let s2 = square(s);
    return intensity * square(1 - s2) / (1 + falloff * s2);
}
