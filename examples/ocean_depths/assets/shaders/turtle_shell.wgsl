// Turtle shell shader
//
// Creates a procedural turtle shell pattern with hexagonal scutes,
// age rings, and subtle iridescence.

#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct TurtleShellData {
    base_color: vec4<f32>,
    accent_color: vec4<f32>,
    age: f32,
    roughness: f32,
}

@group(3) @binding(0)
var<uniform> material: TurtleShellData;

// Hash function for noise
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

// Hexagonal distance function for scute pattern
fn hex_dist(p: vec2<f32>) -> f32 {
    let q = abs(p);
    return max(q.x * 0.866025 + q.y * 0.5, q.y);
}

// Get hexagonal cell coordinates
fn hex_coords(p: vec2<f32>) -> vec2<f32> {
    let r = vec2<f32>(1.0, 1.732);
    let h = r * 0.5;

    let a = p / r - floor(p / r + 0.5);
    let b = (p - h) / r - floor((p - h) / r + 0.5);

    if length(a) < length(b) {
        return a;
    }
    return b;
}

// Voronoi for scute texture
fn voronoi(p: vec2<f32>) -> vec2<f32> {
    let n = floor(p);
    let f = fract(p);

    var m = vec2<f32>(8.0, 0.0);

    for (var j = -1; j <= 1; j++) {
        for (var i = -1; i <= 1; i++) {
            let g = vec2<f32>(f32(i), f32(j));
            let o = vec2<f32>(hash(n + g), hash(n + g + 17.0));
            let r = g - f + o;
            let d = dot(r, r);
            if d < m.x {
                m = vec2<f32>(d, hash(n + g + 42.0));
            }
        }
    }

    return m;
}

// Age ring pattern
fn age_rings(p: vec2<f32>, age: f32) -> f32 {
    let dist = length(p);
    let rings = sin(dist * 15.0 * age) * 0.5 + 0.5;
    return rings * smoothstep(0.0, 0.3, 1.0 - dist);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = globals.time;

    // Scale UV for hexagonal pattern
    let hex_uv = uv * 6.0;
    let hex = hex_coords(hex_uv);
    let hex_edge = smoothstep(0.4, 0.45, hex_dist(hex));

    // Create scute centers with voronoi
    let vor = voronoi(uv * 5.0);
    let scute_center = vor.y;

    // Age rings within each scute
    let ring_pattern = age_rings(hex, material.age);

    // Combine patterns
    let pattern = hex_edge * 0.3 + ring_pattern * 0.4 + scute_center * 0.3;

    // Base colors
    let base = material.base_color.rgb;
    let accent = material.accent_color.rgb;

    // Mix colors based on pattern
    var shell_color = mix(base, accent, pattern);

    // Add subtle iridescence
    let iridescence = sin(uv.x * 20.0 + uv.y * 15.0 + time * 0.5) * 0.05;
    shell_color += vec3<f32>(iridescence, iridescence * 0.8, iridescence * 0.5);

    // Darken the hex edges to show scute boundaries
    shell_color *= 0.7 + hex_edge * 0.3;

    // Add slight specular highlight simulation
    let highlight = pow(max(0.0, dot(normalize(vec3<f32>(uv.x - 0.5, uv.y - 0.5, 1.0)), vec3<f32>(0.3, 0.3, 1.0))), 8.0);
    shell_color += highlight * 0.15;

    return vec4<f32>(shell_color, 1.0);
}
