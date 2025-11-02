// Animated color-shifting shader using perceptual color space
//
// This shader creates smooth color transitions by:
// 1. Using time-based sinusoidal animation
// 2. Blending colors in Oklab perceptual color space
// 3. Varying intensity based on distance from UV center

#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

/// Animation speed multiplier
const ANIMATION_SPEED: f32 = 2.0;

/// UV distance scaling factor
const UV_DISTANCE_SCALE: f32 = 1.4;

/// Converts Oklab color space to linear sRGB
///
/// Oklab is a perceptual color space that produces more natural-looking
/// color transitions than direct RGB interpolation.
/// Reference: https://bottosson.github.io/posts/oklab/
fn oklab_to_linear_srgb(c: vec3<f32>) -> vec3<f32> {
    let L = c.x;
    let a = c.y;
    let b = c.z;

    // Transform from Oklab to LMS cone response
    let l_ = L + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = L - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = L - 0.0894841775 * a - 1.2914855480 * b;

    // Cube the LMS values to get linear LMS
    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    // Transform from linear LMS to linear sRGB
    return vec3<f32>(
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    );
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate time-based animation values
    let time = globals.time * ANIMATION_SPEED;
    let t_sin = sin(time) * 0.5 + 0.5; // Normalized to [0, 1]
    let t_cos = cos(time);

    // Calculate radial distance from UV center for gradient effect
    let distance_to_center = distance(in.uv, vec2<f32>(0.5)) * UV_DISTANCE_SCALE;

    // Define colors in Oklab color space for perceptual blending
    let red = vec3<f32>(0.627955, 0.224863, 0.125846);
    let green = vec3<f32>(0.86644, -0.233887, 0.179498);
    let blue = vec3<f32>(0.701674, 0.274566, -0.169156);
    let white = vec3<f32>(1.0, 0.0, 0.0);

    // Blend colors based on time and distance
    let color_blend = mix(
        mix(red, blue, t_sin),
        mix(green, white, t_cos),
        distance_to_center
    );

    // Convert from Oklab to linear sRGB for output
    return vec4<f32>(oklab_to_linear_srgb(color_blend), 1.0);
}
