// Underwater caustics shader
//
// Simulates the dancing light patterns seen underwater when sunlight
// refracts through the water surface.

#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct CausticsData {
    color: vec4<f32>,
    speed: f32,
}

@group(3) @binding(0)
var<uniform> material: CausticsData;

// Simple noise function for caustics pattern
fn hash(p: vec2<f32>) -> f32 {
    let h = dot(p, vec2<f32>(127.1, 311.7));
    return fract(sin(h) * 43758.5453123);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    return mix(
        mix(hash(i + vec2<f32>(0.0, 0.0)), hash(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y
    );
}

// Fractal Brownian Motion for organic patterns
fn fbm(p: vec2<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 1.0;
    var pos = p;

    for (var i = 0; i < 5; i++) {
        value += amplitude * noise(pos * frequency);
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    return value;
}

// Voronoi-like pattern for caustic cells
fn caustic_pattern(p: vec2<f32>, time: f32) -> f32 {
    let animated_p = p + vec2<f32>(time * 0.1, time * 0.15);

    // Multiple layers of noise at different scales
    let n1 = fbm(animated_p * 3.0);
    let n2 = fbm(animated_p * 5.0 + vec2<f32>(time * 0.2, 0.0));
    let n3 = fbm(animated_p * 8.0 - vec2<f32>(0.0, time * 0.15));

    // Combine and create sharp caustic edges
    let combined = n1 * 0.5 + n2 * 0.3 + n3 * 0.2;
    let caustic = pow(combined, 3.0) * 4.0;

    return clamp(caustic, 0.0, 1.0);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let time = globals.time * material.speed;

    // Use world position for consistent pattern
    let uv = in.uv * 10.0;

    // Generate caustics pattern
    let caustics = caustic_pattern(uv, time);

    // Color with caustics intensity - bright additive-style effect
    let base_color = material.color.rgb;
    let final_color = base_color * caustics * 1.5;

    // Very low base alpha, only show the bright caustic patterns
    // This creates a light-adding effect rather than blocking light
    let alpha = caustics * 0.35;

    return vec4<f32>(final_color, alpha);
}
