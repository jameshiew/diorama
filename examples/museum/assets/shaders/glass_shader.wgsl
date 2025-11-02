// Translucent glass shader for display cases
// Provides realistic glass-like transparency with subtle refraction and fresnel effects

#import bevy_pbr::{
    mesh_view_bindings::{globals, view},
    forward_io::VertexOutput,
    utils::PI,
    view_transformations::position_world_to_clip,
}

@group(3) @binding(0) var<uniform> material: GlassMaterial;

struct GlassMaterial {
    base_color: vec4<f32>,
    transparency: f32,
    refraction_strength: f32,
    fresnel_power: f32,
    _padding: f32,
}

fn fresnel_schlick(cos_theta: f32, f0: f32) -> f32 {
    return f0 + (1.0 - f0) * pow(1.0 - cos_theta, 5.0);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Use the world normal for fresnel calculation
    let normal = normalize(in.world_normal);

    // Calculate view direction from camera to fragment
    let view_dir = normalize(view.world_position - in.world_position.xyz);

    // Fresnel calculation: angle between view direction and surface normal
    let cos_theta = max(0.0, dot(normal, view_dir));

    // Fresnel effect - more transparent when viewing straight on, more reflective at angles
    let fresnel = fresnel_schlick(cos_theta, 0.04); // Glass has ~4% reflectance at normal incidence
    let fresnel_adjusted = pow(fresnel, material.fresnel_power);

    // Base glass color with subtle blue tint
    var glass_color = material.base_color;

    // Add subtle UV-based distortion for glass imperfections
    let uv_distortion = sin(in.uv.x * 10.0) * sin(in.uv.y * 10.0) * 0.002;
    let distorted_uv = in.uv + vec2<f32>(uv_distortion);

    // Subtle color variation based on UV coordinates
    let color_variation = 1.0 + sin(distorted_uv.x * 3.14159) * sin(distorted_uv.y * 3.14159) * 0.02;

    // Subtle time-based shimmer effect
    let shimmer = sin(globals.time * 0.5 + in.world_position.x * 0.1 + in.world_position.z * 0.1) * 0.01 + 1.0;

    // Apply effects to color
    let modified_rgb = glass_color.rgb * color_variation * shimmer;

    // Calculate final transparency
    // More transparent when viewing straight on, less transparent at angles due to fresnel
    let base_transparency = material.transparency;
    let final_transparency = base_transparency * (1.0 - fresnel_adjusted * 0.3);

    return vec4<f32>(modified_rgb, final_transparency);
}
