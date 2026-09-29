import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_light_2d::render::types::{AmbientLight2d, PointLight2d, PointLightMeta, SpotLight2d, SpotLightMeta};
import bevy_light_2d::render::view_transformations::{
    depth_to_world_z,
    frag_coord_to_ndc,
    ndc_to_world,
    ndc_to_uv,
    world_to_ndc,
    world_units_per_pixel,
};

// We're currently only using a single uniform binding for point lights in
// WebGL2, which is limited to 4kb in BatchedUniformBuffer, so we need to
// ensure our point lights can fit in 4kb.
const MAX_POINT_LIGHTS: u32 = 82u;
const MAX_SPOT_LIGHTS:  u32 = 64u;

const F16_MAX: f32 = 65504.0;

@group(0) @binding(1)
var<uniform> ambient_light: AmbientLight2d;

// WebGL2 does not support storage buffers, so we fall back to a fixed length
// array in a uniform buffer.
@if(AVAILABLE_STORAGE_BUFFER_BINDINGS__GE_6)
    @group(0) @binding(2)
    var<storage> point_lights: array<PointLight2d>;
@else
    @group(0) @binding(2)
    var<uniform> point_lights: array<PointLight2d, MAX_POINT_LIGHTS>;

@group(0) @binding(3)
var<uniform> point_light_meta: PointLightMeta;

@group(0) @binding(4)
var sdf: texture_2d<f32>;

@group(0) @binding(5)
var sdf_sampler: sampler;

// Spot lights: SSBO on modern backends, UBO array on WebGL2
@if(AVAILABLE_STORAGE_BUFFER_BINDINGS__GE_6)
    @group(0) @binding(6)
    var<storage> spot_lights: array<SpotLight2d>;
@else
    @group(0) @binding(6)
    var<uniform> spot_lights: array<SpotLight2d, MAX_SPOT_LIGHTS>;

@group(0) @binding(7)
var<uniform> spot_light_meta: SpotLightMeta;

@if(Z_SORTING && MULTISAMPLED)
    @group(0) @binding(8)
    var depth_texture: texture_depth_multisampled_2d;
@elif(Z_SORTING)
    @group(0) @binding(8)
    var depth_texture: texture_depth_2d;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let ndc = frag_coord_to_ndc(in.position.xy);
    let pos = ndc_to_world(ndc);
    let sprite_z = sprite_z(in.position.xy, ndc);

    if is_shadowed(sample_sdf(pos), sprite_z) {
        return vec4(ambient_light.color.rgb, 1.0);
    }

    let min_step = world_units_per_pixel();

    var lighting_color = ambient_light.color.rgb;
    
    // Point lights
    for (var i = 0u; i < point_light_meta.count; i++) {
        let light = point_lights[i];
        let dist = distance(light.center, pos);

        if dist < light.radius {
            let raymarch = raymarch(pos, light.center, sprite_z, min_step);

            if raymarch > 0.0 || light.cast_shadows == 0 {
                lighting_color += light.color.rgb * attenuation(dist, light.radius, light.intensity, light.falloff);
            }
        }
    }

    // Spot lights
    for (var i = 0u; i < spot_light_meta.count; i++) {
        let light = spot_lights[i];
        let effective_center = get_effective_spot_light_center(light, pos);
        let dist = distance(effective_center, pos);
        if dist < light.radius {
            let mask = spot_mask(light, pos, effective_center);
            if mask > 0.0 {
                let vis = raymarch(pos, effective_center, sprite_z, min_step);
                if vis > 0.0 || light.cast_shadows == 0u {
                    lighting_color += light.color.rgb * attenuation(dist, light.radius, light.intensity, light.falloff) * mask;
                }
            }
        }
    }

    return vec4(lighting_color, 1.0);
}

fn square(x: f32) -> f32 {
    return x * x;
}

// Compute light attenutation.
// See https://lisyarus.github.io/blog/posts/point-light-attenuation.html
fn attenuation(dist: f32, radius: f32, intensity: f32, falloff: f32) -> f32 {
    let s = dist / radius;
    if s > 1.0 {
        return 0.0;
    }
    let s2 = square(s);
    return intensity * square(1.0 - s2) / (1.0 + falloff * s2);
}

fn sprite_z(frag_coord: vec2<f32>, ndc: vec2<f32>) -> f32 {
    @if(Z_SORTING)
        let depth = textureLoad(depth_texture, vec2<i32>(frag_coord), 0);
    @if(Z_SORTING)
        return depth_to_world_z(ndc, depth);
    @else
        return -F16_MAX;
}

fn sample_sdf(pos: vec2<f32>) -> vec4<f32> {
    let uv = ndc_to_uv(world_to_ndc(pos));
    return textureSampleLevel(sdf, sdf_sampler, uv, 0.0);
}

fn is_shadowed(sdf_sample: vec4<f32>, sprite_z: f32) -> bool {
    @if(Z_SORTING)
        return sdf_sample.r <= 0.0 && sdf_sample.g > sprite_z;
    @else
        return sdf_sample.r <= 0.0;
}

fn distance_squared(a: vec2<f32>, b: vec2<f32>) -> f32 {
    let c = a - b;
    return dot(c, c);
}

fn raymarch(ray_origin: vec2<f32>, ray_target: vec2<f32>, sprite_z: f32, min_step: f32) -> f32 {
    let ray_direction = normalize(ray_target - ray_origin);
    let stop_at = distance_squared(ray_origin, ray_target);

    var ray_progress: f32 = 0.0;
    var pos = vec2<f32>(0.0);

    for (var i = 0; i < 48; i++) {
        pos = ray_origin + ray_progress * ray_direction;

        if (ray_progress * ray_progress >= stop_at) {
            // ray found target
            return 1.0;
        }

        let sdf_sample = sample_sdf(pos);

        if is_shadowed(sdf_sample, sprite_z) {
            break;
        }

        ray_progress += max(abs(sdf_sample.r), min_step);
    }

    // ray found occluder
    return 0.0;
}

// Calculates the mask for a given spotlight. 
// The direction, inner_angle, and outer_angle can be modulated to control the lit area of the spotlight.
// Returns: a 0..1 value representing the intensity of a spotlight at a given position
fn spot_mask(light: SpotLight2d, pos: vec2<f32>, effective_center: vec2<f32>) -> f32 {
    let to_frag = normalize(pos - effective_center);
    let cos_theta = dot(-to_frag, normalize(light.direction));
    let cos_inner = cos(light.inner_angle);
    let cos_outer = cos(light.outer_angle);
    return clamp(smoothstep(cos_outer, cos_inner, cos_theta), 0.0, 1.0);
}

// Calculates the effective center for a light from a given source_width.
// If the source_width is 0, this will simply return the center position.
// Returns: a vec2<f32> representing the closest point of the light source to the fragment.
fn get_effective_spot_light_center(light: SpotLight2d, frag_pos: vec2<f32>) -> vec2<f32> {
    if (light.source_width <= 0.0) {
        return light.center;
    }
    
    // Compute the direction of the light bar, which is perpendicular to the direction of the light
    let bar_direction = normalize(vec2<f32>(-light.direction.y, light.direction.x));
    
    // Compute the vector from the effective center of the light to the fragment position
    let to_frag = frag_pos - light.center;
    
    // Compute the projection of the fragment position onto the line defined by the light bar
    let projection = dot(to_frag, bar_direction);
    
    // Clamp the projection within the bounds of the actual width of the light bar
    let half_width = light.source_width * 0.5;
    let clamped_projection = clamp(projection, -half_width, half_width);
    
    return light.center + bar_direction * clamped_projection;
}
