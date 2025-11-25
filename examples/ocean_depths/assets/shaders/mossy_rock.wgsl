// Mossy underwater rock shader
//
// Creates a procedural rock texture with moss, barnacles,
// and underwater weathering effects.

#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct MossyRockData {
    rock_color: vec4<f32>,
    moss_color: vec4<f32>,
    moss_amount: f32,
    wetness: f32,
}

@group(3) @binding(0)
var<uniform> material: MossyRockData;

// Noise functions
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn hash3(p: vec3<f32>) -> f32 {
    return fract(sin(dot(p, vec3<f32>(127.1, 311.7, 74.7))) * 43758.5453);
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

    for (var i = 0; i < 6; i++) {
        value += amplitude * noise(pos);
        amplitude *= 0.5;
        pos *= 2.0;
    }

    return value;
}

// Voronoi for barnacle pattern
fn voronoi(p: vec2<f32>) -> vec3<f32> {
    let n = floor(p);
    let f = fract(p);

    var m_dist = 8.0;
    var m_point = vec2<f32>(0.0);
    var m_id = 0.0;

    for (var j = -1; j <= 1; j++) {
        for (var i = -1; i <= 1; i++) {
            let g = vec2<f32>(f32(i), f32(j));
            let o = vec2<f32>(hash(n + g), hash(n + g + 17.0));
            let r = g - f + o;
            let d = dot(r, r);
            if d < m_dist {
                m_dist = d;
                m_point = r;
                m_id = hash(n + g + 42.0);
            }
        }
    }

    return vec3<f32>(sqrt(m_dist), m_id, length(m_point));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = globals.time;

    // Base rock texture with multiple noise layers
    let rock_noise = fbm(uv * 8.0);
    let rock_detail = fbm(uv * 20.0) * 0.3;
    let rock_large = fbm(uv * 2.0) * 0.5;

    // Rock color variation
    var rock_col = material.rock_color.rgb;
    rock_col *= 0.7 + rock_noise * 0.3 + rock_detail;
    rock_col = mix(rock_col, rock_col * 0.6, rock_large);

    // Cracks in the rock
    let crack_noise = fbm(uv * 15.0);
    let cracks = smoothstep(0.45, 0.5, crack_noise) * 0.3;
    rock_col *= 1.0 - cracks;

    // Moss growth (more on top, using Y-like gradient from UV)
    let moss_gradient = smoothstep(0.3, 0.7, uv.y);
    let moss_noise = fbm(uv * 12.0 + time * 0.02);
    let moss_pattern = moss_gradient * moss_noise * material.moss_amount;

    // Blend moss onto rock
    let moss_col = material.moss_color.rgb * (0.8 + moss_noise * 0.2);
    var final_color = mix(rock_col, moss_col, smoothstep(0.3, 0.6, moss_pattern));

    // Barnacles (small circular bumps)
    let barnacle_vor = voronoi(uv * 25.0);
    let barnacle_ring = smoothstep(0.1, 0.15, barnacle_vor.x) * (1.0 - smoothstep(0.15, 0.2, barnacle_vor.x));
    let barnacle_center = 1.0 - smoothstep(0.0, 0.08, barnacle_vor.x);

    // Only add barnacles in certain areas (based on noise)
    let barnacle_area = step(0.5, fbm(uv * 4.0));
    let barnacles = (barnacle_ring * 0.5 + barnacle_center * 0.3) * barnacle_area;
    final_color = mix(final_color, vec3<f32>(0.75, 0.73, 0.7), barnacles);

    // Wetness effect - slight darkening and shine
    let wetness_factor = material.wetness;
    final_color *= 1.0 - wetness_factor * 0.2;

    // Add subtle wet highlights
    let wet_highlight = pow(max(0.0, noise(uv * 30.0 + time * 0.1)), 3.0) * wetness_factor;
    final_color += wet_highlight * 0.15;

    // Underwater color tinting
    final_color = mix(final_color, final_color * vec3<f32>(0.8, 0.9, 1.0), 0.1);

    return vec4<f32>(final_color, 1.0);
}
