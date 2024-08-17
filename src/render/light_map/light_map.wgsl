#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View
#import bevy_light_2d::types::{AmbientLight2d, PointLight2d};
#import bevy_light_2d::view_transformations::{
    frag_coord_to_ndc,
    ndc_to_world,
    ndc_to_uv,
    world_to_ndc
};

// We're currently only using a single uniform binding for point lights in
// WebGL2, which is limited to 4kb in BatchedUniformBuffer, so we need to
// ensure our point lights can fit in 4kb.
const MAX_POINT_LIGHTS: u32 = 82u;

@group(0) @binding(0)
var<uniform> view: View;

@group(0) @binding(1)
var<uniform> ambient_light: AmbientLight2d;

// WebGL2 does not support storage buffers, so we fall back to a fixed length
// array in a uniform buffer.
#if AVAILABLE_STORAGE_BUFFER_BINDINGS >= 6
    @group(0) @binding(2)
    var<storage> point_lights: array<PointLight2d>;
#else
    @group(0) @binding(2)
    var<uniform> point_lights: array<PointLight2d, MAX_POINT_LIGHTS>;
#endif

@group(0) @binding(3)
var sdf: texture_2d<f32>;

@group(0) @binding(4)
var sdf_sampler: sampler;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let pos = ndc_to_world(frag_coord_to_ndc(in.position.xy));

    if get_distance(pos) <= 0.0 {
        return vec4(ambient_light.color.rgb, 1.0);
    }

    var lighting_color = vec3(1.0);

    // WebGL2 does not support storage buffers (or runtime sized arrays), so we
    // need to use a fixed number of point lights.
#if AVAILABLE_STORAGE_BUFFER_BINDINGS >= 6
    let point_light_count = arrayLength(&point_lights);
#else
    let point_light_count = MAX_POINT_LIGHTS;
#endif

    for (var i = 0u; i < point_light_count; i++) {
        let light = point_lights[i];
        let dist = distance(light.center, pos);

        if dist < light.radius {
            let raymarch = raymarch(pos, light.center);

            if raymarch > 0.0 || light.cast_shadows == 0 {
                lighting_color += light.color.rgb * attenuation(light, dist);
            }
        }
    }

    return vec4(ambient_light.color.rgb, 1.0) + vec4(lighting_color, 1.0);
}

fn square(x: f32) -> f32 {
    return x * x;
}

// Compute light attenutation.
// See https://lisyarus.github.io/blog/posts/point-light-attenuation.html
fn attenuation(light: PointLight2d, dist: f32) -> f32 {
    let s = dist / light.radius;
    if s > 1.0 {
        return 0.0;
    }
    let s2 = square(s);
    return light.intensity * square(1 - s2) / (1 + light.falloff * s2);
}

fn get_distance(pos: vec2<f32>) -> f32 {
    let uv = ndc_to_uv(world_to_ndc(pos));
    let dist = textureSampleLevel(sdf, sdf_sampler, uv, 0.0).r;
    return dist;
}

fn distance_squared(a: vec2<f32>, b: vec2<f32>) -> f32 {
    let c = a - b;
    return dot(c, c);
}

fn raymarch(ray_origin: vec2<f32>, ray_target: vec2<f32>) -> f32 {
    let ray_direction = normalize(ray_target - ray_origin);
    let stop_at = distance_squared(ray_origin, ray_target);

    var ray_progress: f32 = 0.0;
    var pos = vec2<f32>(0.0);

    for (var i = 0; i < 32; i++) {
        pos = ray_origin + ray_progress * ray_direction;

        if (ray_progress * ray_progress >= stop_at) {
            // ray found target
            return 1.0;
        }

        let dist = get_distance(pos);

        if dist <= 0.0 {
            break;
        }

        ray_progress += dist;
    }

    // ray found occluder
    return 0.0;
}
