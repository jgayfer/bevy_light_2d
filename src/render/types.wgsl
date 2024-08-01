#define_import_path bevy_light_2d::types

struct LightingSettings {
    // blur circle of confusion
    coc: f32,
}

struct AmbientLight2d {
    color: vec4<f32>,
}

struct LightOccluder2d {
    center: vec2<f32>,
    half_size: vec2<f32>,
}

struct PointLight2d {
    center: vec2<f32>,
    color: vec4<f32>,
    falloff: f32,
    intensity: f32,
    radius: f32,
}
