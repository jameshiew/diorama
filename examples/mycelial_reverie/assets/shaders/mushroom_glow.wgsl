// Bioluminescent mushroom cap shader.
//
// Pulsing, vein-patterned emissive surface with a fresnel edge halo.
// The glow is driven by:
// - FBM noise veins that slowly drift across the cap
// - Outward-travelling ring pulses emitted from the cap's center
// - Fresnel rim lighting for a haloed silhouette

#import bevy_pbr::{
    mesh_view_bindings::{globals, view},
    forward_io::VertexOutput,
}

struct MushroomGlowMaterial {
    base_color: vec4<f32>,
    glow_color: vec4<f32>,
    pulse_speed: f32,
    vein_scale: f32,
    fresnel_power: f32,
    phase_offset: f32,
}

@group(3) @binding(0) var<uniform> material: MushroomGlowMaterial;

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453123);
}

fn value_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    let u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

fn fbm(p: vec2<f32>) -> f32 {
    var total = 0.0;
    var amp = 0.5;
    var freq = 1.0;
    for (var i = 0; i < 4; i = i + 1) {
        total = total + value_noise(p * freq) * amp;
        amp = amp * 0.5;
        freq = freq * 2.0;
    }
    return total;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let time = globals.time * material.pulse_speed + material.phase_offset;

    let uv = in.uv * material.vein_scale;
    let radial = length(in.uv - vec2<f32>(0.5)) * 2.0;

    // Slowly drifting organic veins
    let drift = vec2<f32>(time * 0.12, -time * 0.08);
    let vein_noise = fbm(uv + drift);
    let vein_mask = smoothstep(0.42, 0.7, vein_noise);

    // Secondary fine-scale shimmer
    let shimmer = fbm(uv * 2.4 + vec2<f32>(-time * 0.2, time * 0.15));

    // Concentric outward pulse rings emitted from the cap's center
    let ring_wave = sin(radial * 7.0 - time * 2.6) * 0.5 + 0.5;
    let ring_pulse = pow(ring_wave, 4.0);

    // Soft breathing brightness applied over the whole cap
    let breathe = 0.55 + 0.45 * (sin(time * 0.9) * 0.5 + 0.5);

    // Fresnel rim glow
    let view_dir = normalize(view.world_position.xyz - in.world_position.xyz);
    let normal = normalize(in.world_normal);
    let fresnel = pow(1.0 - max(dot(view_dir, normal), 0.0), material.fresnel_power);

    let emission = clamp(
        vein_mask * 0.65 + shimmer * 0.15 + ring_pulse * 0.55 + fresnel * 0.45,
        0.0,
        1.3,
    ) * breathe;

    let color = mix(material.base_color.rgb, material.glow_color.rgb, clamp(emission, 0.0, 1.0));
    let brightness = 0.55 + emission * 1.6 + fresnel * 0.3;

    return vec4<f32>(color * brightness, material.base_color.a);
}
