// Fish scales shader
//
// Creates iridescent overlapping fish scales with
// animated shimmer effect.

#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct FishScalesData {
    base_color: vec4<f32>,
    iridescence_color: vec4<f32>,
    scale_size: f32,
    shimmer_speed: f32,
}

@group(3) @binding(0)
var<uniform> material: FishScalesData;

// Scale pattern using offset rows
fn scale_pattern(p: vec2<f32>) -> vec3<f32> {
    // Offset every other row
    var uv = p;
    let row = floor(uv.y);
    if (i32(row) % 2) == 1 {
        uv.x += 0.5;
    }

    // Get cell coordinates
    let cell = floor(uv);
    let local = fract(uv);

    // Create overlapping oval scales
    let center = vec2<f32>(0.5, 0.7); // Offset center for overlap
    let dist = length((local - center) * vec2<f32>(1.0, 1.5));

    // Scale edge
    let edge = smoothstep(0.45, 0.5, dist);

    // Scale highlight (rim lighting effect)
    let highlight_pos = vec2<f32>(0.3, 0.4);
    let highlight = 1.0 - smoothstep(0.0, 0.3, length(local - highlight_pos));

    // Return: edge, highlight, cell ID for variation
    let cell_id = fract(sin(dot(cell, vec2<f32>(127.1, 311.7))) * 43758.5453);
    return vec3<f32>(edge, highlight, cell_id);
}

// Iridescence based on view angle simulation
fn iridescence(uv: vec2<f32>, time: f32) -> vec3<f32> {
    let angle = uv.x * 6.28 + uv.y * 3.14 + time * 0.5;

    // Create spectral colors
    let r = sin(angle) * 0.5 + 0.5;
    let g = sin(angle + 2.094) * 0.5 + 0.5;
    let b = sin(angle + 4.188) * 0.5 + 0.5;

    return vec3<f32>(r, g, b);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = globals.time;

    // Scale the UVs for scale pattern
    let scaled_uv = uv * material.scale_size;

    // Get scale pattern
    let scale = scale_pattern(scaled_uv);
    let edge = scale.x;
    let highlight = scale.y;
    let variation = scale.z;

    // Base color with per-scale variation
    var base = material.base_color.rgb;
    base *= 0.85 + variation * 0.3;

    // Iridescence effect
    let irid = iridescence(uv + variation, time * material.shimmer_speed);
    let irid_color = material.iridescence_color.rgb * irid;

    // Animated shimmer wave
    let shimmer_wave = sin(uv.x * 20.0 - time * material.shimmer_speed * 2.0 + variation * 6.28);
    let shimmer = max(0.0, shimmer_wave) * 0.3;

    // Combine colors
    var final_color = base;

    // Add iridescence based on angle
    final_color = mix(final_color, irid_color, 0.3 + shimmer * 0.2);

    // Apply scale edge darkening
    final_color *= 0.6 + (1.0 - edge) * 0.4;

    // Add highlight on each scale
    final_color += highlight * 0.25 * (1.0 + shimmer);

    // Subtle color variation per scale
    final_color += (variation - 0.5) * 0.1;

    // Add overall shimmer
    final_color += shimmer * vec3<f32>(0.2, 0.25, 0.3);

    return vec4<f32>(final_color, 1.0);
}
