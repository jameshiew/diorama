// Bioluminescent jellyfish shader
//
// Creates a translucent, pulsing jellyfish material with
// internal glow and surface patterns.

#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct JellyfishData {
    base_color: vec4<f32>,
    glow_color: vec4<f32>,
    pulse_speed: f32,
    translucency: f32,
}

@group(3) @binding(0)
var<uniform> material: JellyfishData;

// Smooth noise
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    return mix(
        mix(hash(i), hash(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y
    );
}

fn fbm(p: vec2<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var pos = p;

    for (var i = 0; i < 4; i++) {
        value += amplitude * noise(pos);
        amplitude *= 0.5;
        pos *= 2.0;
    }

    return value;
}

// Radial pattern for jellyfish bell
fn radial_pattern(uv: vec2<f32>, segments: f32) -> f32 {
    let centered = uv - 0.5;
    let angle = atan2(centered.y, centered.x);
    let radius = length(centered);

    // Create radial segments
    let segment = sin(angle * segments) * 0.5 + 0.5;

    return segment * (1.0 - radius * 2.0);
}

// Vein-like patterns
fn veins(uv: vec2<f32>, time: f32) -> f32 {
    let centered = uv - 0.5;
    let angle = atan2(centered.y, centered.x);
    let radius = length(centered);

    // Branching veins from center
    let vein_angle = sin(angle * 8.0 + radius * 10.0 - time * 0.5);
    let vein = smoothstep(0.7, 1.0, vein_angle) * (1.0 - radius * 1.5);

    return max(0.0, vein);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = globals.time;

    // Pulsing effect
    let pulse = sin(time * material.pulse_speed) * 0.5 + 0.5;
    let fast_pulse = sin(time * material.pulse_speed * 3.0) * 0.5 + 0.5;

    // Distance from center
    let centered = uv - 0.5;
    let dist = length(centered);

    // Base bell shape (brighter in center, fading at edges)
    let bell_shape = 1.0 - smoothstep(0.0, 0.5, dist);

    // Radial pattern (like jellyfish segments)
    let radial = radial_pattern(uv, 12.0);

    // Internal vein pattern
    let vein_pattern = veins(uv, time);

    // Organic noise texture
    let organic = fbm(uv * 8.0 + time * 0.1);

    // Base translucent color
    var base = material.base_color.rgb;

    // Add radial variation
    base += radial * 0.15;

    // Internal glow that pulses
    let glow_intensity = bell_shape * (0.5 + pulse * 0.5);
    let glow = material.glow_color.rgb * glow_intensity;

    // Combine base and glow
    var final_color = mix(base, glow, 0.4 + pulse * 0.2);

    // Add vein highlights
    final_color += vein_pattern * material.glow_color.rgb * 0.5;

    // Add organic texture variation
    final_color *= 0.9 + organic * 0.2;

    // Edge glow effect (brighter at thin edges)
    let edge_glow = smoothstep(0.35, 0.5, dist) * (1.0 - smoothstep(0.5, 0.55, dist));
    final_color += edge_glow * material.glow_color.rgb * 0.5;

    // Bioluminescent flicker
    let flicker = noise(uv * 20.0 + time) * fast_pulse * 0.15;
    final_color += flicker * material.glow_color.rgb;

    // Calculate alpha for translucency
    let alpha = material.translucency * (0.4 + bell_shape * 0.5 + pulse * 0.1);

    return vec4<f32>(final_color, alpha);
}
