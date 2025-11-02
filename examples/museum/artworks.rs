//! # Artwork System
//!
//! This module handles all artwork generation, placement, and interaction in the museum.
//!
//! ## Features
//! - Procedural texture generation for diverse painting styles
//! - Interactive dialogue system for artwork descriptions
//! - Animated sculptures with pulsing, color cycling, and rotation
//! - Physics-enabled installations
//! - Multiple material types including shader-based effects
//!
//! ## Painting Styles
//! Supports 12 different procedural art styles:
//! - Abstract, Geometric, ColorField, Organic
//! - Fractal (shader-based), Minimalist, Digital
//! - Noise, Cellular, Clouds, Marble, Gold
//!
//! ## Sculpture Types
//! - Twisted: Stacked rotating segments
//! - Geometric: Multi-part glowing assembly
//! - Organic: Flowing spherical forms
//! - Crystal: Color-cycling pillars
//!
//! ## Performance Notes
//! - Textures generated at 2048x2048 for high quality
//! - Saturating arithmetic prevents overflow in procedural generation
//! - Dialogue runners automatically cleaned up after completion

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_yarnspinner::prelude::*;
use diorama::picking::Hint;
use noise::{NoiseFn, Perlin};

use crate::config::{FrameType, PaintingConfig, PaintingStyle, SculptureConfig, SculptureType};
use crate::materials::MuseumMaterials;
use crate::shader_materials::*;
use crate::{MuseumAssets, Rotating};

// Constants for painting and frame dimensions - scaled by 1.5x
const FRAME_DEPTH_REGULAR: f32 = 0.15; // Scaled from 0.1 to 0.15
const PAINTING_ART_DEPTH_REGULAR: f32 = 0.03; // Scaled from 0.02 to 0.03
const EFFECTIVE_PAINTING_OFFSET_REGULAR: f32 =
    FRAME_DEPTH_REGULAR / 2.0 + PAINTING_ART_DEPTH_REGULAR / 2.0; // 0.09 (was 0.06)

#[derive(Component)]
pub struct PaintingDialogue {
    pub node_name: String,
}

// Animation components for sculpture garden
#[derive(Component)]
#[allow(dead_code)]
pub struct MorphingSculpture {
    pub speed: f32,
    pub amplitude: f32,
    pub base_mesh: Handle<Mesh>,
}

#[derive(Component)]
pub struct PulsingSculpture {
    pub speed: f32,
    pub scale_range: (f32, f32),
    pub phase: f32,
}

#[derive(Component)]
pub struct ColorCyclingSculpture {
    pub speed: f32,
    pub hue_offset: f32,
}

#[derive(Component)]
pub struct MaterialCycler {
    pub materials: Vec<Handle<StandardMaterial>>,
    pub current_index: usize,
}

pub fn place_artworks(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    museum_assets: &Res<MuseumAssets>,
    museum_materials: &MuseumMaterials,
) {
    place_wall_paintings(commands, meshes, materials, images, museum_materials);
    place_sculptures(commands, meshes, materials, museum_materials);
    place_central_installation(commands, meshes, materials, images, museum_assets);
}

fn place_wall_paintings(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    museum_materials: &MuseumMaterials,
) {
    // Use config-driven approach to reduce hardcoded values
    for config in PaintingConfig::main_gallery() {
        create_framed_painting(
            commands,
            meshes,
            materials,
            images,
            config.name,
            config.position,
            config.style,
            config.frame_type,
            museum_materials,
        );
    }
}

fn place_sculptures(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    museum_materials: &MuseumMaterials,
) {
    // Use config-driven approach to reduce hardcoded values
    for config in SculptureConfig::sculpture_garden() {
        create_sculpture(
            commands,
            meshes,
            materials,
            config.name,
            config.position,
            config.sculpture_type,
            museum_materials,
        );
    }
}

fn place_central_installation(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    museum_assets: &Res<MuseumAssets>,
) {
    // Create multiple materials for cycling
    let material_variants = vec![
        create_holographic_material(materials, images),
        create_crystal_material(materials),
        create_liquid_metal_material(materials),
        create_energy_material(materials),
        create_neon_material(materials),
    ];

    let initial_material = material_variants[0].clone();

    commands
        .spawn((
            Name::new("Central Holographic Installation"),
            Hint::new("🎨 Interactive Sphere - Click to cycle through 5 unique materials!"),
            Mesh3d(meshes.add(Sphere::new(1.5))), // Scaled from 1.0 to 1.5
            MeshMaterial3d(initial_material),
            Transform::from_xyz(0.0, 3.0, 0.0), // Scaled Y from 2.0 to 3.0
            Rotating,
            RigidBody::Dynamic,
            Collider::sphere(1.5), // Match mesh dimensions exactly (radius)
            MaterialCycler {
                materials: material_variants,
                current_index: 0,
            },
        ))
        .observe(on_sphere_click);

    for i in 0..6 {
        let angle = (i as f32) * std::f32::consts::PI * 2.0 / 6.0;
        let radius = 3.75; // Scaled from 2.5 to 3.75
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        let orbiting_material = create_orbiting_element_material(materials, i, museum_assets);

        commands.spawn((
            Name::new(format!("Orbiting Element {}", i + 1)),
            Hint::new("💬 Mysterious Cube - Click to hear its story"),
            Mesh3d(meshes.add(Cuboid::new(0.45, 0.45, 0.45))), // Scaled from 0.3 to 0.45
            MeshMaterial3d(orbiting_material),
            Transform::from_xyz(x, 2.25 + (i as f32 * 0.3), z), // Scaled Y from 1.5 to 2.25, spacing from 0.2 to 0.3
            Rotating,
        ));
    }
}

fn create_framed_painting(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    name: &str,
    position: Vec3,
    style: PaintingStyle,
    frame_type: FrameType,
    museum_materials: &MuseumMaterials,
) {
    let frame_material = match frame_type {
        FrameType::Wood => museum_materials.frame_wood.clone(),
        FrameType::Gold => museum_materials.frame_gold.clone(),
    };

    // Offset frame away from wall to prevent z-fighting
    // Frame needs to be positioned away from wall surface by at least frame_depth/2 + small gap
    const WALL_CLEARANCE: f32 = FRAME_DEPTH_REGULAR / 2.0 + 0.015; // 0.09 units clearance (scaled from 0.06)

    let rotation: Quat;
    let painting_offset: Vec3;
    let frame_position: Vec3;

    if position.z < -13.5 {
        // North wall (main room) or corridor walls
        if position.x > 5.0 && position.x < 7.0 {
            // Right corridor wall (faces west toward corridor center)
            rotation = Quat::from_rotation_y(std::f32::consts::PI / 2.0);
            frame_position = position + Vec3::new(-WALL_CLEARANCE, 0.0, 0.0);
            painting_offset = Vec3::new(-EFFECTIVE_PAINTING_OFFSET_REGULAR, 0.0, 0.0);
        } else if position.x < -5.0 && position.x > -7.0 {
            // Left corridor wall (faces east toward corridor center)
            rotation = Quat::from_rotation_y(-std::f32::consts::PI / 2.0);
            frame_position = position + Vec3::new(WALL_CLEARANCE, 0.0, 0.0);
            painting_offset = Vec3::new(EFFECTIVE_PAINTING_OFFSET_REGULAR, 0.0, 0.0);
        } else {
            // Main room north wall (faces south)
            rotation = Quat::IDENTITY;
            frame_position = position + Vec3::new(0.0, 0.0, WALL_CLEARANCE);
            painting_offset = Vec3::new(0.0, 0.0, EFFECTIVE_PAINTING_OFFSET_REGULAR);
        }
    } else if position.z > 13.5 {
        // Updated from 9.0 to account for scaled walls
        rotation = Quat::from_rotation_y(std::f32::consts::PI);
        frame_position = position + Vec3::new(0.0, 0.0, -WALL_CLEARANCE);
        painting_offset = Vec3::new(0.0, 0.0, -EFFECTIVE_PAINTING_OFFSET_REGULAR);
    } else if position.x > 13.5 {
        // Updated from 9.0 to account for scaled walls
        rotation = Quat::from_rotation_y(-std::f32::consts::PI / 2.0);
        frame_position = position + Vec3::new(-WALL_CLEARANCE, 0.0, 0.0);
        painting_offset = Vec3::new(-EFFECTIVE_PAINTING_OFFSET_REGULAR, 0.0, 0.0);
    } else if position.x < -13.5 {
        // Updated from -9.0 to account for scaled walls
        rotation = Quat::from_rotation_y(std::f32::consts::PI / 2.0);
        frame_position = position + Vec3::new(WALL_CLEARANCE, 0.0, 0.0);
        painting_offset = Vec3::new(EFFECTIVE_PAINTING_OFFSET_REGULAR, 0.0, 0.0);
    } else {
        warn!(
            "Painting \"{}\" at {:?} is not on a recognized main hall wall boundary. Defaulting orientation.",
            name, position
        );
        rotation = Quat::IDENTITY;
        frame_position = position + Vec3::new(0.0, 0.0, WALL_CLEARANCE);
        painting_offset = Vec3::new(0.0, 0.0, EFFECTIVE_PAINTING_OFFSET_REGULAR);
    }

    commands.spawn((
        Name::new(format!("{name} Frame")),
        Mesh3d(meshes.add(Cuboid::new(2.7, 2.1, FRAME_DEPTH_REGULAR))), // Scaled from (1.8, 1.4)
        MeshMaterial3d(frame_material),
        Transform::from_translation(frame_position).with_rotation(rotation),
    ));

    // Handle fractal painting separately due to different material types
    // Spawn the painting entity and attach click observer
    let mut painting_entity = if let PaintingStyle::Fractal = style {
        commands.spawn((
            Name::new(name.to_string()),
            Hint::new("🖼️ Procedural Artwork - Click to discuss the algorithms behind this piece"),
            Mesh3d(meshes.add(Cuboid::new(2.4, 1.8, PAINTING_ART_DEPTH_REGULAR))), // Scaled from (1.6, 1.2)
            MeshMaterial3d(museum_materials.fractal_painting.clone()),
            Transform::from_translation(frame_position + painting_offset).with_rotation(rotation),
            PaintingDialogue {
                node_name: get_dialogue_node_for_painting(name),
            },
        ))
    } else {
        // Use traditional texture-based material for other styles
        let painting_texture = generate_artwork_texture(images, style, 2048, 2048);
        let painting_material = materials.add(StandardMaterial {
            base_color_texture: Some(painting_texture),
            base_color: Color::WHITE,
            metallic: 0.0,
            perceptual_roughness: 0.8,
            ..default()
        });

        commands.spawn((
            Name::new(name.to_string()),
            Hint::new("🖼️ Procedural Artwork - Click to discuss the algorithms behind this piece"),
            Mesh3d(meshes.add(Cuboid::new(2.4, 1.8, PAINTING_ART_DEPTH_REGULAR))), // Scaled from (1.6, 1.2)
            MeshMaterial3d(painting_material),
            Transform::from_translation(frame_position + painting_offset).with_rotation(rotation),
            PaintingDialogue {
                node_name: get_dialogue_node_for_painting(name),
            },
        ))
    };

    painting_entity.observe(on_painting_click);
}

fn create_sculpture(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    name: &str,
    position: Vec3,
    sculpture_type: SculptureType,
    museum_materials: &MuseumMaterials,
) {
    match sculpture_type {
        SculptureType::Twisted => {
            let material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.2, 0.2),
                metallic: 0.3,
                perceptual_roughness: 0.4,
                ..default()
            });

            for i in 0..8 {
                let height = i as f32 * 0.225; // Scaled from 0.15 to 0.225 (1.5x)
                let rotation = Quat::from_rotation_y(i as f32 * 0.3);
                let scale_factor = 1.0 - (i as f32 * 0.1);

                commands.spawn((
                    Name::new(format!("{name} Segment {i}")),
                    Mesh3d(meshes.add(Cuboid::new(
                        0.45 * scale_factor,
                        0.225,
                        0.15 * scale_factor,
                    ))), // Scaled dimensions by 1.5x
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(position + Vec3::new(0.0, height, 0.0))
                        .with_rotation(rotation),
                ));
            }
        }
        SculptureType::Geometric => {
            // Use new custom shader material for geometric sculpture
            let material = museum_materials.glowing_sculpture.clone();

            commands.spawn((
                Name::new(format!("{name} Base")),
                Mesh3d(meshes.add(Cuboid::new(1.2, 0.3, 1.2))), // Scaled from (0.8, 0.2, 0.8) by 1.5x
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position),
            ));

            commands.spawn((
                Name::new(format!("{name} Middle")),
                Mesh3d(meshes.add(Sphere::new(0.45))), // Scaled from 0.3 to 0.45 (1.5x)
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position + Vec3::new(0.0, 0.45, 0.0)), // Scaled Y offset from 0.3 to 0.45
                Rotating,
            ));

            commands.spawn((
                Name::new(format!("{name} Top")),
                Mesh3d(meshes.add(Cylinder::new(0.225, 0.6))), // Scaled radius from 0.15 to 0.225, height from 0.4 to 0.6
                MeshMaterial3d(material),
                Transform::from_translation(position + Vec3::new(0.0, 1.05, 0.0)), // Scaled Y offset from 0.7 to 1.05
            ));
        }
        SculptureType::Organic => {
            let material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.2, 0.8),
                metallic: 0.1,
                perceptual_roughness: 0.6,
                ..default()
            });

            for i in 0..5 {
                let angle = (i as f32) * std::f32::consts::PI * 2.0 / 5.0;
                let radius = 0.45 + (i as f32 * 0.15); // Scaled from (0.3 + i * 0.1) to (0.45 + i * 0.15)
                let x = angle.cos() * radius * 0.75; // Scaled multiplier from 0.5 to 0.75
                let z = angle.sin() * radius * 0.75; // Scaled multiplier from 0.5 to 0.75
                let y = i as f32 * 0.15; // Scaled from 0.1 to 0.15

                commands.spawn((
                    Name::new(format!("{name} Flow {i}")),
                    Mesh3d(meshes.add(Sphere::new(0.3 - i as f32 * 0.03))), // Scaled from (0.2 - i * 0.02) to (0.3 - i * 0.03)
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(position + Vec3::new(x, y, z)),
                ));
            }
        }
        SculptureType::Crystal => {
            // Use geometric shader material for magical crystal effect
            let material = MeshMaterial3d(museum_materials.glowing_sculpture.clone());

            for i in 0..6 {
                let angle = (i as f32) * std::f32::consts::PI / 3.0;
                let x = angle.cos() * 0.45; // Scaled from 0.3 to 0.45 (1.5x)
                let z = angle.sin() * 0.45; // Scaled from 0.3 to 0.45 (1.5x)
                let height = 0.75 + (i as f32 % 3.0) * 0.3; // Scaled from (0.5 + i * 0.2) to (0.75 + i * 0.3)

                commands.spawn((
                    Name::new(format!("{name} Crystal {i}")),
                    Mesh3d(meshes.add(Cylinder::new(0.075, height))), // Scaled radius from 0.05 to 0.075
                    material.clone(),
                    Transform::from_translation(position + Vec3::new(x, height / 2.0, z)),
                    Rotating,
                    ColorCyclingSculpture {
                        speed: 0.8,
                        hue_offset: i as f32 * 0.2, // Different hue offset for each crystal
                    },
                ));
            }
        }
    }
}

fn generate_artwork_texture(
    images: &mut ResMut<Assets<Image>>,
    style: PaintingStyle,
    width: u32,
    height: u32,
) -> Handle<Image> {
    match style {
        PaintingStyle::Abstract => generate_abstract_texture(images, width, height),
        PaintingStyle::Geometric => generate_geometric_texture(images, width, height),
        PaintingStyle::ColorField => generate_colorfield_texture(images, width, height),
        PaintingStyle::Organic => generate_organic_texture(images, width, height),
        PaintingStyle::Fractal => generate_fractal_texture(images, width, height),
        PaintingStyle::Minimalist => generate_minimalist_texture(images, width, height),
        PaintingStyle::Digital => generate_digital_texture(images, width, height),
        PaintingStyle::Noise => generate_noise_texture(images, width, height),
        PaintingStyle::Cellular => generate_cellular_texture(images, width, height),
        PaintingStyle::Clouds => generate_clouds_texture(images, width, height),
        PaintingStyle::Marble => generate_marble_art_texture(images, width, height),
        PaintingStyle::Gold => generate_gold_texture(images, width, height),
    }
}

fn generate_abstract_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(1234);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            let noise1 = perlin.get([nx * 5.0, ny * 5.0]);
            let noise2 = perlin.get([nx * 10.0, ny * 10.0]);
            let noise3 = perlin.get([nx * 20.0, ny * 20.0]);

            let combined = noise1 + noise2 * 0.5 + noise3 * 0.25;

            let r = ((combined + 1.0) * 0.5 * 255.0) as u8;
            let g = ((noise2 + 1.0) * 0.5 * 255.0) as u8;
            let b = ((noise3 + 1.0) * 0.5 * 255.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
        }
    }

    create_image(images, data, width, height)
}

fn generate_geometric_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let grid_x = (x / 32) % 2;
            let grid_y = (y / 32) % 2;
            let checkerboard = (grid_x + grid_y) % 2;

            let color = if checkerboard == 0 {
                [200, 100, 50, 255]
            } else {
                [50, 100, 200, 255]
            };

            data.extend_from_slice(&color);
        }
    }

    create_image(images, data, width, height)
}

fn generate_colorfield_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for _x in 0..width {
            let ny = y as f32 / height as f32;

            let r = (255.0 * (1.0 - ny)) as u8;
            let g = (255.0 * ny) as u8;
            let b = (255.0 * (ny * (1.0 - ny) * 4.0)) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
        }
    }

    create_image(images, data, width, height)
}

fn generate_organic_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(5678);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            let organic = perlin.get([nx * 3.0, ny * 3.0]).abs();
            let flow = perlin.get([nx * 8.0 + organic, ny * 8.0 + organic]);

            let intensity = (organic + flow.abs()) * 0.5;

            let r = (intensity * 255.0) as u8;
            let g = (intensity * 180.0) as u8;
            let b = (intensity * 120.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
        }
    }

    create_image(images, data, width, height)
}

fn generate_fractal_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = (x as f32 / width as f32 - 0.5) * 4.0;
            let ny = (y as f32 / height as f32 - 0.5) * 4.0;

            let mut zx = nx;
            let mut zy = ny;
            let mut iterations = 0;
            const MAX_ITERATIONS: u32 = 50;

            while zx * zx + zy * zy < 4.0 && iterations < MAX_ITERATIONS {
                let temp = zx * zx - zy * zy + nx;
                zy = 2.0 * zx * zy + ny;
                zx = temp;
                iterations += 1;
            }

            let color_value = iterations as f32 / MAX_ITERATIONS as f32;

            let r = (color_value * 255.0) as u8;
            let g = ((color_value * 0.5) * 255.0) as u8;
            let b = ((1.0 - color_value) * 255.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
        }
    }

    create_image(images, data, width, height)
}

fn generate_minimalist_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f32 / width as f32;
            let is_line = (nx > 0.3 && nx < 0.7) && (y > height / 3 && y < 2 * height / 3);

            let color = if is_line {
                [20, 20, 20, 255]
            } else {
                [240, 240, 235, 255]
            };

            data.extend_from_slice(&color);
        }
    }

    create_image(images, data, width, height)
}

fn generate_digital_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let bit_x = (x / 8) % 2;
            let bit_y = (y / 8) % 2;
            let intensity = ((x + y) % 64) as f32 / 64.0;

            let r = if bit_x == 1 {
                (255.0 * intensity) as u8
            } else {
                0
            };
            let g = if bit_y == 1 {
                (255.0 * intensity) as u8
            } else {
                0
            };
            let b = if (bit_x + bit_y) % 2 == 1 {
                (255.0 * intensity) as u8
            } else {
                0
            };
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
        }
    }

    create_image(images, data, width, height)
}

fn generate_noise_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(9999);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            let noise = perlin.get([nx * 50.0, ny * 50.0]);
            let intensity = ((noise + 1.0) * 0.5 * 255.0) as u8;

            data.extend_from_slice(&[intensity, intensity, intensity, 255]);
        }
    }

    create_image(images, data, width, height)
}

fn generate_cellular_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let cell_size = 16;
            let cell_x = x / cell_size;
            let cell_y = y / cell_size;

            // Use saturating arithmetic to prevent overflow with high resolution textures
            let hash_value =
                (cell_x.saturating_add(cell_y.saturating_mul(13))).saturating_mul(1234567);
            let alive = hash_value % 100 < 30;

            let color = if alive {
                [255, 100, 100, 255]
            } else {
                [100, 100, 255, 255]
            };

            data.extend_from_slice(&color);
        }
    }

    create_image(images, data, width, height)
}

fn generate_clouds_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(4567);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            let cloud1 = perlin.get([nx * 4.0, ny * 4.0]);
            let cloud2 = perlin.get([nx * 8.0, ny * 8.0]) * 0.5;
            let cloud3 = perlin.get([nx * 16.0, ny * 16.0]) * 0.25;

            let density = (cloud1 + cloud2 + cloud3 + 1.0) * 0.5;
            let intensity = density.clamp(0.0, 1.0);

            let r = (200.0 + intensity * 55.0) as u8;
            let g = (220.0 + intensity * 35.0) as u8;
            let b = (255.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
        }
    }

    create_image(images, data, width, height)
}

fn generate_marble_art_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(7890);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            let vein1 = perlin.get([nx * 6.0, ny * 2.0]);
            let vein2 = perlin.get([nx * 12.0, ny * 4.0]) * 0.5;
            let texture = perlin.get([nx * 20.0, ny * 20.0]) * 0.1;

            let marble = (vein1 + vein2 + texture + 1.0) * 0.5;

            let r = (marble * 180.0 + 75.0) as u8;
            let g = (marble * 160.0 + 95.0) as u8;
            let b = (marble * 140.0 + 115.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
        }
    }

    create_image(images, data, width, height)
}

fn generate_gold_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let perlin = Perlin::new(12345);
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;

            // Create gold-like metallic patterns
            let base_noise = perlin.get([nx * 8.0, ny * 8.0]);
            let fine_detail = perlin.get([nx * 32.0, ny * 32.0]) * 0.3;
            let metallic_sheen = perlin.get([nx * 4.0, ny * 16.0]) * 0.4;

            let gold_pattern = (base_noise + fine_detail + metallic_sheen + 1.0) * 0.5;

            // Gold color palette
            let r = (gold_pattern * 100.0 + 155.0) as u8;
            let g = (gold_pattern * 80.0 + 140.0) as u8;
            let b = (gold_pattern * 30.0 + 20.0) as u8;
            let a = 255u8;

            data.extend_from_slice(&[r, g, b, a]);
        }
    }

    create_image(images, data, width, height)
}

fn create_holographic_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
) -> Handle<StandardMaterial> {
    let holographic_texture = generate_holographic_texture(images, 512, 512);

    materials.add(StandardMaterial {
        base_color_texture: Some(holographic_texture),
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.8),
        metallic: 0.9,
        perceptual_roughness: 0.0,
        alpha_mode: AlphaMode::Blend,
        emissive: LinearRgba::rgb(0.5, 0.5, 1.0),
        ..default()
    })
}

fn create_crystal_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Color::srgba(0.8, 0.8, 1.0, 0.6),
        metallic: 0.0,
        perceptual_roughness: 0.0,
        reflectance: 0.9,
        alpha_mode: AlphaMode::Blend,
        emissive: LinearRgba::rgb(0.1, 0.0, 0.2),
        ior: 1.5,
        ..default()
    })
}

fn create_liquid_metal_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.9, 0.95),
        metallic: 1.0,
        perceptual_roughness: 0.0,
        reflectance: 1.0,
        emissive: LinearRgba::rgb(0.1, 0.1, 0.15),
        ..default()
    })
}

fn create_energy_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 1.0, 0.5, 0.7),
        metallic: 0.0,
        perceptual_roughness: 0.1,
        alpha_mode: AlphaMode::Add,
        emissive: LinearRgba::rgb(0.0, 2.0, 1.0),
        unlit: true,
        ..default()
    })
}

fn create_neon_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.2, 0.8),
        metallic: 0.0,
        perceptual_roughness: 0.2,
        emissive: LinearRgba::rgb(2.0, 0.4, 1.6),
        emissive_exposure_weight: 0.8,
        ..default()
    })
}

fn create_orbiting_element_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    index: usize,
    museum_assets: &Res<MuseumAssets>,
) -> Handle<StandardMaterial> {
    let hue = (index as f32 * 60.0) % 360.0;
    let color = Color::hsl(hue, 0.8, 0.6);

    // Use the preloaded wavy texture from MuseumAssets
    let wavy_texture = museum_assets.wavy_texture.clone();

    materials.add(StandardMaterial {
        base_color: color,
        base_color_texture: Some(wavy_texture),
        metallic: 0.7,
        perceptual_roughness: 0.2,
        emissive: LinearRgba::from(color) * 0.2,
        ..default()
    })
}

fn generate_holographic_texture(
    images: &mut ResMut<Assets<Image>>,
    width: u32,
    height: u32,
) -> Handle<Image> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let nx = x as f32 / width as f32;
            let ny = y as f32 / height as f32;

            let interference = ((nx * 50.0).sin() * (ny * 50.0).sin() + 1.0) * 0.5;
            let rainbow = ((nx + ny) * std::f32::consts::PI).sin().abs();

            let r = (interference * rainbow * 255.0) as u8;
            let g = (interference * (1.0 - rainbow) * 255.0) as u8;
            let b = (interference * 255.0) as u8;
            let a = (interference * 200.0 + 55.0) as u8;

            data.extend_from_slice(&[r, g, b, a]);
        }
    }

    create_image(images, data, width, height)
}

fn create_image(
    images: &mut ResMut<Assets<Image>>,
    data: Vec<u8>,
    width: u32,
    height: u32,
) -> Handle<Image> {
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

fn on_sphere_click(
    _click: On<Pointer<Click>>,
    mut material_cyclers: Query<(&mut MeshMaterial3d<StandardMaterial>, &mut MaterialCycler)>,
) {
    if let Ok((mut material_component, mut cycler)) =
        material_cyclers.get_mut(_click.event().entity)
    {
        // Cycle to the next material
        cycler.current_index = (cycler.current_index + 1) % cycler.materials.len();
        material_component.0 = cycler.materials[cycler.current_index].clone();
    }
}

fn on_painting_click(
    _click: On<Pointer<Click>>,
    mut commands: Commands,
    project: Res<YarnProject>,
    painting_query: Query<&PaintingDialogue>,
    existing_runners: Query<&DialogueRunner>,
) {
    if let Ok(painting_dialogue) = painting_query.get(_click.event().entity) {
        // Check if any dialogue is already running to prevent crashes and overlapping conversations
        for dialogue_runner in existing_runners.iter() {
            if dialogue_runner.is_running() {
                // Already in a conversation, ignore the click
                return;
            }
        }

        // No active dialogue found, safe to start a new one
        let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
        dialogue_runner.start_node(&painting_dialogue.node_name);
        commands.spawn(dialogue_runner);
    }
}

/// Cleans up DialogueRunner entities that have finished their conversations
/// This prevents multiple DialogueRunner entities from accumulating in the world
/// which can cause crashes when starting new conversations
pub fn cleanup_finished_dialogue_runners(
    mut commands: Commands,
    dialogue_runners: Query<(Entity, &DialogueRunner)>,
) {
    for (entity, dialogue_runner) in dialogue_runners.iter() {
        if !dialogue_runner.is_running() {
            // This DialogueRunner has finished its conversation, clean it up
            commands.entity(entity).despawn();
        }
    }
}

fn get_dialogue_node_for_painting(painting_name: &str) -> String {
    match painting_name {
        "Abstract Composition #1" => "AbstractComposition1",
        "Geometric Harmony" => "GeometricHarmony",
        "Color Study #47" => "ColorStudy47",
        "Organic Forms" => "OrganicForms",
        "Fractal Dreams" => "FractalDreams",
        "Minimalist Study" => "MinimalistStudy",
        "Digital Landscape" => "DigitalLandscape",
        "Noise Patterns" => "NoisePatterns",
        "Cellular Automata" => "CellularAutomata",
        "Wave Function" => "WaveFunction",
        "Perlin Clouds" => "PerlinClouds",
        "Marble Veins" => "MarbleVeins",
        _ => {
            warn!("No dialogue node found for painting: {}", painting_name);
            "FractalDreams" // Fallback to existing node
        }
    }
    .to_string()
}

// Function to create sculptures inside the second room display cases
#[allow(clippy::too_many_arguments)] // Function needs many shader material asset collections
pub fn place_second_room_display_case_sculptures(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    animated_materials: &mut ResMut<Assets<AnimatedMaterial>>,
    holographic_materials: &mut ResMut<Assets<HolographicMaterial>>,
    portal_materials: &mut ResMut<Assets<PortalMaterial>>,
    energy_materials: &mut ResMut<Assets<EnergyFieldMaterial>>,
    liquid_materials: &mut ResMut<Assets<LiquidMetalMaterial>>,
    constellation_materials: &mut ResMut<Assets<ConstellationMaterial>>,
    museum_materials: &MuseumMaterials,
    parent: Entity,
) {
    // Sculptures positioned inside the second room display cases on pedestals
    // The pedestals are at y=0.5 with height 1.0, so top is at y=1.0
    // Position sculptures well above the pedestal top to prevent any clipping
    let display_sculptures = [
        (
            "Animated Color Sphere",
            Vec3::new(-7.0, 1.5, -7.0), // Northwest pedestal - raised from 1.35 to 1.5
            DisplaySculptureType::AnimatedSphere,
        ),
        (
            "Holographic Crystal",
            Vec3::new(7.0, 1.5, -7.0), // Northeast pedestal - raised from 1.35 to 1.5
            DisplaySculptureType::HolographicCrystal,
        ),
        (
            "Portal Gateway",
            Vec3::new(-7.0, 1.5, 7.0), // Southwest pedestal - raised from 1.35 to 1.5
            DisplaySculptureType::PortalDisc,
        ),
        (
            "Energy Field Torus",
            Vec3::new(7.0, 1.5, 7.0), // Southeast pedestal - raised from 1.35 to 1.5
            DisplaySculptureType::EnergyTorus,
        ),
    ];

    for (name, position, sculpture_type) in display_sculptures {
        create_display_case_sculpture(
            commands,
            meshes,
            materials,
            animated_materials,
            holographic_materials,
            portal_materials,
            energy_materials,
            liquid_materials,
            constellation_materials,
            name,
            position,
            sculpture_type,
            museum_materials,
            parent,
        );
    }

    // Add liquid metal cube to display case 3 (southwest pedestal)
    let additional_sculptures = [(
        "Liquid Metal Cube",
        Vec3::new(-7.0, 1.5, 7.0), // Inside display case 3 (southwest pedestal)
        DisplaySculptureType::LiquidMetalCube,
    )];

    for (name, position, sculpture_type) in additional_sculptures {
        create_display_case_sculpture(
            commands,
            meshes,
            materials,
            animated_materials,
            holographic_materials,
            portal_materials,
            energy_materials,
            liquid_materials,
            constellation_materials,
            name,
            position,
            sculpture_type,
            museum_materials,
            parent,
        );
    }

    // Create larger constellation sphere with physics on central pedestal
    let constellation_material = crate::shader_materials::create_constellation_material(
        constellation_materials,
        Color::srgb(1.0, 1.0, 1.0), // Pure white stars for better contrast
        Color::srgb(0.0, 0.0, 0.0), // Not used in new shader
    );
    let central_sculpture = commands
        .spawn((
            Name::new("Central Constellation Sphere"),
            Hint::new("⭐ Constellation Sphere - Observe the twinkling stars and nebulae within"),
            Mesh3d(meshes.add(Sphere::new(1.2))),
            MeshMaterial3d(constellation_material),
            Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)), // On central pedestal
            ColorCyclingSculpture {
                speed: 0.3, // Slower color cycling to not interfere with stars
                hue_offset: 240.0,
            },
            Rotating,
            RigidBody::Dynamic,
            Collider::sphere(1.2), // Match mesh dimensions exactly (radius)
        ))
        .id();
    commands.entity(parent).add_child(central_sculpture);
}

// New sculpture types for display cases using shader materials
#[derive(Clone, Copy)]
enum DisplaySculptureType {
    AnimatedSphere,
    HolographicCrystal,
    PortalDisc,
    EnergyTorus,
    LiquidMetalCube,
}

#[allow(clippy::too_many_arguments)] // Function needs many shader material asset collections
fn create_display_case_sculpture(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    animated_materials: &mut ResMut<Assets<AnimatedMaterial>>,
    holographic_materials: &mut ResMut<Assets<HolographicMaterial>>,
    portal_materials: &mut ResMut<Assets<PortalMaterial>>,
    energy_materials: &mut ResMut<Assets<EnergyFieldMaterial>>,
    liquid_materials: &mut ResMut<Assets<LiquidMetalMaterial>>,
    _constellation_materials: &mut ResMut<Assets<ConstellationMaterial>>,
    name: &str,
    position: Vec3,
    sculpture_type: DisplaySculptureType,
    _museum_materials: &MuseumMaterials,
    parent: Entity,
) {
    match sculpture_type {
        DisplaySculptureType::AnimatedSphere => {
            let animated_material =
                crate::shader_materials::create_animated_material(animated_materials);
            let sculpture = commands
                .spawn((
                    Name::new(name.to_string()),
                    Mesh3d(meshes.add(Sphere::new(0.4))),
                    MeshMaterial3d(animated_material),
                    Transform::from_translation(position),
                    PulsingSculpture {
                        speed: 2.0,
                        scale_range: (0.9, 1.1),
                        phase: 0.0,
                    },
                    Rotating,
                ))
                .id();
            commands.entity(parent).add_child(sculpture);
        }
        DisplaySculptureType::HolographicCrystal => {
            let holographic_material = crate::shader_materials::create_holographic_material(
                holographic_materials,
                Color::srgb(0.0, 0.9, 1.0),
                1.5,
            );
            let sculpture = commands
                .spawn((
                    Name::new(name.to_string()),
                    Mesh3d(meshes.add(Mesh::from(Cylinder::new(0.3, 0.8)))),
                    MeshMaterial3d(holographic_material),
                    Transform::from_translation(position),
                    ColorCyclingSculpture {
                        speed: 1.5,
                        hue_offset: 120.0,
                    },
                    Rotating,
                ))
                .id();
            commands.entity(parent).add_child(sculpture);
        }
        DisplaySculptureType::PortalDisc => {
            let portal_material = crate::shader_materials::create_portal_material(
                portal_materials,
                Color::srgb(1.0, 1.0, 1.0), // Bright center
                Color::srgb(0.2, 0.0, 0.8), // Purple edge
            );
            let sculpture = commands
                .spawn((
                    Name::new(name.to_string()),
                    Mesh3d(meshes.add(Circle::new(0.4).mesh())),
                    MeshMaterial3d(portal_material),
                    Transform::from_translation(position)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                    Rotating,
                ))
                .id();
            commands.entity(parent).add_child(sculpture);
        }
        DisplaySculptureType::EnergyTorus => {
            let energy_material = crate::shader_materials::create_energy_field_material(
                energy_materials,
                Color::srgb(0.0, 0.8, 1.0),
                2.5,
            );
            let sculpture = commands
                .spawn((
                    Name::new(name.to_string()),
                    Mesh3d(meshes.add(Torus::new(0.2, 0.4))),
                    MeshMaterial3d(energy_material),
                    Transform::from_translation(position),
                    PulsingSculpture {
                        speed: 3.0,
                        scale_range: (0.8, 1.2),
                        phase: 1.57, // Start at different phase
                    },
                    Rotating,
                ))
                .id();
            commands.entity(parent).add_child(sculpture);
        }
        DisplaySculptureType::LiquidMetalCube => {
            let liquid_material = crate::shader_materials::create_liquid_metal_material(
                liquid_materials,
                Color::srgb(0.8, 0.8, 0.9),
            );
            let sculpture = commands
                .spawn((
                    Name::new(name.to_string()),
                    Mesh3d(meshes.add(Cuboid::new(0.6, 0.6, 0.6))),
                    MeshMaterial3d(liquid_material),
                    Transform::from_translation(position).with_rotation(Quat::from_euler(
                        EulerRot::XYZ,
                        0.5,
                        0.5,
                        0.0,
                    )),
                    Rotating,
                ))
                .id();
            commands.entity(parent).add_child(sculpture);
        }
    }
}
