use avian3d::prelude::*;
use bevy::mesh::{Indices, VertexAttributeValues};
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_terrain);
    }
}

#[derive(Component)]
pub struct Terrain;

fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 200.0;
    let subdivisions = 100;
    let height_scale = 10.0;
    let perlin = Perlin::new(1);

    // Create a plane mesh
    let mut mesh = Plane3d::default()
        .mesh()
        .size(size, size)
        .subdivisions(subdivisions)
        .build();

    // We need to capture heights for the collider.
    // The plane is centered at 0,0, from -size/2 to size/2.
    // Subdivisions = 100 means 101 vertices along each axis.
    // Step size = size / subdivisions = 200 / 100 = 2.0.

    let mut heights = vec![vec![0.0; subdivisions as usize + 1]; subdivisions as usize + 1];

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for pos in positions.iter_mut() {
            let x = pos[0] as f64;
            let z = pos[2] as f64;

            let y = perlin.get([x * 0.05, z * 0.05]) * height_scale
                + perlin.get([x * 0.1, z * 0.1]) * (height_scale * 0.5);

            pos[1] = y as f32;

            // Map world pos to grid index
            // x goes from -100 to 100.
            // index = (x + 100) / 2
            let grid_x =
                ((x + size as f64 / 2.0) / (size as f64 / subdivisions as f64)).round() as usize;
            let grid_z =
                ((z + size as f64 / 2.0) / (size as f64 / subdivisions as f64)).round() as usize;

            if grid_z < heights.len() && grid_x < heights[0].len() {
                heights[grid_z][grid_x] = y as f32;
            }
        }
    }

    mesh.compute_normals();

    // Create trimesh collider from the mesh data
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

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.5, 0.3),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, -10.0, 0.0),
        RigidBody::Static,
        collider,
        Terrain,
        Name::new("Alien Terrain"),
    ));
}
