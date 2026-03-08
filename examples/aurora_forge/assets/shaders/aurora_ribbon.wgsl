#import bevy_pbr::{
    mesh_view_bindings::{globals, view},
    forward_io::VertexOutput,
}

struct AuroraRibbonMaterial {
    start_color: vec4<f32>,
    end_color: vec4<f32>,
    band_density: f32,
    flow_speed: f32,
    glow_strength: f32,
    alpha_bias: f32,
}

@group(3) @binding(0) var<uniform> material: AuroraRibbonMaterial;

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(91.7, 413.3))) * 24634.6345);
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

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let time = globals.time * material.flow_speed;
    let uv = in.uv;
    let warped = uv * vec2<f32>(1.0, material.band_density);

    let band_a = sin(warped.y + time * 7.0 + sin(warped.x * 4.0 + time));
    let band_b = sin(warped.y * 1.7 - time * 5.0 + cos(warped.x * 5.3 - time * 0.8));
    let drift = noise(vec2<f32>(uv.x * 3.0 + time * 0.4, uv.y * 5.0 - time * 0.25));

    let veil = smoothstep(0.0, 0.18, uv.x) * (1.0 - smoothstep(0.82, 1.0, uv.x));
    let crown = smoothstep(0.0, 0.1, uv.y) * (1.0 - smoothstep(0.9, 1.0, uv.y));
    let energy = clamp((band_a * 0.4 + band_b * 0.25 + drift * 0.6 + uv.y * 0.35) * 0.5 + 0.5, 0.0, 1.0);

    let view_dir = normalize(view.world_position.xyz - in.world_position.xyz);
    let fresnel = pow(1.0 - max(dot(view_dir, normalize(in.world_normal)), 0.0), 2.0);

    let color = mix(material.start_color.rgb, material.end_color.rgb, energy);
    let alpha = clamp((energy * material.alpha_bias + fresnel * 0.35) * veil * crown, 0.0, 1.0);

    return vec4<f32>(color * (0.55 + energy * material.glow_strength + fresnel * 0.45), alpha);
}
