// Emissive geometric pattern shader with animated effects
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    mesh_view_bindings::globals,
}

struct GeometricData {
    primary_color: vec4<f32>,
    secondary_color: vec4<f32>,
    glow_intensity: f32,
    animation_speed: f32,
    metallic: f32,
    roughness: f32,
    time: f32,
    _padding: f32,
}

@group(3) @binding(0) var<uniform> material: GeometricData;
@group(3) @binding(1) var noise_texture: texture_2d<f32>;
@group(3) @binding(2) var noise_sampler: sampler;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // Get time for animation
    let time = globals.time;

    // Sample noise texture with animated UV coordinates
    let animated_uv = in.uv + vec2<f32>(time * 0.1, time * 0.05) * material.animation_speed;
    let noise = textureSample(noise_texture, noise_sampler, animated_uv).r;

    // Create animated patterns
    let wave1 = sin(in.uv.x * 10.0 + time * material.animation_speed * 2.0) * 0.5 + 0.5;
    let wave2 = cos(in.uv.y * 8.0 + time * material.animation_speed * 1.5) * 0.5 + 0.5;
    let pattern = wave1 * wave2 * noise;

    // Mix colors based on pattern
    let base_color = mix(material.primary_color, material.secondary_color, pattern);

    // Create pulsing glow effect
    let pulse = sin(time * material.animation_speed * 3.0) * 0.3 + 0.7;
    let glow_factor = pattern * material.glow_intensity * pulse;

    // Calculate final color with emissive glow
    let emissive_contribution = base_color.rgb * glow_factor;
    let final_color = base_color.rgb + emissive_contribution;

    var out: FragmentOutput;
    out.color = vec4<f32>(final_color, base_color.a);

    return out;
}
