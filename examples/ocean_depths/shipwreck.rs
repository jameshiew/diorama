//! Ancient shipwreck structure
//!
//! A sunken ship that serves as habitat for sea creatures,
//! with interactive elements and dialogue triggers.

use avian3d::prelude::*;
use bevy::picking::events::{Click, Pointer};
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use diorama::picking::Hint;

use crate::dialogue::{OceanDialogue, start_dialogue, terrain_height_at};

pub struct ShipwreckPlugin;

impl Plugin for ShipwreckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shipwreck);
    }
}

fn spawn_shipwreck(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Position the shipwreck at a dramatic angle
    let wreck_x = -30.0;
    let wreck_z = 25.0;
    let terrain_y = terrain_height_at(wreck_x, wreck_z);
    let wreck_pos = Vec3::new(wreck_x, terrain_y + 1.0, wreck_z);

    // Weathered wood material
    let wood_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.18, 0.12),
        perceptual_roughness: 0.95,
        metallic: 0.0,
        ..default()
    });

    // Rusted metal material
    let rust_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.25, 0.15),
        perceptual_roughness: 0.85,
        metallic: 0.3,
        ..default()
    });

    // Barnacle-covered material
    let barnacle_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.45),
        perceptual_roughness: 1.0,
        metallic: 0.0,
        ..default()
    });

    // Ship rotation (tilted and partially buried)
    let ship_rotation = Quat::from_euler(EulerRot::XYZ, 0.15, 0.8, 0.25);

    // Main hull - elongated box
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(12.0, 3.0, 4.0))),
        MeshMaterial3d(wood_material.clone()),
        Transform::from_translation(wreck_pos).with_rotation(ship_rotation),
        Collider::cuboid(12.0, 3.0, 4.0),
        RigidBody::Static,
        Name::new("Shipwreck Hull"),
    ));

    // Bow section (pointed front)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(3.0, 2.5, 3.0))),
        MeshMaterial3d(wood_material.clone()),
        Transform::from_translation(wreck_pos + ship_rotation * Vec3::new(7.0, 0.5, 0.0))
            .with_rotation(ship_rotation * Quat::from_rotation_y(0.4)),
        Collider::cuboid(3.0, 2.5, 3.0),
        RigidBody::Static,
        Name::new("Shipwreck Bow"),
    ));

    // Stern section
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(4.0, 4.0, 3.5))),
        MeshMaterial3d(wood_material.clone()),
        Transform::from_translation(wreck_pos + ship_rotation * Vec3::new(-6.0, 1.0, 0.0))
            .with_rotation(ship_rotation),
        Collider::cuboid(4.0, 4.0, 3.5),
        RigidBody::Static,
        Name::new("Shipwreck Stern"),
    ));

    // Broken mast (fallen)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.3, 8.0))),
        MeshMaterial3d(wood_material.clone()),
        Transform::from_translation(wreck_pos + Vec3::new(2.0, 2.0, 5.0))
            .with_rotation(Quat::from_rotation_x(1.2) * Quat::from_rotation_z(0.3)),
        Collider::cylinder(0.3, 8.0),
        RigidBody::Static,
        Name::new("Fallen Mast"),
    ));

    // Standing mast stub
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.35, 3.0))),
        MeshMaterial3d(wood_material.clone()),
        Transform::from_translation(wreck_pos + ship_rotation * Vec3::new(0.0, 3.0, 0.0))
            .with_rotation(ship_rotation),
        Collider::cylinder(0.35, 3.0),
        RigidBody::Static,
        Name::new("Mast Stub"),
    ));

    // Anchor
    commands.spawn((
        Mesh3d(meshes.add(Torus::new(0.1, 0.6))),
        MeshMaterial3d(rust_material.clone()),
        Transform::from_translation(wreck_pos + Vec3::new(8.0, -1.0, 3.0))
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        Name::new("Anchor Ring"),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.15, 2.0, 0.15))),
        MeshMaterial3d(rust_material.clone()),
        Transform::from_translation(wreck_pos + Vec3::new(8.0, -2.0, 3.0)),
        Name::new("Anchor Shaft"),
    ));

    // Scattered cargo crates
    let crate_positions = [
        Vec3::new(3.0, 0.5, 6.0),
        Vec3::new(-2.0, 0.3, 7.0),
        Vec3::new(5.0, 0.2, -5.0),
        Vec3::new(-4.0, 0.4, -6.0),
    ];

    for (i, offset) in crate_positions.iter().enumerate() {
        let crate_pos = wreck_pos + *offset;
        let crate_terrain_y = terrain_height_at(crate_pos.x, crate_pos.z);

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 0.8, 0.8))),
            MeshMaterial3d(barnacle_material.clone()),
            Transform::from_xyz(crate_pos.x, crate_terrain_y + 0.5 + offset.y, crate_pos.z)
                .with_rotation(Quat::from_rotation_y(i as f32 * 0.7)),
            Collider::cuboid(1.0, 0.8, 0.8),
            RigidBody::Static,
            Name::new(format!("Cargo Crate {}", i + 1)),
        ));
    }

    // Cannon
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.25, 2.0))),
        MeshMaterial3d(rust_material.clone()),
        Transform::from_translation(wreck_pos + Vec3::new(4.0, -0.5, 4.0)).with_rotation(
            Quat::from_rotation_z(std::f32::consts::FRAC_PI_2) * Quat::from_rotation_y(0.3),
        ),
        Name::new("Cannon"),
    ));

    // Ship's wheel (decorative)
    commands.spawn((
        Mesh3d(meshes.add(Torus::new(0.05, 0.5))),
        MeshMaterial3d(wood_material.clone()),
        Transform::from_translation(wreck_pos + ship_rotation * Vec3::new(-5.0, 3.5, 0.0))
            .with_rotation(ship_rotation * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        Name::new("Ship's Wheel"),
    ));

    // Porthole windows (rusted metal rings)
    for i in 0..4 {
        let porthole_offset = Vec3::new(-3.0 + i as f32 * 2.0, 0.5, 2.1);
        commands.spawn((
            Mesh3d(meshes.add(Torus::new(0.05, 0.3))),
            MeshMaterial3d(rust_material.clone()),
            Transform::from_translation(wreck_pos + ship_rotation * porthole_offset)
                .with_rotation(ship_rotation * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            Name::new(format!("Porthole {}", i + 1)),
        ));
    }

    // Spawn the octopus near the shipwreck
    spawn_octopus(&mut commands, &mut meshes, &mut materials, wreck_pos);

    // Spawn giant clam nearby
    spawn_giant_clam(&mut commands, &mut meshes, &mut materials, wreck_pos);
}

fn spawn_octopus(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    wreck_pos: Vec3,
) {
    let octopus_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.3, 0.4),
        emissive: Color::srgb(0.1, 0.05, 0.06).into(),
        perceptual_roughness: 0.5,
        metallic: 0.1,
        ..default()
    });

    let octopus_pos = wreck_pos + Vec3::new(-3.0, 2.0, 0.0);

    // Octopus body (mantle)
    let mut octopus = commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.8))),
        MeshMaterial3d(octopus_material.clone()),
        Transform::from_translation(octopus_pos).with_scale(Vec3::new(1.0, 0.7, 0.8)),
        Name::new("Octopus"),
        Hint::new("🐙 A wise octopus guards the shipwreck's secrets"),
        OceanDialogue {
            node_name: "Octopus".to_string(),
        },
    ));

    octopus.observe(on_creature_click);

    // Octopus tentacles
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * std::f32::consts::TAU;
        let tentacle_offset = Vec3::new(angle.cos() * 0.6, -0.3, angle.sin() * 0.6);

        commands.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.12, 1.2))),
            MeshMaterial3d(octopus_material.clone()),
            Transform::from_translation(octopus_pos + tentacle_offset).with_rotation(
                Quat::from_rotation_z(0.5 * angle.sin()) * Quat::from_rotation_x(0.8),
            ),
            Name::new(format!("Tentacle {}", i + 1)),
        ));
    }
}

fn spawn_giant_clam(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    wreck_pos: Vec3,
) {
    let clam_x = wreck_pos.x + 10.0;
    let clam_z = wreck_pos.z - 8.0;
    let terrain_y = terrain_height_at(clam_x, clam_z);
    let clam_pos = Vec3::new(clam_x, terrain_y + 0.5, clam_z);

    // Shell exterior
    let shell_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.55, 0.5),
        perceptual_roughness: 0.8,
        metallic: 0.1,
        ..default()
    });

    // Interior iridescent
    let interior_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.85, 0.95),
        emissive: Color::srgb(0.1, 0.08, 0.12).into(),
        perceptual_roughness: 0.2,
        metallic: 0.5,
        ..default()
    });

    // Pearl
    let pearl_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.92, 0.98),
        emissive: Color::srgb(0.15, 0.12, 0.18).into(),
        perceptual_roughness: 0.1,
        metallic: 0.3,
        ..default()
    });

    // Bottom shell
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.2))),
        MeshMaterial3d(shell_material.clone()),
        Transform::from_translation(clam_pos).with_scale(Vec3::new(1.5, 0.4, 1.2)),
        Collider::sphere(1.2),
        RigidBody::Static,
        Name::new("Giant Clam Shell"),
    ));

    // Top shell (slightly open)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.1))),
        MeshMaterial3d(shell_material),
        Transform::from_translation(clam_pos + Vec3::new(0.0, 0.5, -0.2))
            .with_scale(Vec3::new(1.4, 0.35, 1.1))
            .with_rotation(Quat::from_rotation_x(-0.3)),
        Name::new("Giant Clam Top Shell"),
    ));

    // Interior
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.9))),
        MeshMaterial3d(interior_material),
        Transform::from_translation(clam_pos + Vec3::new(0.0, 0.2, 0.0))
            .with_scale(Vec3::new(1.2, 0.25, 0.9)),
        Name::new("Clam Interior"),
    ));

    // Pearl
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.25))),
        MeshMaterial3d(pearl_material),
        Transform::from_translation(clam_pos + Vec3::new(0.0, 0.35, 0.0)),
        Name::new("Pearl"),
    ));

    // Clickable trigger entity
    let mut clam_trigger = commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::from_translation(clam_pos + Vec3::Y * 0.5),
        Name::new("Giant Clam Trigger"),
        Hint::new("🦪 A giant clam with a magnificent pearl - it seems eager to talk!"),
        OceanDialogue {
            node_name: "GiantClam".to_string(),
        },
    ));

    clam_trigger.observe(on_creature_click);
}

fn on_creature_click(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    project: Res<YarnProject>,
    dialogue_query: Query<&OceanDialogue>,
    existing_runners: Query<&DialogueRunner>,
) {
    if let Ok(creature_dialogue) = dialogue_query.get(click.event().entity) {
        start_dialogue(
            &mut commands,
            &project,
            &creature_dialogue.node_name,
            &existing_runners,
        );
    }
}
