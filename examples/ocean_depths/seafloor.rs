//! Seafloor terrain generation using Perlin noise

use avian3d::prelude::*;
use bevy::math::Vec4;
use bevy::mesh::{Indices, VertexAttributeValues};
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

use crate::materials::{MossyRockData, MossyRockMaterial};

// Terrain generation constants
const TERRAIN_SIZE: f32 = 150.0;
const TERRAIN_SUBDIVISIONS: u32 = 80;
const TERRAIN_HEIGHT_SCALE: f64 = 6.0;
const TERRAIN_Y_OFFSET: f32 = -5.0;
const NOISE_SEED: u32 = 42;
const ROCK_COUNT: u32 = 30;

pub struct SeafloorPlugin;

impl Plugin for SeafloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_seafloor);
    }
}

#[derive(Component)]
pub struct Seafloor;

fn spawn_seafloor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rock_materials: ResMut<Assets<MossyRockMaterial>>,
) {
    let perlin = Perlin::new(NOISE_SEED);

    // Create seafloor mesh with undulating terrain
    let mut mesh = Plane3d::default()
        .mesh()
        .size(TERRAIN_SIZE, TERRAIN_SIZE)
        .subdivisions(TERRAIN_SUBDIVISIONS)
        .build();

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for pos in positions.iter_mut() {
            let x = pos[0] as f64;
            let z = pos[2] as f64;

            // Multi-octave noise for natural terrain
            let y = perlin.get([x * 0.03, z * 0.03]) * TERRAIN_HEIGHT_SCALE
                + perlin.get([x * 0.08, z * 0.08]) * (TERRAIN_HEIGHT_SCALE * 0.3)
                + perlin.get([x * 0.15, z * 0.15]) * (TERRAIN_HEIGHT_SCALE * 0.1);

            pos[1] = y as f32;
        }
    }

    mesh.compute_normals();

    // Create trimesh collider from mesh data
    let vertex_positions: Vec<Vec3> = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .and_then(|attr| match attr {
            VertexAttributeValues::Float32x3(positions) => {
                Some(positions.iter().map(|p| Vec3::from_array(*p)).collect())
            }
            _ => None,
        })
        .unwrap_or_default();

    let indices: Vec<[u32; 3]> = mesh
        .indices()
        .map(|indices| match indices {
            Indices::U32(indices) => indices.chunks(3).map(|c| [c[0], c[1], c[2]]).collect(),
            Indices::U16(indices) => indices
                .chunks(3)
                .map(|c| [c[0] as u32, c[1] as u32, c[2] as u32])
                .collect(),
        })
        .unwrap_or_default();

    let collider = Collider::trimesh(vertex_positions, indices);

    // Sandy seafloor material
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.76, 0.70, 0.50), // Sandy beige
            perceptual_roughness: 0.95,
            metallic: 0.0,
            ..default()
        })),
        Transform::from_xyz(0.0, TERRAIN_Y_OFFSET, 0.0),
        RigidBody::Static,
        collider,
        Seafloor,
        Name::new("Seafloor"),
    ));

    // Spawn scattered rocks
    spawn_rocks(&mut commands, &mut meshes, &mut rock_materials, &perlin);
}

fn spawn_rocks(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<MossyRockMaterial>>,
    perlin: &Perlin,
) {
    let rock_mesh = meshes.add(Sphere::new(1.0));

    for _ in 0..ROCK_COUNT {
        let x = (rand::random::<f32>() - 0.5) * 120.0;
        let z = (rand::random::<f32>() - 0.5) * 120.0;

        let terrain_y = perlin.get([x as f64 * 0.03, z as f64 * 0.03]) * TERRAIN_HEIGHT_SCALE
            + perlin.get([x as f64 * 0.08, z as f64 * 0.08]) * (TERRAIN_HEIGHT_SCALE * 0.3);

        let scale = 0.5 + rand::random::<f32>() * 2.0;

        // Each rock gets slightly different moss coverage and color variation
        let moss_amount = 0.3 + rand::random::<f32>() * 0.5;
        let rock_variation = rand::random::<f32>() * 0.1;

        let rock_material = materials.add(MossyRockMaterial {
            data: MossyRockData {
                rock_color: Vec4::new(0.4 + rock_variation, 0.38 + rock_variation, 0.35, 1.0),
                moss_color: Vec4::new(0.15 + rock_variation, 0.4, 0.2, 1.0),
                moss_amount,
                wetness: 0.7 + rand::random::<f32>() * 0.3,
                _padding: 0,
            },
        });

        commands.spawn((
            Mesh3d(rock_mesh.clone()),
            MeshMaterial3d(rock_material),
            Transform::from_xyz(x, terrain_y as f32 + TERRAIN_Y_OFFSET + scale * 0.3, z)
                .with_scale(Vec3::new(
                    scale * (0.8 + rand::random::<f32>() * 0.4),
                    scale * (0.5 + rand::random::<f32>() * 0.5),
                    scale * (0.8 + rand::random::<f32>() * 0.4),
                ))
                .with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    rand::random::<f32>() * 0.3,
                    rand::random::<f32>() * std::f32::consts::TAU,
                    rand::random::<f32>() * 0.3,
                )),
            Collider::sphere(scale),
            RigidBody::Static,
            Name::new("Rock"),
        ));
    }
}
