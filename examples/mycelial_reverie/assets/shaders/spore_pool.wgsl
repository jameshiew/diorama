// Spore pool water surface shader.
//
// Stacks several effects to create a glowing, slightly viscous pool:
// - Concentric ripple rings that emanate from the pool's center
// - Domain-warped FBM caustics that drift over the surface
// - A handful of slow-drifting bright mote highlights
// - A depth gradient from the shallow rim to the deep center
// - Fresnel edge glow for a haloed silhouette when viewed obliquely

#import bevy_pbr::{
    mesh_view_bindings::{globals, view},
    forward_io::VertexOutput,
}

struct SporePoolMaterial {
    shallow_color: vec4<f32>,
    deep_color: vec4<f32>,
    mote_color: vec4<f32>,
    ripple_scale: f32,
    flow_speed: f32,
    glow_strength: f32,
    _padding: f32,
}

@group(3) @binding(0) var<uniform> material: SporePoolMaterial;

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(91.7, 413.3))) * 24634.6345);
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
    let time = globals.time * material.flow_speed;

    let centered = in.uv - vec2<f32>(0.5);
    let radial = length(centered) * 2.0;

    // Concentric rippling rings expanding outward
    let ripple_a = sin(radial * material.ripple_scale - time * 2.2);
    let ripple_b = sin(radial * material.ripple_scale * 0.55 - time * 1.1);
    let ripple = (ripple_a * 0.55 + ripple_b * 0.35) * 0.5 + 0.5;

    // Domain-warped caustics
    let warp = vec2<f32>(
        fbm(in.uv * 2.0 + vec2<f32>(time * 0.15, 0.0)),
        fbm(in.uv * 2.0 + vec2<f32>(0.0, -time * 0.12)),
    );
    let caustic_raw = fbm(in.uv * 6.0 + warp * 1.4 + vec2<f32>(time * 0.2, -time * 0.13));
    let caustic = pow(clamp(caustic_raw, 0.0, 1.0), 2.2) * 1.8;

    // Slowly drifting spore motes
    var motes = 0.0;
    for (var i = 0; i < 5; i = i + 1) {
        let fi = f32(i);
        let offset = vec2<f32>(
            sin(time * 0.35 + fi * 1.7) * 0.22,
            cos(time * 0.41 + fi * 2.1) * 0.22,
        );
        let mote_center = vec2<f32>(0.5) + offset;
        let d = length(in.uv - mote_center);
        let sparkle = 0.6 + 0.4 * sin(time * 1.8 + fi * 3.0);
        motes = motes + smoothstep(0.1, 0.0, d) * sparkle * 0.6;
    }

    // Depth gradient - deeper glow toward the pool's center
    let depth = smoothstep(1.0, 0.0, radial);
    let base = mix(material.shallow_color.rgb, material.deep_color.rgb, depth);

    // Fresnel edge glow
    let view_dir = normalize(view.world_position.xyz - in.world_position.xyz);
    let normal = normalize(in.world_normal);
    let fresnel = pow(1.0 - max(dot(view_dir, normal), 0.0), 2.6);

    let glow = caustic * material.glow_strength + ripple * 0.35 + motes;
    let color = base + material.mote_color.rgb * glow;
    let brightness = 0.45 + glow * 0.85 + fresnel * 0.35;
    let alpha = clamp(material.shallow_color.a + fresnel * 0.45 + motes * 0.35, 0.0, 1.0);

    return vec4<f32>(color * brightness, alpha);
}
