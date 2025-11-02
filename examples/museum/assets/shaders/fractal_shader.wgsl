#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct FractalData {
    base_color: vec4<f32>,
    time: f32,
    zoom: f32,
    offset_x: f32,
    offset_y: f32,
    max_iterations: f32,
    color_intensity: f32,
    animation_speed: f32,
    _padding: f32,
}

@group(3) @binding(0) var<uniform> material: FractalData;

fn mandelbrot_iterations(c: vec2<f32>) -> f32 {
    var z = vec2<f32>(0.0, 0.0);
    var iterations = 0.0;
    let max_iter = u32(material.max_iterations);

    for (var i = 0u; i < max_iter; i += 1u) {
        if (dot(z, z) > 4.0) {
            break;
        }

        // z = z^2 + c
        let z_new = vec2<f32>(
            z.x * z.x - z.y * z.y + c.x,
            2.0 * z.x * z.y + c.y
        );
        z = z_new;
        iterations = f32(i);
    }

    // Smooth iteration count for better gradients
    if (dot(z, z) > 4.0) {
        iterations = iterations + 1.0 - log2(log2(dot(z, z)) / 2.0);
    }

    return iterations / material.max_iterations;
}

fn julia_iterations(z: vec2<f32>, c: vec2<f32>) -> f32 {
    var current_z = z;
    var iterations = 0.0;
    let max_iter = u32(material.max_iterations);

    for (var i = 0u; i < max_iter; i += 1u) {
        if (dot(current_z, current_z) > 4.0) {
            break;
        }

        // z = z^2 + c
        let z_new = vec2<f32>(
            current_z.x * current_z.x - current_z.y * current_z.y + c.x,
            2.0 * current_z.x * current_z.y + c.y
        );
        current_z = z_new;
        iterations = f32(i);
    }

    // Smooth iteration count
    if (dot(current_z, current_z) > 4.0) {
        iterations = iterations + 1.0 - log2(log2(dot(current_z, current_z)) / 2.0);
    }

    return iterations / material.max_iterations;
}

fn color_from_iteration(t: f32) -> vec3<f32> {
    // Create a colorful gradient based on iteration count
    let t_mod = fract(t * material.color_intensity + material.time * material.animation_speed);

    // Multi-colored palette using sine waves
    let r = 0.5 + 0.5 * sin(6.28318 * t_mod + 0.0);
    let g = 0.5 + 0.5 * sin(6.28318 * t_mod + 2.09439); // +2π/3
    let b = 0.5 + 0.5 * sin(6.28318 * t_mod + 4.18879); // +4π/3

    // Add some shimmer and variation
    let shimmer = 0.1 * sin(t * 50.0 + material.time * 3.0);

    return vec3<f32>(
        clamp(r + shimmer, 0.0, 1.0),
        clamp(g + shimmer, 0.0, 1.0),
        clamp(b + shimmer, 0.0, 1.0)
    );
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Transform UV to complex plane coordinates
    let uv = (in.uv - 0.5) * material.zoom;
    let c = vec2<f32>(
        uv.x + material.offset_x,
        uv.y + material.offset_y
    );

    // Animate between Mandelbrot and Julia sets
    let time_cycle = sin(material.time * 0.3) * 0.5 + 0.5;

    // Mandelbrot set calculation
    let mandelbrot_iter = mandelbrot_iterations(c);

    // Julia set calculation with animated parameter
    let julia_c = vec2<f32>(
        -0.8 + 0.2 * cos(material.time * 0.7),
        0.156 + 0.1 * sin(material.time * 0.5)
    );
    let julia_iter = julia_iterations(c, julia_c);

    // Blend between Mandelbrot and Julia
    let final_iter = mix(mandelbrot_iter, julia_iter, time_cycle);

    // Generate colorful output
    var final_color: vec3<f32>;
    if (final_iter >= 0.98) {
        // Inside the set - use a dark color with some variation
        final_color = vec3<f32>(0.05, 0.02, 0.1) + 0.1 * color_from_iteration(final_iter * 10.0);
    } else {
        // Outside the set - use the colorful gradient
        final_color = color_from_iteration(final_iter);
    }

    // Mix with base color
    final_color = mix(final_color, material.base_color.rgb, 0.1);

    return vec4<f32>(final_color, material.base_color.a);
}
