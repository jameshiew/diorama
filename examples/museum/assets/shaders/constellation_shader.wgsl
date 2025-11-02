#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct ConstellationMaterial {
    star_color: vec4<f32>,
    nebula_color: vec4<f32>,
    twinkle_speed: f32,
    star_density: f32,
}

@group(3) @binding(0) var<uniform> material: ConstellationMaterial;

// Improved hash functions for better star distribution
fn hash13(p: vec3<f32>) -> f32 {
    var p3 = fract(p * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn hash33(p: vec3<f32>) -> vec3<f32> {
    var p3 = fract(p * vec3<f32>(0.1031, 0.1030, 0.0973));
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.xxy + p3.yzz) * p3.zyx);
}

fn hash21(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.13);
    p3 += dot(p3, p3.yzx + 3.333);
    return fract((p3.x + p3.y) * p3.z);
}

// Create moving star field with depth layers
fn create_moving_stars(uv: vec2<f32>, time: f32) -> vec3<f32> {
    var stars = vec3<f32>(0.0);

    // Multiple layers for depth and movement variety
    for (var layer = 0; layer < 4; layer++) {
        let layer_f = f32(layer);
        let speed = 0.03 + layer_f * 0.02; // Different speeds per layer
        let scale = 12.0 + layer_f * 8.0; // Different densities per layer

        // Layer-specific movement with slight rotation
        let rotation = time * 0.01 * (1.0 + layer_f * 0.2);
        let cos_rot = cos(rotation);
        let sin_rot = sin(rotation);
        let centered_uv = uv - 0.5;
        let rotated_uv = vec2<f32>(
            centered_uv.x * cos_rot - centered_uv.y * sin_rot,
            centered_uv.x * sin_rot + centered_uv.y * cos_rot
        ) + 0.5;

        let moving_uv = rotated_uv + vec2<f32>(time * speed, time * speed * 0.3);
        let cell_uv = moving_uv * scale;
        let cell_id = floor(cell_uv);
        let local_uv = fract(cell_uv);

        // Star position with layer-specific seed
        let star_seed = cell_id + vec2<f32>(layer_f * 127.3, layer_f * 269.5);
        let offset = vec2<f32>(hash21(star_seed), hash21(star_seed + vec2<f32>(100.0, 200.0))) * 0.8 + 0.1;
        let star_distance = length(local_uv - offset);

        // Star properties
        let star_brightness = hash21(star_seed + vec2<f32>(300.0, 400.0));
        let star_size = (0.015 + hash21(star_seed + vec2<f32>(500.0, 600.0)) * 0.02) / (1.0 + layer_f * 0.3);

        // Show stars based on density and layer
        let density_threshold = 0.6 + layer_f * 0.1 - material.star_density * 0.3;
        if (star_brightness > density_threshold) {
            // Star glow with sharper edges
            var star_glow = 1.0 - smoothstep(0.0, star_size, star_distance);
            star_glow = pow(star_glow, 2.0);

            // Add diffraction spikes for brighter stars
            if (star_brightness > 0.85) {
                let spike_size = star_size * 3.0;
                let spike1 = 1.0 - smoothstep(0.0, spike_size, abs(local_uv.x - offset.x)) *
                            smoothstep(spike_size * 0.1, 0.0, abs(local_uv.y - offset.y));
                let spike2 = 1.0 - smoothstep(0.0, spike_size, abs(local_uv.y - offset.y)) *
                            smoothstep(spike_size * 0.1, 0.0, abs(local_uv.x - offset.x));
                star_glow += (spike1 + spike2) * 0.4;
            }

            // Enhanced twinkling
            let twinkle_seed = hash21(star_seed + vec2<f32>(700.0, 800.0)) * 6.28;
            let fast_twinkle = sin(time * material.twinkle_speed * 3.0 + twinkle_seed);
            let slow_twinkle = sin(time * material.twinkle_speed * 0.8 + twinkle_seed * 1.7);
            let twinkle = 0.6 + 0.25 * fast_twinkle + 0.15 * slow_twinkle;

            // Star color based on temperature
            let temp = hash21(star_seed + vec2<f32>(900.0, 1000.0));
            var star_color = material.star_color.rgb;

            if (temp > 0.8) {
                star_color = vec3<f32>(0.7, 0.8, 1.0); // Blue giants
            } else if (temp > 0.6) {
                star_color = vec3<f32>(0.9, 0.95, 1.0); // Blue-white
            } else if (temp > 0.4) {
                star_color = vec3<f32>(1.0, 1.0, 0.9); // White
            } else if (temp > 0.2) {
                star_color = vec3<f32>(1.0, 0.9, 0.7); // Yellow
            } else {
                star_color = vec3<f32>(1.0, 0.6, 0.4); // Red
            }

            // Layer brightness for depth effect
            let layer_brightness = 1.0 - layer_f * 0.2;
            let final_intensity = star_glow * star_brightness * twinkle * layer_brightness * 2.5;

            stars += star_color * final_intensity;
        }
    }

    return stars;
}

// Create moving cosmic dust particles
fn create_cosmic_dust(uv: vec2<f32>, time: f32) -> vec3<f32> {
    let dust_speed = 0.02;
    let moving_uv = uv + vec2<f32>(time * dust_speed, time * dust_speed * 0.5);

    // Fine dust particles
    let dust_scale = 40.0;
    let dust_cell = floor(moving_uv * dust_scale);
    let dust_local = fract(moving_uv * dust_scale);

    let dust_brightness = hash21(dust_cell) * 0.05; // Very faint
    let dust_size = 0.8 / dust_scale;
    let dust_distance = length(dust_local - vec2<f32>(0.5));

    let dust_glow = 1.0 - smoothstep(0.0, dust_size, dust_distance);
    return vec3<f32>(0.3, 0.3, 0.4) * dust_glow * dust_brightness;
}

// Generate occasional shooting stars/meteors
fn create_shooting_stars(uv: vec2<f32>, time: f32) -> vec3<f32> {
    var result = vec3<f32>(0.0);

    // Multiple shooting star tracks with different patterns
    for (var i = 0; i < 4; i++) {
        let track_time = time * 0.15 + f32(i) * 234.5; // Faster shooting stars
        let track_cycle = fract(track_time);

        // Show shooting star 8% of the time per track (more frequent)
        if (track_cycle < 0.08) {
            let seed = floor(track_time) + f32(i) * 789.1;

            // More dramatic start and end positions
            let start_angle = hash13(vec3<f32>(seed, 0.0, 0.0)) * 6.28;
            let start_radius = 1.8 + hash13(vec3<f32>(seed, 1.0, 0.0)) * 0.5;
            let start_pos = vec2<f32>(cos(start_angle), sin(start_angle)) * start_radius;

            // End position with more variety
            let end_angle = start_angle + (hash13(vec3<f32>(seed, 2.0, 0.0)) - 0.5) * 2.0;
            let end_radius = 0.2 + hash13(vec3<f32>(seed, 3.0, 0.0)) * 0.3;
            let end_pos = vec2<f32>(cos(end_angle), sin(end_angle)) * end_radius;

            let progress = track_cycle / 0.08;
            let eased_progress = progress * progress * (3.0 - 2.0 * progress); // Smooth easing
            let current_pos = mix(start_pos, end_pos, eased_progress);

            // Longer, more dramatic trail
            let trail_length = 0.6;
            let trail_start = mix(start_pos, end_pos, max(0.0, eased_progress - trail_length));

            // Distance to trail line with better calculation
            let line_dir = normalize(current_pos - trail_start);
            let to_point = uv - trail_start;
            let proj_length = dot(to_point, line_dir);
            let closest_point = trail_start + line_dir * clamp(proj_length, 0.0, length(current_pos - trail_start));
            let dist_to_line = length(uv - closest_point);

            // Enhanced trail width and intensity
            let trail_width = 0.025 + hash13(vec3<f32>(seed, 4.0, 0.0)) * 0.015;
            if (dist_to_line < trail_width && proj_length >= 0.0 && proj_length <= length(current_pos - trail_start)) {
                var trail_intensity = 1.0 - (dist_to_line / trail_width);
                trail_intensity = pow(trail_intensity, 1.5); // Sharper falloff

                var fade = 1.0 - (proj_length / length(current_pos - trail_start));
                fade = pow(fade, 0.8); // Softer fade along length

                // Bright head of meteor
                let head_intensity = 1.0 - smoothstep(0.8, 1.0, proj_length / length(current_pos - trail_start));

                // Color variation for different meteors
                let meteor_type = hash13(vec3<f32>(seed, 5.0, 0.0));
                var meteor_color = vec3<f32>(1.0, 0.9, 0.6); // Default yellow-white

                if (meteor_type > 0.7) {
                    meteor_color = vec3<f32>(0.8, 1.0, 0.9); // Green meteor
                } else if (meteor_type > 0.4) {
                    meteor_color = vec3<f32>(1.0, 0.7, 0.4); // Orange meteor
                }

                let final_intensity = trail_intensity * fade * (1.0 + head_intensity * 2.0) * 3.0;
                result += meteor_color * final_intensity;
            }
        }
    }

    return result;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = globals.time;

    // Start with pure black space background
    var color = vec3<f32>(0.0, 0.0, 0.0);

    // Add subtle cosmic dust for texture
    color += create_cosmic_dust(uv, time) * 0.5;

    // Add moving star field (main feature)
    color += create_moving_stars(uv, time);

    // Add dramatic shooting stars
    color += create_shooting_stars(uv, time);

    // Add very subtle deep space glow around the edges
    let center_dist = length(uv - 0.5);
    let space_glow = (1.0 - smoothstep(0.0, 0.8, center_dist)) * 0.02;
    color += vec3<f32>(0.05, 0.03, 0.08) * space_glow;

    // Slight vignette effect for depth
    let vignette = 1.0 - smoothstep(0.4, 1.0, center_dist) * 0.3;
    color *= vignette;

    // Enhance contrast for deeper space feel
    color = pow(color, vec3<f32>(0.85));

    return vec4<f32>(color, 1.0);
}
