// Story page shader - the parchment surface of the open book.
//
// Layers, from bottom up:
// - Warm parchment with FBM paper grain and a soft gutter shadow
// - Procedural lines of cursive script (wavy strokes broken into words)
// - Settled ink to the west, fresh gold-glowing words near the frontier
// - A shimmering, noise-wobbled frontier band where writing is happening
// - Blank sketch-paper beyond the frontier with faint ruled ghost lines
//
// All sampling is in world space so the script stays put as the mesh
// undulates, and text runs along x like lines on a real page.

#import bevy_pbr::{
    mesh_view_bindings::{globals, view},
    forward_io::VertexOutput,
}

struct StoryPageMaterial {
    parchment_color: vec4<f32>,
    ink_color: vec4<f32>,
    glow_color: vec4<f32>,
    sketch_color: vec4<f32>,
    frontier_x: f32,
    band_width: f32,
    line_spacing: f32,
    script_scale: f32,
}

@group(3) @binding(0) var<uniform> material: StoryPageMaterial;

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

// Cursive script mask: rows of wavy strokes along x, broken into words,
// with letter-weight modulation so the strokes read as handwriting.
fn script_mask(p: vec2<f32>) -> f32 {
    let spacing = material.line_spacing;
    let line_id = floor(p.y / spacing);
    let line_seed = hash(vec2<f32>(line_id, 7.3));
    let baseline = (line_id + 0.5) * spacing;

    let wave = sin(p.x * material.script_scale + line_seed * 6.28) * 0.09 * spacing
        + sin(p.x * material.script_scale * 3.1 + line_seed * 12.0) * 0.045 * spacing;
    let dist = abs(p.y - (baseline + wave));
    let stroke = 1.0 - smoothstep(0.025 * spacing, 0.13 * spacing, dist);

    // Word gaps - some cells along the line are left blank
    let word_cell = floor(p.x / (spacing * 1.1));
    let word = step(0.22, hash(vec2<f32>(word_cell, line_id * 3.1)));

    // Letter-weight modulation within words
    let letters = 0.62 + 0.38 * sin(p.x * material.script_scale * 6.3 + line_seed * 20.0);

    return stroke * word * letters;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let p = in.world_position.xz;
    let time = globals.time;

    // The frontier wanders slightly, like a hand moving down the page
    let frontier = material.frontier_x
        + value_noise(vec2<f32>(p.y * 0.12, time * 0.18)) * 2.6 - 1.3;

    // Parchment base with paper grain and fibers
    let grain = fbm(p * 0.55) * 0.16 + fbm(p * 3.4) * 0.07;
    var color = material.parchment_color.rgb * (0.82 + grain);

    // Soft shadow in the spine gutter, ink staining near the river.
    // Squared by multiplication: pow() is indeterminate for negative bases.
    let gutter_x = p.x / 7.0;
    let stain_x = p.x / 3.4;
    let gutter_shadow = exp(-(gutter_x * gutter_x)) * 0.22;
    let ink_stain = exp(-(stain_x * stain_x)) * 0.5;
    color = color * (1.0 - gutter_shadow);
    color = mix(color, material.ink_color.rgb * 0.5, ink_stain);

    // Vignette toward the page edges
    let edge = smoothstep(40.0, 48.0, abs(p.x)) + smoothstep(26.0, 32.0, abs(p.y));
    color = color * (1.0 - clamp(edge, 0.0, 1.0) * 0.28);

    // Text only inside the margins of each page
    let margin = smoothstep(3.6, 5.2, abs(p.x))
        * (1.0 - smoothstep(42.0, 44.5, abs(p.x)))
        * (1.0 - smoothstep(26.0, 28.5, abs(p.y)));

    // Written side of the frontier
    let written = 1.0 - smoothstep(frontier - 1.5, frontier + 1.5, p.x);
    let text = script_mask(p) * margin * written;

    // Words near the frontier are freshly inked and still glow
    let freshness = 1.0 - smoothstep(0.0, 22.0, frontier - p.x);
    let flicker = 0.6 + 0.4 * sin(time * 2.4 + p.x * 0.8 + p.y * 0.5);
    let text_color = mix(
        material.ink_color.rgb,
        material.glow_color.rgb * (1.6 + flicker),
        clamp(freshness, 0.0, 1.0) * 0.95,
    );
    color = mix(color, text_color, clamp(text, 0.0, 1.0) * 0.95);

    // The shimmering frontier band itself
    let band = 1.0 - smoothstep(0.0, material.band_width, abs(p.x - frontier));
    let sparkle = pow(fbm(p * 2.6 + vec2<f32>(time * 0.5, -time * 0.35)), 2.0);
    color = color + material.glow_color.rgb * band * (0.9 + sparkle * 3.2);

    // Beyond the frontier: blank paper, cooler, with faint ruled ghost lines
    // and pencil hatching waiting for words
    let unwritten = smoothstep(frontier + material.band_width * 0.4, frontier + 7.0, p.x);
    let ruled = 1.0
        - smoothstep(
            0.02 * material.line_spacing,
            0.07 * material.line_spacing,
            abs(fract(p.y / material.line_spacing) - 0.5) * material.line_spacing,
        );
    let hatch = (0.5 + 0.5 * sin((p.x + p.y) * 3.1)) * fbm(p * 0.7);
    let blank = material.parchment_color.rgb * 1.06 + vec3<f32>(0.0, 0.01, 0.03);
    var sketch = blank * (0.96 + grain * 0.5);
    sketch = mix(sketch, material.sketch_color.rgb, ruled * margin * 0.16 + hatch * 0.05);
    color = mix(color, sketch, unwritten);

    // Faint paper sheen
    let view_dir = normalize(view.world_position.xyz - in.world_position.xyz);
    let normal = normalize(in.world_normal);
    let sheen = pow(clamp(dot(view_dir, normal), 0.0, 1.0), 3.0) * 0.05;
    color = color + material.parchment_color.rgb * sheen;

    return vec4<f32>(color, 1.0);
}
