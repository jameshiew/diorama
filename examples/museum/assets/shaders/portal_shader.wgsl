#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct PortalMaterial {
    center_color: vec4<f32>,
    edge_color: vec4<f32>,
    rotation_speed: f32,
    distortion_strength: f32,
}

@group(3) @binding(0) var<uniform> material: PortalMaterial;

// Rotate 2D point around origin
fn rotate2d(p: vec2<f32>, angle: f32) -> vec2<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec2<f32>(
        p.x * c - p.y * s,
        p.x * s + p.y * c
    );
}

// Generate fractal noise
fn fractal_noise(p: vec2<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 1.0;
    var max_value = 0.0;

    for (var i = 0; i < 6; i++) {
        let noise_val = sin(p.x * frequency) * cos(p.y * frequency);
        value += noise_val * amplitude;
        max_value += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    return value / max_value;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Center UV coordinates around 0.5
    let uv = in.uv - 0.5;
    let time = globals.time;

    // Calculate distance from center
    let dist = length(uv);

    // Create rotating coordinates
    let rotated_uv = rotate2d(uv, time * material.rotation_speed);

    // Create spiral pattern
    let angle = atan2(rotated_uv.y, rotated_uv.x);
    let spiral = sin((angle * 8.0) + (dist * 20.0) - (time * 4.0));

    // Add distortion waves
    let distortion = fractal_noise(rotated_uv * 10.0 + time * 0.5) * material.distortion_strength;
    let modified_dist = dist + distortion * 0.1;

    // Create tunnel effect
    let tunnel = smoothstep(0.0, 0.3, modified_dist) * smoothstep(0.8, 0.4, modified_dist);

    // Create energy rings
    let rings = sin(modified_dist * 30.0 - time * 6.0) * 0.5 + 0.5;

    // Combine spiral and rings
    let pattern = (spiral * 0.5 + 0.5) * rings * tunnel;

    // Create color gradient from center to edge
    let color_mix = smoothstep(0.0, 0.5, modified_dist);
    let portal_color = mix(material.center_color.rgb, material.edge_color.rgb, color_mix);

    // Add energy pulse effect
    let pulse = 0.8 + 0.2 * sin(time * 8.0);

    // Final color with pattern overlay
    let final_color = portal_color * pattern * pulse;

    // Create alpha based on tunnel effect and pattern intensity
    let alpha = tunnel * pattern * 0.8;

    return vec4<f32>(final_color, alpha);
}
