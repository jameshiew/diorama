#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

// "The sculpture exists in superposition - simultaneously crystalline and liquid,
// geometric and organic, order and chaos intertwined in eternal dance."

struct MorphingSculptureMaterial {
    base_color: vec4<f32>,
    secondary_color: vec4<f32>,
    morph_speed: f32,
    morph_intensity: f32,
    detail_scale: f32,
    glow_strength: f32,
}

@group(3) @binding(0) var<uniform> material: MorphingSculptureMaterial;

// Constants for esoteric mathematical beauty
const PHI: f32 = 1.618033988749895; // Golden ratio - divine proportion
const PI: f32 = 3.141592653589793;
const TAU: f32 = 6.283185307179586;
const EULER: f32 = 2.718281828459045;

// Hash function for pseudo-random values
fn hash(p: vec2<f32>) -> f32 {
    let h = dot(p, vec2<f32>(127.1, 311.7));
    return fract(sin(h) * 43758.5453);
}

// 3D hash for more complex patterns
fn hash3d(p: vec3<f32>) -> f32 {
    let h = dot(p, vec3<f32>(127.1, 311.7, 74.7));
    return fract(sin(h) * 43758.5453);
}

// Smooth 2D noise
fn noise2d(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);

    // Cubic smoothstep for better interpolation
    let u = f * f * (3.0 - 2.0 * f);

    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));

    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

// 3D noise for volumetric effects
fn noise3d(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);

    let u = f * f * (3.0 - 2.0 * f);

    let a = hash3d(i);
    let b = hash3d(i + vec3<f32>(1.0, 0.0, 0.0));
    let c = hash3d(i + vec3<f32>(0.0, 1.0, 0.0));
    let d = hash3d(i + vec3<f32>(1.0, 1.0, 0.0));
    let e = hash3d(i + vec3<f32>(0.0, 0.0, 1.0));
    let f1 = hash3d(i + vec3<f32>(1.0, 0.0, 1.0));
    let g = hash3d(i + vec3<f32>(0.0, 1.0, 1.0));
    let h = hash3d(i + vec3<f32>(1.0, 1.0, 1.0));

    let layer1 = mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
    let layer2 = mix(mix(e, f1, u.x), mix(g, h, u.x), u.y);

    return mix(layer1, layer2, u.z);
}

// Fractal Brownian Motion for complex detail
fn fbm(p: vec3<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 1.0;
    var pos = p;

    for (var i = 0; i < 6; i++) {
        value += amplitude * noise3d(pos * frequency);
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    return value;
}

// Voronoi-like cellular pattern for organic detail
fn voronoi(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);

    var min_dist = 1.0;

    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let neighbor = vec2<f32>(f32(x), f32(y));
            let point = vec2<f32>(
                hash(i + neighbor),
                hash(i + neighbor + vec2<f32>(23.45, 67.89))
            );
            let diff = neighbor + point - f;
            let dist = length(diff);
            min_dist = min(min_dist, dist);
        }
    }

    return min_dist;
}

// Smooth pulse function for rhythmic animations
fn pulse(t: f32, frequency: f32) -> f32 {
    return 0.5 + 0.5 * sin(t * frequency);
}

// Twist distortion for shape morphing
fn twist(p: vec3<f32>, amount: f32) -> vec3<f32> {
    let angle = p.y * amount;
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(
        p.x * c - p.z * s,
        p.y,
        p.x * s + p.z * c
    );
}

// Spherical distortion for organic bulging
fn spherical_distort(p: vec3<f32>, center: vec3<f32>, radius: f32, strength: f32) -> vec3<f32> {
    let diff = p - center;
    let dist = length(diff);
    let influence = max(0.0, 1.0 - dist / radius);
    let distortion = diff * influence * strength;
    return p + distortion;
}

// Mandelbrot-like recursive complexity at a point
fn mandelbrot_field(p: vec2<f32>, iterations: i32) -> f32 {
    var z = vec2<f32>(0.0, 0.0);
    var c = p * 0.5;
    var escape_time = 0.0;

    for (var i = 0; i < iterations; i++) {
        if (length(z) > 2.0) {
            break;
        }
        // z = z^2 + c
        z = vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        escape_time += 1.0;
    }

    return escape_time / f32(iterations);
}

// Hyperbolic geometry transformation - non-Euclidean space
fn hyperbolic_transform(p: vec3<f32>, time: f32) -> vec3<f32> {
    let r = length(p);
    let theta = atan2(p.z, p.x);
    let phi = acos(p.y / max(r, 0.001));

    // Apply hyperbolic distortion with time variation
    let new_r = sinh(r * (1.0 + 0.3 * sin(time * 0.5))) / cosh(r * 0.5);
    let new_theta = theta + time * 0.3 + sin(phi * 5.0) * 0.5;
    let new_phi = phi + cos(theta * 3.0 + time * 0.4) * 0.3;

    return vec3<f32>(
        new_r * sin(new_phi) * cos(new_theta),
        new_r * cos(new_phi),
        new_r * sin(new_phi) * sin(new_theta)
    );
}

// Quantum foam - chaotic micro-structure
fn quantum_foam(p: vec3<f32>, time: f32) -> f32 {
    var foam = 0.0;
    var amplitude = 1.0;
    var frequency = 1.0;
    var pos = p;

    for (var i = 0; i < 8; i++) {
        let n = noise3d(pos * frequency + time * 0.5);
        foam += n * amplitude;

        // Rotate in 3D space for complexity
        pos = vec3<f32>(
            pos.x * cos(0.7) - pos.z * sin(0.7),
            pos.y,
            pos.x * sin(0.7) + pos.z * cos(0.7)
        );

        frequency *= PHI; // Golden ratio scaling
        amplitude *= 0.5;
    }

    return foam;
}

// DNA helix-like structure
fn double_helix(p: vec3<f32>, time: f32) -> f32 {
    let angle1 = p.y * 3.0 + time;
    let angle2 = p.y * 3.0 + time + PI;

    let helix1 = length(vec2<f32>(
        p.x - cos(angle1) * 0.3,
        p.z - sin(angle1) * 0.3
    ));

    let helix2 = length(vec2<f32>(
        p.x - cos(angle2) * 0.3,
        p.z - sin(angle2) * 0.3
    ));

    let connecting_bars = abs(sin(p.y * 12.0 + time * 2.0)) * 0.1;

    return min(min(helix1, helix2), connecting_bars);
}

// Sierpinski-like fractal dimension
fn fractal_dimension(p: vec3<f32>, time: f32) -> f32 {
    var scale = 1.0;
    var pos = p;
    var dimension = 0.0;

    for (var i = 0; i < 6; i++) {
        pos = abs(pos);

        // Manual component swapping (WGSL doesn't support arbitrary swizzle assignment)
        if (pos.x < pos.y) {
            let temp = pos.x;
            pos.x = pos.y;
            pos.y = temp;
        }
        if (pos.x < pos.z) {
            let temp = pos.x;
            pos.x = pos.z;
            pos.z = temp;
        }
        if (pos.y < pos.z) {
            let temp = pos.y;
            pos.y = pos.z;
            pos.z = temp;
        }

        pos = pos * 2.0 - vec3<f32>(1.0);
        pos.x -= 1.0;

        dimension += noise3d(pos * scale + time * 0.1) / scale;
        scale *= 2.0;
    }

    return dimension;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = globals.time * material.morph_speed;

    // Create multidimensional position coordinates
    let pos3d = vec3<f32>(
        uv.x * material.detail_scale,
        uv.y * material.detail_scale,
        time * 0.2
    );

    // === LAYER 1: Hyperbolic Non-Euclidean Geometry ===
    var hyperbolic_pos = hyperbolic_transform(pos3d, time);

    // === LAYER 2: Multi-axis Twisted Dimensions ===
    let twist_amount = sin(time * 0.3) * TAU;
    var twisted_pos = twist(hyperbolic_pos, twist_amount);
    twisted_pos = twist(twisted_pos.yzx, twist_amount * 0.7).zxy; // Triple-axis twist

    // === LAYER 3: Organic Pulsing Distortions (breathing geometry) ===
    let breath_cycle = sin(time * 0.4) * 0.5 + 0.5;
    for (var i = 0; i < 5; i++) {
        let center = vec3<f32>(
            sin(time * (0.5 + f32(i) * 0.15) + f32(i) * TAU / 5.0) * 0.4,
            cos(time * (0.6 + f32(i) * 0.13) + f32(i) * TAU / 5.0) * 0.4,
            sin(time * (0.7 + f32(i) * 0.17) + f32(i) * TAU / 5.0) * 0.4
        );
        twisted_pos = spherical_distort(twisted_pos, center, 1.5, material.morph_intensity * (0.2 + breath_cycle * 0.3));
    }

    // === LAYER 4: Quantum Foam Micro-Structure ===
    let foam = quantum_foam(twisted_pos * 3.0, time);
    let foam_contribution = foam * 0.15;

    // === LAYER 5: DNA-like Helix Structures ===
    let helix = double_helix(twisted_pos * 2.0, time * 2.0);
    let helix_glow = exp(-helix * 8.0) * 0.6;

    // === LAYER 6: Fractal Dimension Complexity ===
    let fractal = fractal_dimension(twisted_pos * 1.5, time);

    // === LAYER 7: Mandelbrot Field Influence ===
    let mandel = mandelbrot_field(twisted_pos.xy * 2.0 + time * 0.1, 20);

    // === LAYER 8: Multi-Octave Surface Patterns ===
    let detail_ultra = fbm(twisted_pos * 4.0 + vec3<f32>(time * 0.15));
    let detail_high = fbm(twisted_pos * 8.0 + vec3<f32>(time * 0.25));
    let detail_med = voronoi(twisted_pos.xy * 12.0 + time * 0.3);
    let detail_low = noise3d(twisted_pos * 16.0 + time * 0.35);

    let combined_detail = detail_ultra * 0.35 + detail_high * 0.25 + detail_med * 0.25 + detail_low * 0.15;

    // === LAYER 9: Traveling Wave Interference Patterns ===
    let wave1 = sin(twisted_pos.x * 15.0 + time * 3.0) * cos(twisted_pos.y * 17.0 + time * 2.7);
    let wave2 = sin(twisted_pos.y * 19.0 + time * 2.3) * cos(twisted_pos.z * 21.0 + time * 3.2);
    let wave3 = sin(twisted_pos.z * 23.0 + time * 2.8) * cos(twisted_pos.x * 13.0 + time * 3.5);
    let interference = (wave1 + wave2 + wave3) / 3.0;

    // === LAYER 10: Energy Ley Lines (sacred geometry) ===
    let ley_line1 = abs(sin(twisted_pos.x * 20.0 + twisted_pos.y * 15.0 + time * 4.0));
    let ley_line2 = abs(sin(twisted_pos.y * 25.0 + twisted_pos.z * 18.0 + time * 3.5));
    let ley_line3 = abs(sin(twisted_pos.z * 22.0 + twisted_pos.x * 19.0 + time * 4.2));
    let ley_lines = pow(min(min(ley_line1, ley_line2), ley_line3), 4.0);

    // === COLOR SYNTHESIS ===
    // Base color transitions through time and space
    let color_phase = time * 0.2 + combined_detail * 2.0 + mandel * 3.0;
    let hue_shift = fract(color_phase);

    // Multi-dimensional color mixing
    var base_color = mix(
        material.base_color.rgb,
        material.secondary_color.rgb,
        hue_shift
    );

    // Apply fractal complexity to color
    base_color = base_color * (0.5 + combined_detail * 0.5 + fractal * 0.3);

    // Interference pattern modulation
    base_color = base_color * (0.7 + interference * 0.3);

    // === IRIDESCENT QUANTUM SHIMMER ===
    let shimmer_complexity = sin(twisted_pos.x * 30.0 + time * 5.0) *
                            cos(twisted_pos.y * 35.0 + time * 4.5) *
                            sin(twisted_pos.z * 40.0 + time * 5.5) *
                            foam;

    // Spectral decomposition
    let spectral = vec3<f32>(
        0.5 + 0.5 * sin(shimmer_complexity * TAU + 0.0),
        0.5 + 0.5 * sin(shimmer_complexity * TAU + TAU / 3.0),
        0.5 + 0.5 * sin(shimmer_complexity * TAU + 2.0 * TAU / 3.0)
    );

    base_color = base_color + spectral * 0.3 + foam_contribution;

    // === ENERGY EMISSIONS ===
    // Ley line glow
    let ley_glow_color = vec3<f32>(
        0.9 + 0.1 * sin(time * 2.0),
        0.7 + 0.3 * cos(time * 1.7),
        0.3 + 0.7 * sin(time * 2.3)
    );
    base_color = base_color + ley_glow_color * ley_lines * material.glow_strength;

    // Helix structure glow
    base_color = base_color + vec3<f32>(0.3, 0.8, 1.0) * helix_glow * material.glow_strength;

    // Mandelbrot field emission
    base_color = base_color + vec3<f32>(1.0, 0.5, 0.8) * mandel * 0.3;

    // === BREATHING LUMINESCENCE ===
    let cosmic_breath = sin(time * 0.5) * 0.5 + 0.5;
    let subsurface_glow = pow(cosmic_breath, 2.0);
    base_color = base_color * (0.7 + subsurface_glow * 0.3);

    // === DIMENSIONAL DEPTH ===
    let center_dist = length(uv - 0.5);
    let rim_power = pow(center_dist * 2.0, 1.5);

    // Chromatic aberration at dimensional boundaries
    let chromatic = vec3<f32>(
        1.0 + rim_power * 0.15 + foam * 0.1,
        1.0 + foam * 0.05,
        1.0 - rim_power * 0.15 - foam * 0.1
    );
    base_color = base_color * chromatic;

    // Rim lighting enhancement
    base_color = base_color * (1.0 + rim_power * 0.4);

    // === FINAL HARMONIC RESONANCE ===
    let resonance = sin(time * PHI) * 0.15 + 0.85;
    base_color = base_color * resonance;

    return vec4<f32>(base_color, material.base_color.a);
}
