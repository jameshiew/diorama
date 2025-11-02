#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct EnergyFieldMaterial {
    energy_color: vec4<f32>,
    arc_intensity: f32,
    flow_speed: f32,
    noise_scale: f32,
}

@group(3) @binding(0) var<uniform> material: EnergyFieldMaterial;

// Improved noise function for more organic electrical arcs
fn hash(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.13);
    p3 += dot(p3, p3.yzx + 3.333);
    return fract((p3.x + p3.y) * p3.z);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);

    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));

    let u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

// Fractal brownian motion for complex patterns
fn fbm(p: vec2<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 1.0;

    for (var i = 0; i < 5; i++) {
        value += amplitude * noise(p * frequency);
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    return value;
}

// Create electrical arc patterns
fn electrical_arc(uv: vec2<f32>, time: f32) -> f32 {
    let flow_uv = uv + vec2<f32>(time * material.flow_speed * 0.1, 0.0);

    // Main arc path with turbulence
    let main_path = fbm(flow_uv * material.noise_scale);
    let turbulence = fbm(flow_uv * material.noise_scale * 2.0 + vec2<f32>(time * 0.5, 0.0)) * 0.3;

    // Create branching patterns
    let branch1 = fbm(flow_uv * material.noise_scale * 1.5 + vec2<f32>(0.0, time * 0.3));
    let branch2 = fbm(flow_uv * material.noise_scale * 0.8 + vec2<f32>(time * 0.2, 0.0));

    // Combine patterns
    let combined = main_path + turbulence * 0.5 + branch1 * 0.3 + branch2 * 0.2;

    // Create sharp electric arc effect
    let arc_threshold = 0.4 + 0.1 * sin(time * 10.0 + uv.x * 20.0);
    let arc_intensity = smoothstep(arc_threshold - 0.05, arc_threshold + 0.05, combined);

    return arc_intensity;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = globals.time;

    // Create multiple electrical arc layers
    let arc1 = electrical_arc(uv, time);
    let arc2 = electrical_arc(uv + vec2<f32>(0.3, 0.1), time * 1.3);
    let arc3 = electrical_arc(uv + vec2<f32>(-0.2, 0.4), time * 0.7);

    // Combine arcs with different intensities
    let combined_arcs = arc1 + arc2 * 0.6 + arc3 * 0.4;

    // Create pulsing energy effect
    let pulse = 0.7 + 0.3 * sin(time * 6.0);
    let energy_pulse = 0.8 + 0.2 * sin(time * 12.0 + uv.x * 10.0);

    // Add flowing energy streams
    let stream_noise = fbm(uv * 8.0 + vec2<f32>(time * material.flow_speed, 0.0));
    let energy_stream = smoothstep(0.3, 0.7, stream_noise) * 0.5;

    // Create core energy field
    let field_base = fbm(uv * 4.0 + time * 0.2) * 0.3;

    // Combine all effects
    let total_energy = (combined_arcs * material.arc_intensity + energy_stream + field_base) * pulse * energy_pulse;

    // Apply energy color
    let energy_color = material.energy_color.rgb * total_energy;

    // Create alpha with additive blending in mind
    let alpha = total_energy * material.energy_color.a;

    // Add some sparkle effects
    let sparkle = step(0.98, hash(floor(uv * 100.0) + time)) * 2.0;
    let final_color = energy_color + sparkle * material.energy_color.rgb * 0.5;

    return vec4<f32>(final_color, alpha);
}
