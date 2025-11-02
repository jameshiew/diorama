#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct LiquidMetalMaterial {
    base_color: vec4<f32>,
    ripple_speed: f32,
    ripple_frequency: f32,
    metallic_strength: f32,
}

@group(3) @binding(0) var<uniform> material: LiquidMetalMaterial;

// Generate smooth random values
fn smooth_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);

    // Smooth interpolation
    let u = f * f * (3.0 - 2.0 * f);

    // Hash function for pseudo-random values
    let a = sin(dot(i, vec2<f32>(127.1, 311.7))) * 43758.5453;
    let b = sin(dot(i + vec2<f32>(1.0, 0.0), vec2<f32>(127.1, 311.7))) * 43758.5453;
    let c = sin(dot(i + vec2<f32>(0.0, 1.0), vec2<f32>(127.1, 311.7))) * 43758.5453;
    let d = sin(dot(i + vec2<f32>(1.0, 1.0), vec2<f32>(127.1, 311.7))) * 43758.5453;

    return mix(mix(fract(a), fract(b), u.x), mix(fract(c), fract(d), u.x), u.y);
}

// Fractal noise for complex surface patterns
fn fractal_noise(p: vec2<f32>) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 1.0;

    for (var i = 0; i < 6; i++) {
        value += amplitude * smooth_noise(p * frequency);
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    return value;
}

// Calculate surface normal for realistic metallic reflection
fn calculate_normal(uv: vec2<f32>, time: f32) -> vec3<f32> {
    let eps = 0.001;

    let h_x = fractal_noise(uv + vec2<f32>(eps, 0.0) + time * 0.1);
    let h_x2 = fractal_noise(uv - vec2<f32>(eps, 0.0) + time * 0.1);
    let h_y = fractal_noise(uv + vec2<f32>(0.0, eps) + time * 0.1);
    let h_y2 = fractal_noise(uv - vec2<f32>(0.0, eps) + time * 0.1);

    let dx = (h_x - h_x2) / (2.0 * eps);
    let dy = (h_y - h_y2) / (2.0 * eps);

    return normalize(vec3<f32>(-dx, -dy, 1.0));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let time = globals.time;

    // Create flowing liquid surface with multiple ripple layers
    let ripple1 = sin((uv.x * material.ripple_frequency + time * material.ripple_speed) * 2.0);
    let ripple2 = sin((uv.y * material.ripple_frequency * 1.3 + time * material.ripple_speed * 1.1) * 1.8);
    let ripple3 = sin((length(uv - 0.5) * material.ripple_frequency * 2.0 + time * material.ripple_speed * 0.8) * 2.5);

    // Add surface turbulence
    let turbulence = fractal_noise(uv * 8.0 + time * 0.2) * 0.3;

    // Combine ripples and turbulence
    let surface_height = (ripple1 + ripple2 + ripple3) * 0.1 + turbulence;

    // Calculate surface normal for reflection
    let normal = calculate_normal(uv, time * material.ripple_speed);

    // Create metallic reflection effect
    let view_dir = normalize(vec3<f32>(0.0, 0.0, 1.0)); // Simplified view direction
    let reflection = reflect(-view_dir, normal);

    // Simulate environment reflection (simplified)
    let env_color = vec3<f32>(0.5, 0.7, 1.0); // Sky-like color
    let reflected_color = env_color * (0.5 + 0.5 * reflection.z);

    // Create fresnel effect
    let fresnel = pow(1.0 - max(0.0, dot(normal, view_dir)), 2.0);

    // Base metallic color with surface variations
    var base_color = material.base_color.rgb;
    base_color += surface_height * 0.2; // Vary color based on surface height

    // Mix base color with reflections based on metallic strength
    let metallic_factor = material.metallic_strength;
    let final_color = mix(base_color, reflected_color, metallic_factor * fresnel);

    // Add highlights for liquid metal shine
    let highlight_intensity = pow(max(0.0, normal.z), 16.0);
    let highlight_color = vec3<f32>(1.0, 1.0, 1.0) * highlight_intensity * 0.5;

    // Add subtle color shifting based on surface movement
    let color_shift = sin(time * 2.0 + surface_height * 10.0) * 0.1;
    let shifted_color = final_color + vec3<f32>(color_shift, color_shift * 0.5, -color_shift * 0.3);

    // Combine everything
    let result_color = shifted_color + highlight_color;

    return vec4<f32>(result_color, material.base_color.a);
}
