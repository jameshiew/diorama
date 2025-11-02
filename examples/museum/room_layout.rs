//! # Museum Room Layout
//!
//! Defines the spatial architecture and structure of the museum.
//!
//! ## Layout Overview
//! - **Main Room**: 30x30 units, ceiling height 6.0 units
//! - **Corridor**: Connects main room to second exhibition room
//! - **Second Room**: 30x30 units, features display cases and shader art
//!
//! ## Entity Hierarchy
//! ```text
//! Museum Root
//! ├── Main Room
//! │   ├── Room Structure (floor, walls, ceiling)
//! │   ├── Entrance
//! │   └── Display Areas
//! ├── Corridor
//! │   └── Corridor Structure
//! └── Second Room
//!     ├── Room Structure
//!     ├── Display Cases (4 with pedestals)
//!     ├── Central Pedestal
//!     └── Shader Artwork Panels
//! ```
//!
//! ## Physics
//! All architectural elements have:
//! - `RigidBody::Static` for immovability
//! - `Collider` matching mesh dimensions exactly
//! - Proper clearances to prevent z-fighting
//!
//! ## Design Considerations
//! - Wall thickness: 0.3 units for structural appearance
//! - Doorway clearances for player movement
//! - Display case glass uses shader material for realism
//! - Pedestals positioned for optimal sculpture viewing

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::helpers::{create_group, spawn_static_cuboid, spawn_static_cylinder};
use crate::materials::MuseumMaterials;
use crate::shader_materials::*;
use crate::{CEILING_HEIGHT, WALL_THICKNESS, artworks};

/// Build the main room structure with proper entity hierarchy
#[allow(clippy::too_many_arguments)] // Function needs many shader material asset collections
pub fn build_room(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    standard_materials: &mut ResMut<Assets<StandardMaterial>>,
    animated_materials: &mut ResMut<Assets<AnimatedMaterial>>,
    holographic_materials: &mut ResMut<Assets<HolographicMaterial>>,
    portal_materials: &mut ResMut<Assets<PortalMaterial>>,
    energy_materials: &mut ResMut<Assets<EnergyFieldMaterial>>,
    liquid_materials: &mut ResMut<Assets<LiquidMetalMaterial>>,
    constellation_materials: &mut ResMut<Assets<ConstellationMaterial>>,
    morphing_materials: &mut ResMut<Assets<crate::shader_materials::MorphingSculptureMaterial>>,
) {
    // Create museum root entity with proper hierarchy
    let museum_root = commands
        .spawn((
            Name::new("Museum Root"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    // Create main room
    create_main_room(commands, meshes, materials, museum_root);

    // Create corridor connecting to second room
    create_corridor(commands, meshes, materials, museum_root);

    // Create second room
    create_second_room(
        commands,
        meshes,
        materials,
        museum_root,
        standard_materials,
        animated_materials,
        holographic_materials,
        portal_materials,
        energy_materials,
        liquid_materials,
        constellation_materials,
    );

    // Create corridor to third room (branches from second room)
    create_third_room_corridor(commands, meshes, materials, museum_root);

    // Create third room with morphing sculpture
    create_third_room(commands, meshes, materials, museum_root, morphing_materials);
}

fn create_main_room(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    // Create main room root entity
    let room_root = commands
        .spawn((
            Name::new("Main Room"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(room_root);

    // Create organized sub-structures as children
    create_room_structure(commands, meshes, materials, room_root);
    create_entrance(commands, meshes, materials, room_root);
    create_display_areas(commands, meshes, materials, room_root);
}

fn create_room_structure(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    // Create a sub-group for the room structure
    let structure_root = commands
        .spawn((
            Name::new("Room Structure"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(structure_root);

    // Create floor, walls, and ceiling as children of structure
    create_floor(commands, meshes, materials, structure_root);
    create_walls(commands, meshes, materials, structure_root);
    create_ceiling(commands, meshes, materials, structure_root);
}

fn create_floor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    spawn_static_cuboid(
        commands,
        meshes,
        "Room Floor",
        Vec3::new(30.0, 0.15, 30.0), // Scaled from 20x20 to 30x30
        materials.floor.clone(),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Some(parent),
    );
}

fn create_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    // Create walls group
    let walls_root = create_group(commands, "Walls", Some(parent));

    // North wall (back) - with corridor opening
    create_north_wall_sections(commands, meshes, materials, walls_root);

    // East wall (right) - solid wall
    let east_wall_x = 15.0 - WALL_THICKNESS / 2.0; // Scaled from 10.0 to 15.0
    spawn_static_cuboid(
        commands,
        meshes,
        "East Wall",
        Vec3::new(WALL_THICKNESS, CEILING_HEIGHT, 30.0), // Scaled from 20.0 to 30.0
        materials.wall.clone(),
        Transform::from_xyz(east_wall_x, CEILING_HEIGHT / 2.0, 0.0),
        Some(walls_root),
    );

    // West wall (left) - solid wall
    let west_wall_x = -15.0 + WALL_THICKNESS / 2.0; // Scaled from -10.0 to -15.0
    spawn_static_cuboid(
        commands,
        meshes,
        "West Wall",
        Vec3::new(WALL_THICKNESS, CEILING_HEIGHT, 30.0), // Scaled from 20.0 to 30.0
        materials.wall.clone(),
        Transform::from_xyz(west_wall_x, CEILING_HEIGHT / 2.0, 0.0),
        Some(walls_root),
    );

    // South wall sections (with entrance gap) - create as a group
    create_south_wall_sections(commands, meshes, materials, walls_root);
}

fn create_south_wall_sections(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    let south_wall_root = create_group(commands, "South Wall", Some(parent));

    // Left section
    spawn_static_cuboid(
        commands,
        meshes,
        "South Wall Left",
        Vec3::new(9.0, CEILING_HEIGHT, WALL_THICKNESS), // Scaled from 6.0 to 9.0
        materials.wall.clone(),
        Transform::from_xyz(-10.5, CEILING_HEIGHT / 2.0, 15.0 - WALL_THICKNESS / 2.0), // Scaled from -7.0 to -10.5, 10.0 to 15.0
        Some(south_wall_root),
    );

    // Right section
    spawn_static_cuboid(
        commands,
        meshes,
        "South Wall Right",
        Vec3::new(9.0, CEILING_HEIGHT, WALL_THICKNESS), // Scaled from 6.0 to 9.0
        materials.wall.clone(),
        Transform::from_xyz(10.5, CEILING_HEIGHT / 2.0, 15.0 - WALL_THICKNESS / 2.0), // Scaled from 7.0 to 10.5, 10.0 to 15.0
        Some(south_wall_root),
    );
}

fn create_ceiling(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    spawn_static_cuboid(
        commands,
        meshes,
        "Room Ceiling",
        Vec3::new(30.0, 0.15, 30.0), // Scaled from 20x20 to 30x30
        materials.ceiling.clone(),
        Transform::from_xyz(0.0, CEILING_HEIGHT, 0.0),
        Some(parent),
    );
}

fn create_entrance(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    // Create entrance area group
    let entrance_root = create_group(commands, "Entrance Area", Some(parent));

    // Create entrance platform
    spawn_static_cuboid(
        commands,
        meshes,
        "Entrance Platform",
        Vec3::new(6.0, 1., 3.0), // Scaled from 4x2 to 6x3
        materials.floor.clone(),
        Transform::from_xyz(0., 0., 18.0), // Scaled from 12.0 to 18.0
        Some(entrance_root),
    );

    // Create entrance pillars group
    let pillars_root = create_group(commands, "Entrance Pillars", Some(entrance_root));

    // Left and right pillars
    for (name, x) in [
        ("Entrance Pillar Left", -3.0),
        ("Entrance Pillar Right", 3.0),
    ] {
        spawn_static_cylinder(
            commands,
            meshes,
            name,
            0.45, // Scaled radius from 0.3 to 0.45
            CEILING_HEIGHT,
            materials.pedestal_marble.clone(),
            Transform::from_xyz(x, CEILING_HEIGHT / 2.0, 15.0), // Scaled from ±2.0, 10.0 to ±3.0, 15.0
            Some(pillars_root),
        );
    }
}

fn create_display_areas(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    // Create display areas group
    let display_root = commands
        .spawn((
            Name::new("Display Areas"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(display_root);

    // Central display island
    let central_island = commands
        .spawn((
            Name::new("Central Display Island"),
            Mesh3d(meshes.add(Cylinder::new(3.0, 0.3))), // Scaled radius from 2.0 to 3.0, height from 0.2 to 0.3
            MeshMaterial3d(materials.pedestal_marble.clone()),
            Transform::from_xyz(0.0, 0.15, 0.0), // Scaled Y from 0.1 to 0.15
            RigidBody::Static,
            Collider::cylinder(3.0, 0.3), // Match mesh dimensions exactly (radius, height)
        ))
        .id();
    commands.entity(display_root).add_child(central_island);

    // Information kiosks using brushed steel
    create_information_kiosks(commands, meshes, materials, display_root);

    // Create corner pedestals group
    create_corner_pedestals(commands, meshes, materials, display_root);

    // Create wall mount points group
    create_wall_mount_points(commands, display_root);

    // Add decorative stone elements
    create_decorative_stone_elements(commands, meshes, materials, display_root);
}

fn create_corner_pedestals(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    let pedestals_root = commands
        .spawn((
            Name::new("Corner Pedestals"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(pedestals_root);

    let pedestal_positions = [
        Vec3::new(-10.5, 0.6, -10.5), // Scaled from (-7.0, 0.4, -7.0)
        Vec3::new(10.5, 0.6, -10.5),  // Scaled from (7.0, 0.4, -7.0)
        Vec3::new(-10.5, 0.6, 10.5),  // Scaled from (-7.0, 0.4, 7.0)
        Vec3::new(10.5, 0.6, 10.5),   // Scaled from (7.0, 0.4, 7.0)
    ];

    for (i, position) in pedestal_positions.iter().enumerate() {
        // Use marble pedestal material for all corner pedestals
        let material = materials.pedestal_marble.clone();

        let pedestal = commands
            .spawn((
                Name::new(format!("Corner Pedestal {}", i + 1)),
                Mesh3d(meshes.add(Cylinder::new(1.2, 1.2))), // Scaled radius and height from 0.8 to 1.2
                MeshMaterial3d(material),
                Transform::from_translation(*position),
                RigidBody::Static,
                Collider::cylinder(1.2, 1.2), // Match mesh dimensions exactly (radius, height)
            ))
            .id();
        commands.entity(pedestals_root).add_child(pedestal);
    }
}

fn create_wall_mount_points(commands: &mut Commands, parent: Entity) {
    let wall_mounts_root = commands
        .spawn((
            Name::new("Wall Mount Points"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(wall_mounts_root);

    let wall_mount_positions = [
        // North wall - scaled from ±6, ±2 to ±9, ±3, Z from -9.8 to -14.7
        Vec3::new(-9.0, 3.0, -14.7),
        Vec3::new(-3.0, 3.0, -14.7),
        Vec3::new(3.0, 3.0, -14.7),
        Vec3::new(9.0, 3.0, -14.7),
        // East wall - scaled X from 9.8 to 14.7, Z positions from ±6, ±2 to ±9, ±3
        Vec3::new(14.7, 3.0, -9.0),
        Vec3::new(14.7, 3.0, -3.0),
        Vec3::new(14.7, 3.0, 3.0),
        Vec3::new(14.7, 3.0, 9.0),
        // West wall - scaled X from -9.8 to -14.7, Z positions from ±6, ±2 to ±9, ±3
        Vec3::new(-14.7, 3.0, -9.0),
        Vec3::new(-14.7, 3.0, -3.0),
        Vec3::new(-14.7, 3.0, 3.0),
        Vec3::new(-14.7, 3.0, 9.0),
    ];

    for (i, position) in wall_mount_positions.iter().enumerate() {
        let mount_point = commands
            .spawn((
                Name::new(format!("Wall Mount Point {}", i + 1)),
                Transform::from_translation(*position),
            ))
            .id();
        commands.entity(wall_mounts_root).add_child(mount_point);
    }
}

fn create_north_wall_sections(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    let north_wall_root = create_group(commands, "North Wall", Some(parent));

    // Left section (west side)
    spawn_static_cuboid(
        commands,
        meshes,
        "North Wall Left",
        Vec3::new(9.0, CEILING_HEIGHT, WALL_THICKNESS),
        materials.wall.clone(),
        Transform::from_xyz(-10.5, CEILING_HEIGHT / 2.0, -15.0 + WALL_THICKNESS / 2.0),
        Some(north_wall_root),
    );

    // Right section (east side)
    spawn_static_cuboid(
        commands,
        meshes,
        "North Wall Right",
        Vec3::new(9.0, CEILING_HEIGHT, WALL_THICKNESS),
        materials.wall.clone(),
        Transform::from_xyz(10.5, CEILING_HEIGHT / 2.0, -15.0 + WALL_THICKNESS / 2.0),
        Some(north_wall_root),
    );
}

fn create_corridor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    // Create corridor root entity
    let corridor_root = commands
        .spawn((
            Name::new("Corridor"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(corridor_root);

    // Corridor dimensions
    let corridor_length = 20.0;
    let corridor_width = 12.0;
    let corridor_center_z = -15.0 - corridor_length / 2.0; // Extending north from main room

    // Create corridor floor
    let corridor_floor = commands
        .spawn((
            Name::new("Corridor Floor"),
            Mesh3d(meshes.add(Cuboid::new(corridor_width, 0.15, corridor_length))),
            MeshMaterial3d(materials.floor.clone()),
            Transform::from_xyz(0.0, 0.0, corridor_center_z),
            RigidBody::Static,
            Collider::cuboid(corridor_width, 0.15, corridor_length), // Match mesh dimensions exactly
        ))
        .id();
    commands.entity(corridor_root).add_child(corridor_floor);

    // Create corridor ceiling
    let corridor_ceiling = commands
        .spawn((
            Name::new("Corridor Ceiling"),
            Mesh3d(meshes.add(Cuboid::new(corridor_width, 0.15, corridor_length))),
            MeshMaterial3d(materials.ceiling.clone()),
            Transform::from_xyz(0.0, CEILING_HEIGHT, corridor_center_z),
            RigidBody::Static,
            Collider::cuboid(corridor_width, 0.15, corridor_length), // Match mesh dimensions exactly
        ))
        .id();
    commands.entity(corridor_root).add_child(corridor_ceiling);

    // Create corridor walls
    create_corridor_walls(
        commands,
        meshes,
        materials,
        corridor_root,
        corridor_center_z,
        corridor_length,
        corridor_width,
    );
}

fn create_corridor_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
    corridor_center_z: f32,
    corridor_length: f32,
    corridor_width: f32,
) {
    // Left wall (west)
    let left_wall = commands
        .spawn((
            Name::new("Corridor Left Wall"),
            Mesh3d(meshes.add(Cuboid::new(WALL_THICKNESS, CEILING_HEIGHT, corridor_length))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(
                -corridor_width / 2.0 + WALL_THICKNESS / 2.0,
                CEILING_HEIGHT / 2.0,
                corridor_center_z,
            ),
            RigidBody::Static,
            Collider::cuboid(
                WALL_THICKNESS,
                CEILING_HEIGHT,
                corridor_length, // Match mesh dimensions exactly
            ),
        ))
        .id();
    commands.entity(parent).add_child(left_wall);

    // Right wall (east)
    let right_wall = commands
        .spawn((
            Name::new("Corridor Right Wall"),
            Mesh3d(meshes.add(Cuboid::new(WALL_THICKNESS, CEILING_HEIGHT, corridor_length))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(
                corridor_width / 2.0 - WALL_THICKNESS / 2.0,
                CEILING_HEIGHT / 2.0,
                corridor_center_z,
            ),
            RigidBody::Static,
            Collider::cuboid(
                WALL_THICKNESS,
                CEILING_HEIGHT,
                corridor_length, // Match mesh dimensions exactly
            ),
        ))
        .id();
    commands.entity(parent).add_child(right_wall);
}

#[allow(clippy::too_many_arguments)] // Function needs many shader material asset collections
fn create_second_room(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
    standard_materials: &mut ResMut<Assets<StandardMaterial>>,
    animated_materials: &mut ResMut<Assets<AnimatedMaterial>>,
    holographic_materials: &mut ResMut<Assets<HolographicMaterial>>,
    portal_materials: &mut ResMut<Assets<PortalMaterial>>,
    energy_materials: &mut ResMut<Assets<EnergyFieldMaterial>>,
    liquid_materials: &mut ResMut<Assets<LiquidMetalMaterial>>,
    constellation_materials: &mut ResMut<Assets<ConstellationMaterial>>,
) {
    // Create second room root entity
    let room_root = commands
        .spawn((
            Name::new("Second Room"),
            Transform::from_xyz(0.0, 0.0, -45.0), // Position second room north of corridor
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(room_root);

    // Create second room structure (similar to main room but smaller)
    create_second_room_structure(commands, meshes, materials, room_root);
    create_second_room_display_areas(
        commands,
        meshes,
        materials,
        room_root,
        standard_materials,
        animated_materials,
        holographic_materials,
        portal_materials,
        energy_materials,
        liquid_materials,
        constellation_materials,
    );
}

fn create_second_room_structure(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    let structure_root = commands
        .spawn((
            Name::new("Second Room Structure"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(structure_root);

    // Room dimensions (smaller than main room)
    let room_size = 20.0;

    // Create floor
    let floor = commands
        .spawn((
            Name::new("Second Room Floor"),
            Mesh3d(meshes.add(Cuboid::new(room_size, 0.15, room_size))),
            MeshMaterial3d(materials.floor.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            RigidBody::Static,
            Collider::cuboid(room_size, 0.15, room_size), // Match mesh dimensions exactly
        ))
        .id();
    commands.entity(structure_root).add_child(floor);

    // Create ceiling
    let ceiling = commands
        .spawn((
            Name::new("Second Room Ceiling"),
            Mesh3d(meshes.add(Cuboid::new(room_size, 0.15, room_size))),
            MeshMaterial3d(materials.ceiling.clone()),
            Transform::from_xyz(0.0, CEILING_HEIGHT, 0.0),
            RigidBody::Static,
            Collider::cuboid(room_size, 0.15, room_size), // Match mesh dimensions exactly
        ))
        .id();
    commands.entity(structure_root).add_child(ceiling);

    // Create walls
    create_second_room_walls(commands, meshes, materials, structure_root, room_size);
}

fn create_second_room_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
    room_size: f32,
) {
    let half_size = room_size / 2.0;

    // North wall (solid)
    let north_wall = commands
        .spawn((
            Name::new("Second Room North Wall"),
            Mesh3d(meshes.add(Cuboid::new(room_size, CEILING_HEIGHT, WALL_THICKNESS))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(0.0, CEILING_HEIGHT / 2.0, -half_size + WALL_THICKNESS / 2.0),
            RigidBody::Static,
            Collider::cuboid(room_size, CEILING_HEIGHT, WALL_THICKNESS), // Match mesh dimensions exactly
        ))
        .id();
    commands.entity(parent).add_child(north_wall);

    // East wall (with corridor opening to third room) - create sections
    create_second_room_east_wall_sections(commands, meshes, materials, parent, room_size);

    // West wall (solid)
    let west_wall = commands
        .spawn((
            Name::new("Second Room West Wall"),
            Mesh3d(meshes.add(Cuboid::new(WALL_THICKNESS, CEILING_HEIGHT, room_size))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(-half_size + WALL_THICKNESS / 2.0, CEILING_HEIGHT / 2.0, 0.0),
            RigidBody::Static,
            Collider::cuboid(WALL_THICKNESS, CEILING_HEIGHT, room_size), // Match mesh dimensions exactly
        ))
        .id();
    commands.entity(parent).add_child(west_wall);

    // South wall (with corridor opening) - create sections
    create_second_room_south_wall_sections(commands, meshes, materials, parent, room_size);
}

fn create_second_room_south_wall_sections(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
    room_size: f32,
) {
    let half_size = room_size / 2.0;
    let corridor_opening_width = 12.0;
    let wall_section_width = (room_size - corridor_opening_width) / 2.0;

    // Left section
    let left_section = commands
        .spawn((
            Name::new("Second Room South Wall Left"),
            Mesh3d(meshes.add(Cuboid::new(
                wall_section_width,
                CEILING_HEIGHT,
                WALL_THICKNESS,
            ))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(
                -corridor_opening_width / 2.0 - wall_section_width / 2.0,
                CEILING_HEIGHT / 2.0,
                half_size - WALL_THICKNESS / 2.0,
            ),
            RigidBody::Static,
            Collider::cuboid(
                wall_section_width,
                CEILING_HEIGHT,
                WALL_THICKNESS, // Match mesh dimensions exactly
            ),
        ))
        .id();
    commands.entity(parent).add_child(left_section);

    // Right section
    let right_section = commands
        .spawn((
            Name::new("Second Room South Wall Right"),
            Mesh3d(meshes.add(Cuboid::new(
                wall_section_width,
                CEILING_HEIGHT,
                WALL_THICKNESS,
            ))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(
                corridor_opening_width / 2.0 + wall_section_width / 2.0,
                CEILING_HEIGHT / 2.0,
                half_size - WALL_THICKNESS / 2.0,
            ),
            RigidBody::Static,
            Collider::cuboid(
                wall_section_width,
                CEILING_HEIGHT,
                WALL_THICKNESS, // Match mesh dimensions exactly
            ),
        ))
        .id();
    commands.entity(parent).add_child(right_section);
}

fn create_second_room_east_wall_sections(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
    room_size: f32,
) {
    let half_size = room_size / 2.0;
    let corridor_opening_width = 8.0; // Match the third room corridor width
    let wall_section_height = (room_size - corridor_opening_width) / 2.0;

    // North section (above corridor opening)
    let north_section = commands
        .spawn((
            Name::new("Second Room East Wall North"),
            Mesh3d(meshes.add(Cuboid::new(
                WALL_THICKNESS,
                CEILING_HEIGHT,
                wall_section_height,
            ))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(
                half_size - WALL_THICKNESS / 2.0,
                CEILING_HEIGHT / 2.0,
                -corridor_opening_width / 2.0 - wall_section_height / 2.0,
            ),
            RigidBody::Static,
            Collider::cuboid(WALL_THICKNESS, CEILING_HEIGHT, wall_section_height),
        ))
        .id();
    commands.entity(parent).add_child(north_section);

    // South section (below corridor opening)
    let south_section = commands
        .spawn((
            Name::new("Second Room East Wall South"),
            Mesh3d(meshes.add(Cuboid::new(
                WALL_THICKNESS,
                CEILING_HEIGHT,
                wall_section_height,
            ))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(
                half_size - WALL_THICKNESS / 2.0,
                CEILING_HEIGHT / 2.0,
                corridor_opening_width / 2.0 + wall_section_height / 2.0,
            ),
            RigidBody::Static,
            Collider::cuboid(WALL_THICKNESS, CEILING_HEIGHT, wall_section_height),
        ))
        .id();
    commands.entity(parent).add_child(south_section);
}

#[allow(clippy::too_many_arguments)] // Function needs many shader material asset collections
fn create_second_room_display_areas(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
    standard_materials: &mut ResMut<Assets<StandardMaterial>>,
    animated_materials: &mut ResMut<Assets<AnimatedMaterial>>,
    holographic_materials: &mut ResMut<Assets<HolographicMaterial>>,
    portal_materials: &mut ResMut<Assets<PortalMaterial>>,
    energy_materials: &mut ResMut<Assets<EnergyFieldMaterial>>,
    liquid_materials: &mut ResMut<Assets<LiquidMetalMaterial>>,
    constellation_materials: &mut ResMut<Assets<ConstellationMaterial>>,
) {
    let display_root = commands
        .spawn((
            Name::new("Second Room Display Areas"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(display_root);

    // Central pedestal (smaller than main room)
    let central_pedestal = commands
        .spawn((
            Name::new("Second Room Central Pedestal"),
            Mesh3d(meshes.add(Cylinder::new(2.0, 0.25))),
            MeshMaterial3d(materials.pedestal_marble.clone()),
            Transform::from_xyz(0.0, 0.125, 0.0),
            RigidBody::Static,
            Collider::cylinder(2.0, 0.25), // Match mesh dimensions exactly (radius, height)
        ))
        .id();
    commands.entity(display_root).add_child(central_pedestal);

    // Corner pedestals for second room
    let pedestal_positions = [
        Vec3::new(-7.0, 0.5, -7.0),
        Vec3::new(7.0, 0.5, -7.0),
        Vec3::new(-7.0, 0.5, 7.0),
        Vec3::new(7.0, 0.5, 7.0),
    ];

    // Central pedestal for constellation sphere
    let central_pedestal = commands
        .spawn((
            Name::new("Second Room Central Pedestal"),
            Mesh3d(meshes.add(Cylinder::new(1.0, 1.2))), // Slightly smaller than corner pedestals
            MeshMaterial3d(materials.pedestal_marble.clone()),
            Transform::from_translation(Vec3::new(0.0, 0.6, 0.0)), // Center of room
            RigidBody::Static,
            Collider::cylinder(1.0, 1.2), // Match mesh dimensions exactly (radius, height)
        ))
        .id();
    commands.entity(display_root).add_child(central_pedestal);

    for (i, position) in pedestal_positions.iter().enumerate() {
        let pedestal = commands
            .spawn((
                Name::new(format!("Second Room Pedestal {}", i + 1)),
                Mesh3d(meshes.add(Cylinder::new(1.3, 1.0))), // Increased radius from 1.0 to 1.3 for better proportions
                MeshMaterial3d(materials.pedestal_marble.clone()),
                Transform::from_translation(*position),
                RigidBody::Static,
                Collider::cylinder(1.3, 1.0), // Match mesh dimensions exactly (radius, height)
            ))
            .id();
        commands.entity(display_root).add_child(pedestal);
    }

    // Glass display cases on pedestals
    create_second_room_display_cases(
        commands,
        meshes,
        materials,
        display_root,
        &pedestal_positions,
    );

    // Place sculptures inside the display cases (as children of the second room)
    artworks::place_second_room_display_case_sculptures(
        commands,
        meshes,
        standard_materials,
        animated_materials,
        holographic_materials,
        portal_materials,
        energy_materials,
        liquid_materials,
        constellation_materials,
        materials,
        display_root,
    );

    // Wall mount points for second room
    create_second_room_wall_mounts(commands, display_root);
}

fn create_second_room_wall_mounts(commands: &mut Commands, parent: Entity) {
    let wall_mount_positions = [
        // North wall
        Vec3::new(-6.0, 2.5, -9.7),
        Vec3::new(0.0, 2.5, -9.7),
        Vec3::new(6.0, 2.5, -9.7),
        // East wall
        Vec3::new(9.7, 2.5, -6.0),
        Vec3::new(9.7, 2.5, 0.0),
        Vec3::new(9.7, 2.5, 6.0),
        // West wall
        Vec3::new(-9.7, 2.5, -6.0),
        Vec3::new(-9.7, 2.5, 0.0),
        Vec3::new(-9.7, 2.5, 6.0),
    ];

    for (i, position) in wall_mount_positions.iter().enumerate() {
        let mount_point = commands
            .spawn((
                Name::new(format!("Second Room Wall Mount {}", i + 1)),
                Transform::from_translation(*position),
            ))
            .id();
        commands.entity(parent).add_child(mount_point);
    }
}

// New functions for enhanced display features using unused materials

fn create_information_kiosks(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    let kiosks_root = commands
        .spawn((
            Name::new("Information Kiosks"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(kiosks_root);

    // Two interactive kiosks near the entrance
    let kiosk_positions = [
        Vec3::new(-3.0, 1.2, 8.0), // Left of entrance
        Vec3::new(3.0, 1.2, 8.0),  // Right of entrance
    ];

    for (i, position) in kiosk_positions.iter().enumerate() {
        // Kiosk base
        let base = commands
            .spawn((
                Name::new(format!("Info Kiosk {} Base", i + 1)),
                Mesh3d(meshes.add(Cylinder::new(0.4, 1.2))),
                MeshMaterial3d(materials.pedestal_marble.clone()),
                Transform::from_translation(*position - Vec3::new(0.0, 0.6, 0.0)),
                RigidBody::Static,
                Collider::cylinder(0.4, 1.2),
            ))
            .id();
        commands.entity(kiosks_root).add_child(base);

        // Kiosk screen/display panel
        let screen = commands
            .spawn((
                Name::new(format!("Info Kiosk {} Screen", i + 1)),
                Mesh3d(meshes.add(Cuboid::new(0.8, 0.6, 0.1))),
                MeshMaterial3d(materials.polished_stone.clone()),
                Transform::from_translation(*position + Vec3::new(0.0, 0.3, 0.0)),
                RigidBody::Static,
                Collider::cuboid(0.8, 0.6, 0.1),
            ))
            .id();
        commands.entity(kiosks_root).add_child(screen);
    }
}

fn create_decorative_stone_elements(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    let stone_elements_root = commands
        .spawn((
            Name::new("Decorative Stone Elements"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(stone_elements_root);

    // Stone accent pillars in corners
    let pillar_positions = [
        Vec3::new(-13.5, 3.0, -13.5), // Back left corner
        Vec3::new(13.5, 3.0, -13.5),  // Back right corner
        Vec3::new(-13.5, 3.0, 13.5),  // Front left corner
        Vec3::new(13.5, 3.0, 13.5),   // Front right corner
    ];

    for (i, position) in pillar_positions.iter().enumerate() {
        let pillar = commands
            .spawn((
                Name::new(format!("Stone Pillar {}", i + 1)),
                Mesh3d(meshes.add(Cylinder::new(0.5, 6.0))),
                MeshMaterial3d(materials.polished_stone.clone()),
                Transform::from_translation(*position),
                RigidBody::Static,
                Collider::cylinder(0.5, 6.0),
            ))
            .id();
        commands.entity(stone_elements_root).add_child(pillar);
    }

    // Stone benches for visitors to sit and view art
    let bench_positions = [
        Vec3::new(-8.0, 0.4, 6.0),
        Vec3::new(8.0, 0.4, 6.0),
        Vec3::new(0.0, 0.4, -8.0), // Moved further south to avoid overlap with display case
    ];

    for (i, position) in bench_positions.iter().enumerate() {
        let bench = commands
            .spawn((
                Name::new(format!("Stone Bench {}", i + 1)),
                Mesh3d(meshes.add(Cuboid::new(3.0, 0.8, 0.8))),
                MeshMaterial3d(materials.polished_stone.clone()),
                Transform::from_translation(*position),
                RigidBody::Static,
                Collider::cuboid(3.0, 0.8, 0.8),
            ))
            .id();
        commands.entity(stone_elements_root).add_child(bench);
    }
}

fn create_second_room_display_cases(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
    pedestal_positions: &[Vec3],
) {
    let display_cases_root = commands
        .spawn((
            Name::new("Second Room Display Cases"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(display_cases_root);

    // Create display cases on each pedestal
    for (i, pedestal_pos) in pedestal_positions.iter().enumerate() {
        // Calculate position for display case on top of pedestal
        // Pedestals are 1.2 unit tall and positioned at y=0.6, so top is at y=1.2
        let case_position = Vec3::new(
            pedestal_pos.x,
            1.0 + 0.9, // Top of pedestal + half height of case (1.8/2 = 0.9)
            pedestal_pos.z,
        );

        // Glass case walls - sized to fit on pedestals (removed circular base to prevent clipping)
        let glass_case = commands
            .spawn((
                Name::new(format!("Second Room Display Case {} Glass", i + 1)),
                Mesh3d(meshes.add(Cuboid::new(1.4, 1.8, 1.4))), // Smaller to fit on pedestals
                MeshMaterial3d(materials.glass_display_shader.clone()),
                Transform::from_translation(case_position),
                RigidBody::Static,
                Collider::cuboid(1.4, 1.8, 1.4),
            ))
            .id();
        commands.entity(display_cases_root).add_child(glass_case);
    }
}

// ============================================================================
// Third Room (Morphing Sculpture Gallery)
// ============================================================================

fn create_third_room_corridor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
) {
    // Create corridor root entity - branches east from second room
    let corridor_root = commands
        .spawn((
            Name::new("Third Room Corridor"),
            Transform::from_xyz(0.0, 0.0, -45.0), // Same Z as second room center
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(corridor_root);

    // Corridor dimensions - going east
    let corridor_length = 15.0;
    let corridor_width = 8.0;
    let corridor_center_x = 10.0 + corridor_length / 2.0; // Starting from second room east wall

    // Create corridor floor
    let corridor_floor = commands
        .spawn((
            Name::new("Third Room Corridor Floor"),
            Mesh3d(meshes.add(Cuboid::new(corridor_length, 0.15, corridor_width))),
            MeshMaterial3d(materials.floor.clone()),
            Transform::from_xyz(corridor_center_x, 0.0, 0.0),
            RigidBody::Static,
            Collider::cuboid(corridor_length, 0.15, corridor_width),
        ))
        .id();
    commands.entity(corridor_root).add_child(corridor_floor);

    // Create corridor ceiling
    let corridor_ceiling = commands
        .spawn((
            Name::new("Third Room Corridor Ceiling"),
            Mesh3d(meshes.add(Cuboid::new(corridor_length, 0.15, corridor_width))),
            MeshMaterial3d(materials.ceiling.clone()),
            Transform::from_xyz(corridor_center_x, CEILING_HEIGHT, 0.0),
            RigidBody::Static,
            Collider::cuboid(corridor_length, 0.15, corridor_width),
        ))
        .id();
    commands.entity(corridor_root).add_child(corridor_ceiling);

    // North wall
    let north_wall = commands
        .spawn((
            Name::new("Third Room Corridor North Wall"),
            Mesh3d(meshes.add(Cuboid::new(corridor_length, CEILING_HEIGHT, WALL_THICKNESS))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(
                corridor_center_x,
                CEILING_HEIGHT / 2.0,
                -corridor_width / 2.0 + WALL_THICKNESS / 2.0,
            ),
            RigidBody::Static,
            Collider::cuboid(corridor_length, CEILING_HEIGHT, WALL_THICKNESS),
        ))
        .id();
    commands.entity(corridor_root).add_child(north_wall);

    // South wall
    let south_wall = commands
        .spawn((
            Name::new("Third Room Corridor South Wall"),
            Mesh3d(meshes.add(Cuboid::new(corridor_length, CEILING_HEIGHT, WALL_THICKNESS))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(
                corridor_center_x,
                CEILING_HEIGHT / 2.0,
                corridor_width / 2.0 - WALL_THICKNESS / 2.0,
            ),
            RigidBody::Static,
            Collider::cuboid(corridor_length, CEILING_HEIGHT, WALL_THICKNESS),
        ))
        .id();
    commands.entity(corridor_root).add_child(south_wall);
}

fn create_third_room(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
    morphing_materials: &mut ResMut<Assets<crate::shader_materials::MorphingSculptureMaterial>>,
) {
    // Create third room root entity - positioned east of second room
    let room_root = commands
        .spawn((
            Name::new("Third Room - Morphing Sculpture Gallery"),
            Transform::from_xyz(32.5, 0.0, -45.0), // East of corridor
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(room_root);

    // Room dimensions (smaller intimate space)
    let room_size = 15.0;

    // Create room structure
    create_third_room_structure(commands, meshes, materials, room_root, room_size);

    // Create the central morphing sculpture
    create_morphing_sculpture_display(commands, meshes, materials, room_root, morphing_materials);
}

fn create_third_room_structure(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
    room_size: f32,
) {
    let structure_root = commands
        .spawn((
            Name::new("Third Room Structure"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(structure_root);

    let half_size = room_size / 2.0;

    // Create floor
    let floor = commands
        .spawn((
            Name::new("Third Room Floor"),
            Mesh3d(meshes.add(Cuboid::new(room_size, 0.15, room_size))),
            MeshMaterial3d(materials.floor.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            RigidBody::Static,
            Collider::cuboid(room_size, 0.15, room_size),
        ))
        .id();
    commands.entity(structure_root).add_child(floor);

    // Create ceiling
    let ceiling = commands
        .spawn((
            Name::new("Third Room Ceiling"),
            Mesh3d(meshes.add(Cuboid::new(room_size, 0.15, room_size))),
            MeshMaterial3d(materials.ceiling.clone()),
            Transform::from_xyz(0.0, CEILING_HEIGHT, 0.0),
            RigidBody::Static,
            Collider::cuboid(room_size, 0.15, room_size),
        ))
        .id();
    commands.entity(structure_root).add_child(ceiling);

    // North wall (solid)
    let north_wall = commands
        .spawn((
            Name::new("Third Room North Wall"),
            Mesh3d(meshes.add(Cuboid::new(room_size, CEILING_HEIGHT, WALL_THICKNESS))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(0.0, CEILING_HEIGHT / 2.0, -half_size + WALL_THICKNESS / 2.0),
            RigidBody::Static,
            Collider::cuboid(room_size, CEILING_HEIGHT, WALL_THICKNESS),
        ))
        .id();
    commands.entity(structure_root).add_child(north_wall);

    // East wall (solid)
    let east_wall = commands
        .spawn((
            Name::new("Third Room East Wall"),
            Mesh3d(meshes.add(Cuboid::new(WALL_THICKNESS, CEILING_HEIGHT, room_size))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(half_size - WALL_THICKNESS / 2.0, CEILING_HEIGHT / 2.0, 0.0),
            RigidBody::Static,
            Collider::cuboid(WALL_THICKNESS, CEILING_HEIGHT, room_size),
        ))
        .id();
    commands.entity(structure_root).add_child(east_wall);

    // South wall (solid)
    let south_wall = commands
        .spawn((
            Name::new("Third Room South Wall"),
            Mesh3d(meshes.add(Cuboid::new(room_size, CEILING_HEIGHT, WALL_THICKNESS))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(0.0, CEILING_HEIGHT / 2.0, half_size - WALL_THICKNESS / 2.0),
            RigidBody::Static,
            Collider::cuboid(room_size, CEILING_HEIGHT, WALL_THICKNESS),
        ))
        .id();
    commands.entity(structure_root).add_child(south_wall);

    // West wall (with corridor opening) - create sections
    create_third_room_west_wall_sections(commands, meshes, materials, structure_root, room_size);
}

fn create_third_room_west_wall_sections(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
    room_size: f32,
) {
    let half_size = room_size / 2.0;
    let corridor_opening_width = 8.0;
    let wall_section_width = (room_size - corridor_opening_width) / 2.0;

    // North section
    let north_section = commands
        .spawn((
            Name::new("Third Room West Wall North"),
            Mesh3d(meshes.add(Cuboid::new(
                WALL_THICKNESS,
                CEILING_HEIGHT,
                wall_section_width,
            ))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(
                -half_size + WALL_THICKNESS / 2.0,
                CEILING_HEIGHT / 2.0,
                -corridor_opening_width / 2.0 - wall_section_width / 2.0,
            ),
            RigidBody::Static,
            Collider::cuboid(WALL_THICKNESS, CEILING_HEIGHT, wall_section_width),
        ))
        .id();
    commands.entity(parent).add_child(north_section);

    // South section
    let south_section = commands
        .spawn((
            Name::new("Third Room West Wall South"),
            Mesh3d(meshes.add(Cuboid::new(
                WALL_THICKNESS,
                CEILING_HEIGHT,
                wall_section_width,
            ))),
            MeshMaterial3d(materials.wall.clone()),
            Transform::from_xyz(
                -half_size + WALL_THICKNESS / 2.0,
                CEILING_HEIGHT / 2.0,
                corridor_opening_width / 2.0 + wall_section_width / 2.0,
            ),
            RigidBody::Static,
            Collider::cuboid(WALL_THICKNESS, CEILING_HEIGHT, wall_section_width),
        ))
        .id();
    commands.entity(parent).add_child(south_section);
}

fn create_morphing_sculpture_display(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &MuseumMaterials,
    parent: Entity,
    morphing_materials: &mut ResMut<Assets<crate::shader_materials::MorphingSculptureMaterial>>,
) {
    let display_root = commands
        .spawn((
            Name::new("Morphing Sculpture Display"),
            Transform::default(),
            Visibility::default(),
        ))
        .id();
    commands.entity(parent).add_child(display_root);

    // Central pedestal
    let pedestal = commands
        .spawn((
            Name::new("Morphing Sculpture Pedestal"),
            Mesh3d(meshes.add(Cylinder::new(1.5, 1.5))),
            MeshMaterial3d(materials.pedestal_marble.clone()),
            Transform::from_xyz(0.0, 0.75, 0.0),
            RigidBody::Static,
            Collider::cylinder(1.5, 1.5),
        ))
        .id();
    commands.entity(display_root).add_child(pedestal);

    // Create the morphing sculpture with transcendent complexity
    // Material channels the essence of cosmic transformation
    let morphing_material = crate::shader_materials::create_morphing_sculpture_material(
        morphing_materials,
        Color::srgb(0.5, 0.15, 0.85), // Deep mystical purple
        Color::srgb(0.1, 0.95, 0.95), // Electric cyan
        0.9,                          // Slower morph speed for profound contemplation
        6.0,                          // Maximum detail scale for ultimate complexity
    );

    // Create ultra-high detail base mesh for maximum shader complexity
    let core_mesh = meshes.add(Sphere::new(1.2).mesh().ico(6).unwrap()); // Ultra-high detail icosphere

    // === CORE SCULPTURE: The Eternal Tesseract ===
    // Primary form - the heart of reality
    let core_sculpture = commands
        .spawn((
            Name::new("Core: Eternal Tesseract"),
            Mesh3d(core_mesh.clone()),
            MeshMaterial3d(morphing_material.clone()),
            Transform::from_xyz(0.0, 2.5, 0.0),
            crate::artworks::MorphingSculpture {
                speed: 0.6, // Slower, more profound
                amplitude: 0.4,
                base_mesh: core_mesh,
            },
            crate::Rotating,
        ))
        .id();
    commands.entity(display_root).add_child(core_sculpture);

    // === INNER RING: Orbiting Platonic Solids ===
    // Five elements representing the building blocks of reality
    let platonic_configs = [
        (0.5, 6),  // Icosahedron - water/flow
        (0.45, 5), // Dodecahedron - ether/cosmos
        (0.48, 5), // Octahedron - air/intellect
        (0.52, 6), // Tetrahedron - fire/energy
        (0.46, 5), // Cube - earth/foundation
    ];

    for (i, (radius, ico_level)) in platonic_configs.iter().enumerate() {
        let angle = (i as f32) * std::f32::consts::TAU / 5.0;
        let orbit_radius = 2.0;
        let height_variation = (i as f32 * 0.3).sin() * 0.3;

        let x = angle.cos() * orbit_radius;
        let z = angle.sin() * orbit_radius;
        let y = 2.5 + height_variation;

        let platonic_mesh = meshes.add(Sphere::new(*radius).mesh().ico(*ico_level).unwrap());
        let platonic = commands
            .spawn((
                Name::new(format!("Platonic Solid {}", i + 1)),
                Mesh3d(platonic_mesh.clone()),
                MeshMaterial3d(morphing_material.clone()),
                Transform::from_xyz(x, y, z),
                crate::artworks::MorphingSculpture {
                    speed: 1.0 + (i as f32) * 0.2,
                    amplitude: 0.25,
                    base_mesh: platonic_mesh,
                },
                crate::Rotating,
            ))
            .id();
        commands.entity(display_root).add_child(platonic);
    }

    // === OUTER RING: Mandala Fragments ===
    // Eight fragments forming a sacred circle
    for i in 0..8 {
        let angle = (i as f32) * std::f32::consts::TAU / 8.0;
        let orbit_radius = 3.2;
        let x = angle.cos() * orbit_radius;
        let z = angle.sin() * orbit_radius;

        // Alternate between high and low positions
        let y = if i % 2 == 0 { 3.5 } else { 1.5 };

        let fragment_mesh = meshes.add(Sphere::new(0.3).mesh().ico(5).unwrap());
        let fragment = commands
            .spawn((
                Name::new(format!("Mandala Fragment {}", i + 1)),
                Mesh3d(fragment_mesh.clone()),
                MeshMaterial3d(morphing_material.clone()),
                Transform::from_xyz(x, y, z),
                crate::artworks::MorphingSculpture {
                    speed: 1.5 + (i as f32) * 0.15,
                    amplitude: 0.15,
                    base_mesh: fragment_mesh,
                },
                crate::Rotating,
            ))
            .id();
        commands.entity(display_root).add_child(fragment);
    }

    // === VERTICAL AXIS: Above and Below ===
    // Representing ascension and grounding
    let vertical_positions = [
        (0.0, 4.5, 0.0, "Zenith Crown"),
        (0.0, 0.8, 0.0, "Foundation Root"),
    ];

    for (i, (x, y, z, name)) in vertical_positions.iter().enumerate() {
        let vertex_mesh = meshes.add(Sphere::new(0.6).mesh().ico(5).unwrap());
        let vertex = commands
            .spawn((
                Name::new(name.to_string()),
                Mesh3d(vertex_mesh.clone()),
                MeshMaterial3d(morphing_material.clone()),
                Transform::from_xyz(*x, *y, *z),
                crate::artworks::MorphingSculpture {
                    speed: 0.8 + (i as f32) * 0.4,
                    amplitude: 0.2,
                    base_mesh: vertex_mesh,
                },
                crate::Rotating,
            ))
            .id();
        commands.entity(display_root).add_child(vertex);
    }

    // === RESONANCE NODES: Tetrahedral Formation ===
    // Four nodes forming a perfect tetrahedron around the core
    let tetra_radius = 1.8;
    let tetra_height = tetra_radius * (2.0_f32.sqrt() / 3.0_f32.sqrt());

    let tetrahedral_nodes = [
        (tetra_radius, 2.5 + tetra_height, 0.0),
        (
            -tetra_radius * 0.5,
            2.5 - tetra_height / 3.0,
            tetra_radius * 0.866,
        ),
        (
            -tetra_radius * 0.5,
            2.5 - tetra_height / 3.0,
            -tetra_radius * 0.866,
        ),
        (0.0, 2.5 - tetra_height, 0.0),
    ];

    for (i, (x, y, z)) in tetrahedral_nodes.iter().enumerate() {
        let node_mesh = meshes.add(Sphere::new(0.35).mesh().ico(5).unwrap());
        let node = commands
            .spawn((
                Name::new(format!("Resonance Node {}", i + 1)),
                Mesh3d(node_mesh.clone()),
                MeshMaterial3d(morphing_material.clone()),
                Transform::from_xyz(*x, *y, *z),
                crate::artworks::MorphingSculpture {
                    speed: 1.3 + (i as f32) * 0.25,
                    amplitude: 0.18,
                    base_mesh: node_mesh,
                },
                crate::Rotating,
            ))
            .id();
        commands.entity(display_root).add_child(node);
    }
}
