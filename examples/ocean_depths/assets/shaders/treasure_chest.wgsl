// Treasure chest shader
//
// Creates weathered wood texture with barnacles,
// metal trim, and magical glow effect.

#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct TreasureChestData {
    wood_color: vec4<f32>,
    glow_color: vec4<f32>,
    weathering: f32,
    magic_intensity: f32,
}

@group(3) @binding(0)
var<uniform> material: TreasureChestData;

// Noise functions
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

    for (var i = 0; i < 5; i++) {
        value += amplitude * noise(pos);
        amplitude *= 0.5;
        pos *= 2.0;
    }

    return value;
}

// Wood grain pattern
fn wood_grain(p: vec2<f32>) -> f32 {
    let grain_freq = 30.0;
    let grain_noise = fbm(p * 2.0) * 5.0;
    let grain = sin((p.x + grain_noise) * grain_freq) * 0.5 + 0.5;

    // Add knots
    let knot_noise = fbm(p * 0.5 + 50.0);
    let knots = smoothstep(0.7, 0.75, knot_noise);

    return grain * (1.0 - knots * 0.5) + knots * 0.3;
}

// Barnacle/encrustation pattern
fn barnacles(p: vec2<f32>) -> f32 {
    let n = floor(p * 15.0);
    let f = fract(p * 15.0);

    var min_dist = 1.0;

    for (var j = -1; j <= 1; j++) {
        for (var i = -1; i <= 1; i++) {
            let g = vec2<f32>(f32(i), f32(j));
            let o = vec2<f32>(hash(n + g), hash(n + g + 17.0));
            let r = g - f + o;
            let d = length(r);
            min_dist = min(min_dist, d);
        }
    }

    // Create circular barnacles
    return smoothstep(0.15, 0.1, min_dist);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = globals.time;

    // Base wood texture
    let grain = wood_grain(uv);
    var wood = material.wood_color.rgb;
    wood *= 0.7 + grain * 0.3;

    // Weathering/aging effect
    let weather = fbm(uv * 8.0);
    let dark_spots = smoothstep(0.5, 0.7, weather) * material.weathering;
    wood *= 1.0 - dark_spots * 0.4;

    // Add some green/algae tint from underwater exposure
    let algae = fbm(uv * 12.0 + 20.0);
    let algae_mask = smoothstep(0.4, 0.6, algae) * material.weathering * 0.5;
    wood = mix(wood, vec3<f32>(0.2, 0.35, 0.25), algae_mask);

    // Barnacles
    let barnacle_pattern = barnacles(uv);
    let barnacle_area = fbm(uv * 3.0);
    let barnacle_mask = barnacle_pattern * step(0.4, barnacle_area) * material.weathering;
    wood = mix(wood, vec3<f32>(0.7, 0.68, 0.65), barnacle_mask);

    // Metal trim suggestion (edges)
    let edge_x = smoothstep(0.0, 0.05, uv.x) * smoothstep(1.0, 0.95, uv.x);
    let edge_y = smoothstep(0.0, 0.05, uv.y) * smoothstep(1.0, 0.95, uv.y);
    let is_trim = 1.0 - edge_x * edge_y;

    // Rusty metal color for trim
    let rust = fbm(uv * 20.0);
    let metal_color = mix(vec3<f32>(0.6, 0.5, 0.3), vec3<f32>(0.4, 0.25, 0.15), rust);
    wood = mix(wood, metal_color, is_trim * 0.7);

    // Magical glow emanating from within
    let glow_pulse = sin(time * 2.0) * 0.5 + 0.5;
    let glow_center = vec2<f32>(0.5, 0.3);
    let glow_dist = length(uv - glow_center);
    let glow = (1.0 - smoothstep(0.0, 0.4, glow_dist)) * material.magic_intensity;

    // Glow pulses and has noise
    let glow_noise = fbm(uv * 5.0 + time * 0.5);
    let final_glow = glow * (0.6 + glow_pulse * 0.4) * (0.8 + glow_noise * 0.4);

    var final_color = wood;
    final_color += final_glow * material.glow_color.rgb;

    // Add sparkles
    let sparkle_noise = noise(uv * 50.0 + time * 2.0);
    let sparkles = smoothstep(0.97, 1.0, sparkle_noise) * material.magic_intensity;
    final_color += sparkles * material.glow_color.rgb * 2.0;

    return vec4<f32>(final_color, 1.0);
}
