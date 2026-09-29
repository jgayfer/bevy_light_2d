#define_import_path bevy_light_2d::view_transformations

#import bevy_render::view::View

@group(0) @binding(0) var<uniform> view: View;

fn world_to_ndc(world_position: vec2<f32>) -> vec2<f32> {
    return (view.clip_from_world * vec4(world_position, 0.0, 1.0)).xy;
}

fn ndc_to_world(ndc_position: vec2<f32>) -> vec2<f32> {
    return (view.world_from_clip * vec4(ndc_position, 0.0, 1.0)).xy;
}

fn depth_to_world_z(ndc_position: vec2<f32>, depth: f32) -> f32 {
    let world = view.world_from_clip * vec4(ndc_position, depth, 1.0);
    return world.z / world.w;
}

fn world_units_per_pixel() -> f32 {
    let step = vec2(2.0 / view.viewport.z, 0.0);
    return length(ndc_to_world(step) - ndc_to_world(vec2(0.0)));
}

fn frag_coord_to_uv(frag_coord: vec2<f32>) -> vec2<f32> {
    return (frag_coord - view.viewport.xy) / view.viewport.zw;
}

fn frag_coord_to_ndc(frag_coord: vec2<f32>) -> vec2<f32> {
    return uv_to_ndc(frag_coord_to_uv(frag_coord.xy));
}

fn uv_to_ndc(uv: vec2<f32>) -> vec2<f32> {
    return uv * vec2(2.0, -2.0) + vec2(-1.0, 1.0);
}

fn ndc_to_uv(ndc: vec2<f32>) -> vec2<f32> {
    return ndc * vec2(0.5, -0.5) + vec2(0.5);
}
