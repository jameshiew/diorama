// Coral shader
//
// Creates organic coral surface texture with polyp patterns,
// subtle bioluminescence, and color gradients.

#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct CoralData {
    base_color: vec4<f32>,
    tip_color: vec4<f32>,
    glow_intensity: f32,
    polyp_density: f32,
}

@group(3) @binding(0)
var<uniform> material: CoralData;

// Hash and noise functions
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

// Voronoi for polyp pattern
fn voronoi(p: vec2<f32>) -> vec3<f32> {
    let n = floor(p);
    let f = fract(p);

    var m_dist = 8.0;
    var m_dist2 = 8.0;
    var m_id = 0.0;

    for (var j = -1; j <= 1; j++) {
        for (var i = -1; i <= 1; i++) {
            let g = vec2<f32>(f32(i), f32(j));
            let o = vec2<f32>(hash(n + g), hash(n + g + 17.0)) * 0.8 + 0.1;
            let r = g - f + o;
            let d = dot(r, r);

            if d < m_dist {
                m_dist2 = m_dist;
                m_dist = d;
                m_id = hash(n + g + 42.0);
            } else if d < m_dist2 {
                m_dist2 = d;
            }
        }
    }

    return vec3<f32>(sqrt(m_dist), sqrt(m_dist2) - sqrt(m_dist), m_id);
}

// Polyp tentacle pattern
fn polyp_tentacles(p: vec2<f32>, time: f32) -> f32 {
    let vor = voronoi(p);
    let cell_center = vor.x;

    // Create circular polyp opening
    let polyp_opening = smoothstep(0.15, 0.1, cell_center);

    // Animated tentacle movement
    let angle = atan2(fract(p).y - 0.5, fract(p).x - 0.5);
    let tentacle_wave = sin(angle * 6.0 + time * 2.0) * 0.5 + 0.5;

    return polyp_opening * (0.5 + tentacle_wave * 0.5);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = globals.time;

    // Gradient from base to tip (using V coordinate as height)
    let height_gradient = uv.y;

    // Mix base and tip colors based on height
    var coral_color = mix(material.base_color.rgb, material.tip_color.rgb, height_gradient);

    // Organic surface texture
    let surface_noise = fbm(uv * 10.0);
    coral_color *= 0.85 + surface_noise * 0.3;

    // Polyp pattern
    let polyp_uv = uv * material.polyp_density;
    let polyps = polyp_tentacles(polyp_uv, time);
    let vor = voronoi(polyp_uv);

    // Polyp cells create subtle bumps
    let cell_edge = smoothstep(0.1, 0.2, vor.y);
    coral_color *= 0.9 + cell_edge * 0.2;

    // Add polyp coloring (slightly different hue in openings)
    let polyp_color = material.tip_color.rgb * 1.2;
    coral_color = mix(coral_color, polyp_color, polyps * 0.4);

    // Bioluminescent glow in polyps
    let glow_pulse = sin(time * 1.5 + vor.z * 6.28) * 0.5 + 0.5;
    let bio_glow = polyps * glow_pulse * material.glow_intensity;
    coral_color += bio_glow * material.tip_color.rgb;

    // Add subtle color variation across surface
    let color_var = fbm(uv * 3.0 + 100.0);
    coral_color *= 0.95 + color_var * 0.1;

    // Underwater color cast
    coral_color = mix(coral_color, coral_color * vec3<f32>(0.9, 0.95, 1.0), 0.1);

    return vec4<f32>(coral_color, 1.0);
}
