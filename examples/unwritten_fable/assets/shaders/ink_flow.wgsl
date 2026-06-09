// Flowing ink shader for the spine river, the inkfall, and pooled ink.
//
// - Domain-warped FBM streaks stretched along the flow direction
// - Sparse golden glints that twinkle and drift with the current
// - Fresnel sheen for a wet, viscous silhouette
// - fall_mode switches between horizontal flow (world xz, along z) and a
//   vertical fall (world xy, along -y) with feathered top/bottom edges
//
// World-space sampling keeps the current coherent across mesh seams.

#import bevy_pbr::{
    mesh_view_bindings::{globals, view},
    forward_io::VertexOutput,
}

struct InkFlowMaterial {
    base_color: vec4<f32>,
    glint_color: vec4<f32>,
    flow_speed: f32,
    streak_scale: f32,
    fall_mode: f32,
    phase: f32,
}

@group(3) @binding(0) var<uniform> material: InkFlowMaterial;

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
    let time = globals.time * material.flow_speed + material.phase;
    let falling = material.fall_mode > 0.5;

    // Flow coordinates: cross-current on x, downstream on y
    var p: vec2<f32>;
    if falling {
        p = vec2<f32>(
            in.world_position.x * material.streak_scale,
            in.world_position.y * material.streak_scale * 0.3 + time,
        );
    } else {
        p = vec2<f32>(
            in.world_position.x * material.streak_scale,
            in.world_position.z * material.streak_scale * 0.3 - time,
        );
    }

    // Streaks elongated along the current, warped so they wander
    let warp = vec2<f32>(
        fbm(p * 1.6 + vec2<f32>(0.0, time * 0.2)),
        fbm(p * 1.6 + vec2<f32>(4.7, -time * 0.15)),
    );
    let streaks = fbm(p + warp * 0.9);
    let current = pow(clamp(streaks, 0.0, 1.0), 1.6);

    // Sparse golden glints riding the surface - flecks of gold leaf in the ink
    let cell = floor(p * 3.0);
    let glint_seed = hash(cell);
    let glint_pos = vec2<f32>(hash(cell + 11.3), hash(cell + 47.9));
    let glint_dist = length(fract(p * 3.0) - glint_pos);
    let twinkle = 0.5 + 0.5 * sin(globals.time * 2.6 + glint_seed * 31.0);
    let glint = step(0.90, glint_seed) * smoothstep(0.18, 0.0, glint_dist) * twinkle;

    // Wet fresnel sheen; clamped so float error in the dot product cannot
    // push the pow() base negative.
    let view_dir = normalize(view.world_position.xyz - in.world_position.xyz);
    let normal = normalize(in.world_normal);
    let fresnel = pow(clamp(1.0 - dot(view_dir, normal), 0.0, 1.0), 3.0);

    var color = material.base_color.rgb * (0.55 + current * 0.9);
    color = color + material.glint_color.rgb * (glint * 2.2 + current * 0.12);
    color = color + material.base_color.rgb * fresnel * 2.4 + vec3<f32>(fresnel * 0.08);

    var alpha = material.base_color.a;
    if falling {
        alpha = alpha
            * smoothstep(0.0, 0.12, in.uv.y)
            * (1.0 - smoothstep(0.85, 1.0, in.uv.y));
    }
    alpha = clamp(alpha + glint * 0.3, 0.0, 1.0);

    return vec4<f32>(color, alpha);
}
