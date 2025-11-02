#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct HolographicMaterial {
    base_color: vec4<f32>,
    interference_intensity: f32,
    scan_speed: f32,
}

@group(3) @binding(0) var<uniform> material: HolographicMaterial;

// Generate pseudo-random number from 2D coordinate
fn random(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

// 2D noise function
fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let a = random(i);
    let b = random(i + vec2<f32>(1.0, 0.0));
    let c = random(i + vec2<f32>(0.0, 1.0));
    let d = random(i + vec2<f32>(1.0, 1.0));

    let u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = globals.time;

    // Create scanning lines that move vertically
    let scan_line = sin((uv.y * 20.0) + (time * material.scan_speed)) * 0.5 + 0.5;

    // Create horizontal interference patterns
    let interference = sin((uv.x * 50.0) + (time * 3.0)) * 0.1;

    // Add some noise for digital glitch effect
    let noise_factor = noise(uv * 100.0 + time * 0.5) * 0.2;

    // Calculate holographic flicker
    let flicker = 0.9 + 0.1 * sin(time * 15.0);

    // Create RGB shift effect for authenticity
    let offset = 0.002;
    let r = scan_line + interference + noise_factor;
    let g = sin((uv.y * 20.0 + offset) + (time * material.scan_speed)) * 0.5 + 0.5 + interference;
    let b = sin((uv.y * 20.0 - offset) + (time * material.scan_speed)) * 0.5 + 0.5 + interference;

    // Combine base color with holographic effects
    var color = material.base_color.rgb;
    color.r *= r * flicker;
    color.g *= g * flicker;
    color.b *= b * flicker;

    // Add intensity variation
    color *= material.interference_intensity;

    // Edge fade effect - more transparent at edges
    let edge_fade = smoothstep(0.0, 0.1, uv.x) * smoothstep(1.0, 0.9, uv.x) *
                    smoothstep(0.0, 0.1, uv.y) * smoothstep(1.0, 0.9, uv.y);

    let alpha = material.base_color.a * edge_fade * flicker;

    return vec4<f32>(color, alpha);
}
