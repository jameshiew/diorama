use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use noise::{NoiseFn, Perlin};

/// Material properties for the marble floor
const MARBLE_BASE_COLOR: Color = Color::srgb(0.97, 0.97, 0.97);
const MARBLE_METALLIC: f32 = 0.15;
const MARBLE_ROUGHNESS: f32 = 0.15;
const MARBLE_REFLECTANCE: f32 = 0.9;

/// Noise generation seed for consistent marble patterns
const PERLIN_SEED: u32 = 42;

/// Noise scale factors for multi-octave marble effect
const NOISE_SCALE_PRIMARY: f64 = 8.0;
const NOISE_SCALE_SECONDARY: f64 = 16.0;
const NOISE_SCALE_TERTIARY: f64 = 4.0;

/// Creates a marble floor material with procedurally generated texture
pub fn create_marble_floor_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
) -> Handle<StandardMaterial> {
    let marble_texture = generate_marble_texture(images, 1024, 1024);

    materials.add(StandardMaterial {
        base_color_texture: Some(marble_texture),
        base_color: MARBLE_BASE_COLOR,
        metallic: MARBLE_METALLIC,
        perceptual_roughness: MARBLE_ROUGHNESS,
        reflectance: MARBLE_REFLECTANCE,
        ..default()
    })
}

/// Generates a procedural marble texture using multi-octave Perlin noise
///
/// Creates realistic marble veining by combining multiple noise octaves:
/// - Primary (8x): Base marble pattern
/// - Secondary (16x): Fine detail and variation
/// - Tertiary (4x): Large-scale structure
fn generate_marble_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(PERLIN_SEED);
    let pixel_count = width.saturating_mul(height).saturating_mul(4) as usize;
    let mut data = Vec::with_capacity(pixel_count);

    for y in 0..height {
        for x in 0..width {
            let pixel_color = calculate_marble_pixel(&perlin, x, y, width, height);
            data.extend_from_slice(&pixel_color);
        }
    }

    let image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        default(),
    );

    images.add(image)
}

/// Calculates the RGBA color for a single marble texture pixel
fn calculate_marble_pixel(perlin: &Perlin, x: u32, y: u32, width: u32, height: u32) -> [u8; 4] {
    // Normalize coordinates to [0, 1]
    let nx = f64::from(x) / f64::from(width);
    let ny = f64::from(y) / f64::from(height);

    // Sample noise at multiple scales for realistic marble veining
    let noise_primary = perlin.get([nx * NOISE_SCALE_PRIMARY, ny * NOISE_SCALE_PRIMARY]);
    let noise_secondary = perlin.get([nx * NOISE_SCALE_SECONDARY, ny * NOISE_SCALE_SECONDARY]);
    let noise_tertiary = perlin.get([nx * NOISE_SCALE_TERTIARY, ny * NOISE_SCALE_TERTIARY]);

    // Combine noise octaves with different weights
    let marble_pattern = (noise_primary + noise_secondary * 0.5 + noise_tertiary * 0.25).abs();
    let veining = (marble_pattern * 8.0).sin();

    // Calculate final color with subtle variations
    let base_brightness = 0.9 + veining * 0.1;
    let gray_variation = 0.95 + noise_secondary * 0.05;

    // Slight blue tint to simulate real marble
    let red = (base_brightness * gray_variation * 255.0) as u8;
    let green = (base_brightness * gray_variation * 255.0) as u8;
    let blue = ((base_brightness - 0.02) * gray_variation * 255.0) as u8;

    [red, green, blue, 255]
}
