//! # Material System
//!
//! Comprehensive material library for the museum scene.
//!
//! ## Material Types
//!
//! ### Standard PBR Materials
//! - **Floor**: Polished marble with veining, subtle clearcoat
//! - **Walls**: Textured plaster with warm emissive glow
//! - **Ceiling**: High-reflectance white for light distribution
//! - **Wood Frames**: Procedural wood grain texture
//! - **Gold Frames**: High metallic/reflectance gold
//! - **Marble Pedestals**: High-quality marble with clearcoat
//! - **Polished Stone**: Dark stone with mirror-like finish
//!
//! ### Custom Shader Materials
//! - **GlassMaterial**: Translucent with fresnel and refraction
//! - **GeometricMaterial**: Animated pulsing energy fields
//! - **FractalMaterial**: Real-time Mandelbrot/Julia sets
//!
//! ## Texture Generation
//! All textures use high-resolution Perlin noise for realistic appearance:
//! - Marble veining with multiple noise octaves
//! - Normal maps for surface detail
//! - Wood grain patterns
//! - Stone texture with micro-detail
//!
//! ## Performance
//! - Textures cached at startup (no runtime generation)
//! - Material instances reused across similar objects
//! - Normal maps provide depth without geometry cost

#![allow(dead_code)]

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{
    AsBindGroup, Extent3d, ShaderType, TextureDimension, TextureFormat,
};
use bevy::shader::ShaderRef;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

use crate::shader_materials::{FractalMaterial, create_fractal_material};

/// Translucent glass material with fresnel effects for display cases
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GlassMaterial {
    #[uniform(0)]
    pub data: GlassData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
#[allow(dead_code)] // All fields used by GPU shader, not detectable by static analysis
pub struct GlassData {
    pub base_color: Vec4,
    pub transparency: f32,
    pub refraction_strength: f32,
    pub fresnel_power: f32,
    pub _padding: f32,
}

impl Material for GlassMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/glass_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

/// Animated geometric material with pulsing energy fields
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GeometricMaterial {
    #[uniform(0)]
    pub data: GeometricData,
    #[texture(1)]
    #[sampler(2)]
    pub noise_texture: Option<Handle<Image>>,
}

#[derive(Debug, Clone, Copy, ShaderType)]
#[allow(dead_code)] // All fields used by GPU shader, not detectable by static analysis
pub struct GeometricData {
    pub primary_color: Vec4,
    pub secondary_color: Vec4,
    pub glow_intensity: f32,
    pub animation_speed: f32,
    pub metallic: f32,
    pub roughness: f32,
    pub time: f32,
    pub _padding: f32,
}

impl Material for GeometricMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/geometric_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}

/// Collection of materials used throughout the museum
pub struct MuseumMaterials {
    pub floor: Handle<StandardMaterial>,
    pub wall: Handle<StandardMaterial>,
    pub ceiling: Handle<StandardMaterial>,
    pub frame_wood: Handle<StandardMaterial>,
    pub frame_gold: Handle<StandardMaterial>,
    pub pedestal_marble: Handle<StandardMaterial>,
    // New advanced materials
    pub glass_display_shader: Handle<GlassMaterial>, // Shader-based translucent glass
    pub polished_stone: Handle<StandardMaterial>,
    pub glowing_sculpture: Handle<GeometricMaterial>, // Custom shader for geometric sculpture
    pub fractal_painting: Handle<FractalMaterial>,    // Fractal shader for paintings
}

pub fn create_museum_materials(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    glass_materials: &mut ResMut<Assets<GlassMaterial>>,
    geometric_materials: &mut ResMut<Assets<GeometricMaterial>>,
    fractal_materials: &mut ResMut<Assets<FractalMaterial>>,
    images: &mut ResMut<Assets<Image>>,
) -> MuseumMaterials {
    MuseumMaterials {
        floor: create_marble_floor_material(materials, images),
        wall: create_wall_material(materials, images),
        ceiling: create_ceiling_material(materials),
        frame_wood: create_wood_frame_material(materials, images),
        frame_gold: create_gold_frame_material(materials),
        pedestal_marble: create_marble_pedestal_material(materials, images),
        glass_display_shader: create_glass_display_shader_material(glass_materials),
        polished_stone: create_polished_stone_material(materials, images),
        glowing_sculpture: create_geometric_shader_material(geometric_materials, images),
        fractal_painting: create_fractal_material(
            fractal_materials,
            Color::srgb(0.1, 0.2, 0.8), // Base blue color
            3.0,                        // zoom
            -0.5,                       // offset_x
            0.0,                        // offset_y
        ),
    }
}

fn create_marble_floor_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
) -> Handle<StandardMaterial> {
    let marble_texture = generate_marble_texture(images, 1024, 1024);
    let marble_normal = generate_marble_normal_map(images, 1024, 1024);

    materials.add(StandardMaterial {
        base_color_texture: Some(marble_texture),
        normal_map_texture: Some(marble_normal),
        base_color: Color::srgb(0.95, 0.95, 0.95), // Slightly less bright marble
        metallic: 0.03,                            // Reduced metallic shine
        perceptual_roughness: 0.12,                // Slightly more rough
        reflectance: 0.85,                         // Reduced reflectance
        clearcoat: 0.2,                            // Reduced clearcoat
        clearcoat_perceptual_roughness: 0.05,      // Slightly rougher clearcoat
        specular_transmission: 0.05,               // Reduced transmission
        thickness: 0.05,                           // Thin transmission layer
        ior: 1.5,                                  // Index of refraction for stone
        ..default()
    })
}

fn create_wall_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
) -> Handle<StandardMaterial> {
    let wall_texture = generate_wall_texture(images, 512, 512);
    let wall_normal = generate_wall_normal_map(images, 512, 512);

    materials.add(StandardMaterial {
        base_color_texture: Some(wall_texture),
        normal_map_texture: Some(wall_normal),
        base_color: Color::srgb(0.96, 0.96, 0.94), // Warmer white
        metallic: 0.0,
        perceptual_roughness: 0.6,                      // Slightly smoother
        reflectance: 0.4,                               // Better light bounce
        emissive: LinearRgba::rgb(0.008, 0.008, 0.006), // Reduced warm glow
        emissive_exposure_weight: 0.3,                  // Reduced exposure influence
        ..default()
    })
}

fn create_ceiling_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Color::srgb(0.99, 0.99, 0.97), // Brighter white
        metallic: 0.0,
        perceptual_roughness: 0.8, // Slightly less rough for better light reflection
        reflectance: 0.4,          // Added reflectance for better light bounce
        ..default()
    })
}

fn create_wood_frame_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
) -> Handle<StandardMaterial> {
    let wood_texture = generate_wood_texture(images, 512, 512); // Increased from 128x128

    materials.add(StandardMaterial {
        base_color_texture: Some(wood_texture),
        base_color: Color::srgb(0.6, 0.4, 0.2),
        metallic: 0.0,
        perceptual_roughness: 0.7,
        ..default()
    })
}

fn create_gold_frame_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.84, 0.0),
        metallic: 0.9,
        perceptual_roughness: 0.1,
        reflectance: 0.9,
        ..default()
    })
}

fn create_marble_pedestal_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
) -> Handle<StandardMaterial> {
    let marble_texture = generate_marble_texture(images, 512, 512); // Increased from 256x256

    materials.add(StandardMaterial {
        base_color_texture: Some(marble_texture),
        base_color: Color::srgb(0.9, 0.9, 0.85),
        metallic: 0.2,
        perceptual_roughness: 0.1,
        reflectance: 0.8,
        ..default()
    })
}

fn create_glass_display_shader_material(
    glass_materials: &mut ResMut<Assets<GlassMaterial>>,
) -> Handle<GlassMaterial> {
    glass_materials.add(GlassMaterial {
        data: GlassData {
            base_color: Vec4::new(0.92, 0.95, 1.0, 0.25), // Subtle blue tint with low base alpha
            transparency: 0.25,                           // Base transparency level
            refraction_strength: 1.0,                     // How much the glass refracts light
            fresnel_power: 2.0,                           // Controls how the fresnel effect appears
            _padding: 0.0,
        },
    })
}

fn create_polished_stone_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
) -> Handle<StandardMaterial> {
    let stone_texture = generate_polished_stone_texture(images, 1024, 1024);
    let stone_normal = generate_stone_normal_map(images, 1024, 1024);

    materials.add(StandardMaterial {
        base_color_texture: Some(stone_texture),
        normal_map_texture: Some(stone_normal),
        base_color: Color::srgb(0.3, 0.25, 0.2), // Dark stone
        metallic: 0.1,                           // Slight metallic content
        perceptual_roughness: 0.05,              // Highly polished
        reflectance: 0.8,                        // High reflectance
        clearcoat: 0.6,                          // Strong protective layer
        clearcoat_perceptual_roughness: 0.01,    // Mirror-like clearcoat
        specular_transmission: 0.05,             // Minimal transmission
        thickness: 0.1,                          // Thicker material
        ior: 1.6,                                // Stone-like IOR
        ..default()
    })
}

fn create_geometric_shader_material(
    geometric_materials: &mut ResMut<Assets<GeometricMaterial>>,
    images: &mut ResMut<Assets<Image>>,
) -> Handle<GeometricMaterial> {
    // Create a noise texture for the shader
    let noise_texture = generate_noise_texture(images, 256, 256);

    geometric_materials.add(GeometricMaterial {
        data: GeometricData {
            primary_color: Vec4::new(0.2, 0.4, 0.8, 1.0), // Blue primary
            secondary_color: Vec4::new(0.8, 0.9, 1.0, 1.0), // Light blue secondary
            glow_intensity: 2.5,                          // Moderate glow
            animation_speed: 0.8,                         // Smooth animation
            metallic: 0.9,                                // High metallic
            roughness: 0.1,                               // Very smooth
            time: 0.0,                                    // Will be updated in shader
            _padding: 0.0,
        },
        noise_texture: Some(noise_texture),
    })
}

fn generate_marble_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(42);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            // Create marble-like veining
            let noise1 = perlin.get([nx * 8.0, ny * 8.0]);
            let noise2 = perlin.get([nx * 16.0, ny * 16.0]);
            let noise3 = perlin.get([nx * 4.0, ny * 4.0]);

            let marble_pattern = (noise1 + noise2 * 0.5 + noise3 * 0.25).abs();
            let veining = (marble_pattern * 8.0).sin();

            let base_color = 0.9 + veining * 0.1;
            let gray_variation = 0.95 + noise2 * 0.05;

            let r = (base_color * gray_variation * 255.0) as u8;
            let g = (base_color * gray_variation * 255.0) as u8;
            let b = ((base_color - 0.02) * gray_variation * 255.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
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

fn generate_marble_normal_map(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(42);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            // Generate height variations for normal mapping
            let height1 = perlin.get([nx * 16.0, ny * 16.0]) * 0.5
                + perlin.get([nx * 32.0, ny * 32.0]) * 0.25
                + perlin.get([nx * 64.0, ny * 64.0]) * 0.125;

            // Calculate normal from height gradients
            let height_right = perlin.get([(nx + 1.0 / width as f64) * 16.0, ny * 16.0]) * 0.5;
            let height_up = perlin.get([nx * 16.0, (ny + 1.0 / height as f64) * 16.0]) * 0.5;

            let gradient_x = (height_right - height1) * 0.5 + 0.5;
            let gradient_y = (height_up - height1) * 0.5 + 0.5;
            let gradient_z = 0.8; // Mostly pointing up

            let r = (gradient_x * 255.0) as u8;
            let g = (gradient_y * 255.0) as u8;
            let b = (gradient_z * 255.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
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

fn generate_wall_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(123);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            // Subtle wall texture
            let noise = perlin.get([nx * 20.0, ny * 20.0]) * 0.02;
            let base = 0.92 + noise;

            let r = (base * 255.0) as u8;
            let g = (base * 255.0) as u8;
            let b = ((base - 0.02) * 255.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
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

fn generate_wall_normal_map(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(789);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            // Subtle wall surface variations
            let height_variation = perlin.get([nx * 40.0, ny * 40.0]) * 0.1;
            let gradient_x = height_variation * 0.3 + 0.5;
            let gradient_y = height_variation * 0.3 + 0.5;
            let gradient_z = 0.9; // Mostly flat

            let r = (gradient_x * 255.0) as u8;
            let g = (gradient_y * 255.0) as u8;
            let b = (gradient_z * 255.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
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

fn generate_wood_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(456);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            // Wood grain pattern
            let grain = perlin.get([nx * 2.0, ny * 20.0]) * 0.3;
            let ring = (ny * 10.0).sin() * 0.1;

            let wood_color = 0.5 + grain + ring;

            let r = (wood_color * 0.6 * 255.0) as u8;
            let g = (wood_color * 0.4 * 255.0) as u8;
            let b = (wood_color * 0.2 * 255.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
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

fn generate_polished_stone_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let fbm: Fbm<Perlin> = Fbm::new(654).set_octaves(6).set_frequency(1.0);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            // Complex stone pattern using fractal noise
            let stone_pattern = fbm.get([nx * 8.0, ny * 8.0]);
            let mineral_veins = fbm.get([nx * 20.0, ny * 20.0]) * 0.3;

            let base_tone = 0.25 + stone_pattern * 0.1 + mineral_veins.abs() * 0.15;

            let r = (base_tone * 1.2 * 255.0).min(255.0) as u8;
            let g = (base_tone * 255.0) as u8;
            let b = (base_tone * 0.8 * 255.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
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

fn generate_stone_normal_map(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let fbm: Fbm<Perlin> = Fbm::new(987).set_octaves(4).set_frequency(2.0);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            // Stone surface irregularities
            let height_variation = fbm.get([nx * 32.0, ny * 32.0]) * 0.2;

            // Calculate gradients for normal mapping
            let height_right = fbm.get([(nx + 1.0 / width as f64) * 32.0, ny * 32.0]) * 0.2;
            let height_up = fbm.get([nx * 32.0, (ny + 1.0 / height as f64) * 32.0]) * 0.2;

            let gradient_x = (height_right - height_variation) * 0.5 + 0.5;
            let gradient_y = (height_up - height_variation) * 0.5 + 0.5;
            let gradient_z = 0.7; // Less pronounced than marble

            let r = (gradient_x * 255.0) as u8;
            let g = (gradient_y * 255.0) as u8;
            let b = (gradient_z * 255.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
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

fn generate_noise_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(123); // Different seed for noise texture
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            // Create noise pattern for shader animation
            let noise1 = perlin.get([nx * 8.0, ny * 8.0]);
            let noise2 = perlin.get([nx * 16.0, ny * 16.0]);
            let noise3 = perlin.get([nx * 4.0, ny * 4.0]);

            let combined_noise = (noise1 + noise2 * 0.5 + noise3 * 0.25) * 0.5 + 0.5;
            let noise_value = (combined_noise * 255.0) as u8;

            // Store noise in all channels for shader flexibility
            data.extend_from_slice(&[noise_value, noise_value, noise_value, 255u8]);
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
        RenderAssetUsages::RENDER_WORLD,
    );

    images.add(image)
}
