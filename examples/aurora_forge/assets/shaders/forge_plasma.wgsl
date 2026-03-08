#import bevy_pbr::{
    mesh_view_bindings::{globals, view},
    forward_io::VertexOutput,
}

struct ForgePlasmaMaterial {
    base_color: vec4<f32>,
    hot_color: vec4<f32>,
    swirl_scale: f32,
    pulse_speed: f32,
    fresnel_power: f32,
    _padding: u32,
}

@group(3) @binding(0) var<uniform> material: ForgePlasmaMaterial;

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453123);
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
    let time = globals.time * material.pulse_speed;
    let pos = in.world_position.xz * material.swirl_scale;

    let swirl = sin(pos.x * 2.4 + time + sin(pos.y * 2.8 - time * 0.7)) * 0.5 + 0.5;
    let filaments = abs(sin((pos.x + pos.y) * 3.6 - time * 1.6));
    let turbulence = noise(pos * 3.2 + vec2<f32>(time * 0.3, -time * 0.2));
    let plume = sin(in.world_position.y * 2.2 - time * 3.2) * 0.5 + 0.5;

    let view_dir = normalize(view.world_position.xyz - in.world_position.xyz);
    let normal = normalize(in.world_normal);
    let fresnel = pow(max(1.0 - max(dot(view_dir, normal), 0.0), 0.0), material.fresnel_power);

    let energy = clamp(swirl * 0.35 + filaments * 0.25 + turbulence * 0.55 + plume * 0.35, 0.0, 1.35);
    let glow = smoothstep(0.2, 1.1, energy);
    let heat = clamp(glow + fresnel * 0.45, 0.0, 1.0);

    let color = mix(material.base_color.rgb, material.hot_color.rgb, heat);
    let alpha = clamp(material.base_color.a + glow * 0.45 + fresnel * 0.35, 0.0, 1.0);

    return vec4<f32>(color * (0.7 + glow * 0.8 + fresnel * 0.4), alpha);
}
