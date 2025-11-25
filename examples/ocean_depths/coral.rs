//! Procedural coral reef generation
//!
//! Creates various types of coral formations:
//! - Branching coral (tree-like structures)
//! - Brain coral (spherical with patterns)
//! - Fan coral (flat, fan-shaped)
//! - Tube coral (cylindrical clusters)
//! - Ancient coral (interactive, with dialogue)

use avian3d::prelude::*;
use bevy::math::Vec4;
use bevy::picking::events::{Click, Pointer};
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use diorama::picking::Hint;

use crate::dialogue::{OceanDialogue, start_dialogue, terrain_height_at};
use crate::materials::{CoralData, CoralMaterial};

pub struct CoralPlugin;

impl Plugin for CoralPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_coral_reef, spawn_ancient_coral))
            .add_systems(Update, sway_coral);
    }
}

/// Marker component for coral entities
#[derive(Component)]
pub struct Coral;

/// Component for coral swaying animation
#[derive(Component)]
pub struct CoralSway {
    pub phase: f32,
    pub amplitude: f32,
}

/// Different coral species with unique visual properties
#[derive(Clone, Copy)]
enum CoralSpecies {
    Branching,
    Brain,
    Fan,
    Tube,
}

impl CoralSpecies {
    fn base_color(&self) -> Vec4 {
        match self {
            CoralSpecies::Branching => Vec4::new(1.0, 0.4, 0.5, 1.0), // Pink
            CoralSpecies::Brain => Vec4::new(0.9, 0.75, 0.3, 1.0),    // Golden
            CoralSpecies::Fan => Vec4::new(0.6, 0.2, 0.8, 1.0),       // Purple
            CoralSpecies::Tube => Vec4::new(0.3, 0.9, 0.7, 1.0),      // Teal
        }
    }

    fn tip_color(&self) -> Vec4 {
        match self {
            CoralSpecies::Branching => Vec4::new(1.0, 0.7, 0.75, 1.0), // Light pink tips
            CoralSpecies::Brain => Vec4::new(1.0, 0.9, 0.5, 1.0),      // Bright golden
            CoralSpecies::Fan => Vec4::new(0.8, 0.5, 1.0, 1.0),        // Light purple
            CoralSpecies::Tube => Vec4::new(0.5, 1.0, 0.9, 1.0),       // Light teal
        }
    }

    fn glow_intensity(&self) -> f32 {
        // Slight bioluminescence
        match self {
            CoralSpecies::Branching => 0.2,
            CoralSpecies::Brain => 0.15,
            CoralSpecies::Fan => 0.3,
            CoralSpecies::Tube => 0.25,
        }
    }
}

fn spawn_coral_reef(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CoralMaterial>>,
) {
    // Create coral reef clusters in different areas
    let reef_centers = [
        Vec3::new(15.0, 0.0, 10.0),
        Vec3::new(-20.0, 0.0, -15.0),
        Vec3::new(5.0, 0.0, -25.0),
        Vec3::new(-10.0, 0.0, 20.0),
        Vec3::new(30.0, 0.0, -5.0),
    ];

    for center in reef_centers {
        spawn_coral_cluster(&mut commands, &mut meshes, &mut materials, center);
    }
}

fn spawn_coral_cluster(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<CoralMaterial>>,
    center: Vec3,
) {
    let coral_count = 15 + (rand::random::<u32>() % 10);

    for _ in 0..coral_count {
        let offset_x = (rand::random::<f32>() - 0.5) * 20.0;
        let offset_z = (rand::random::<f32>() - 0.5) * 20.0;

        let x = center.x + offset_x;
        let z = center.z + offset_z;
        let terrain_y = terrain_height_at(x, z);

        let species = match rand::random::<u32>() % 4 {
            0 => CoralSpecies::Branching,
            1 => CoralSpecies::Brain,
            2 => CoralSpecies::Fan,
            _ => CoralSpecies::Tube,
        };

        spawn_coral(
            commands,
            meshes,
            materials,
            species,
            Vec3::new(x, terrain_y, z),
        );
    }
}

fn spawn_coral(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<CoralMaterial>>,
    species: CoralSpecies,
    position: Vec3,
) {
    let scale = 0.5 + rand::random::<f32>() * 1.5;
    let phase = rand::random::<f32>() * std::f32::consts::TAU;

    let material = materials.add(CoralMaterial {
        data: CoralData {
            base_color: species.base_color(),
            tip_color: species.tip_color(),
            glow_intensity: species.glow_intensity(),
            polyp_density: 15.0 + rand::random::<f32>() * 10.0,
            _padding: 0,
        },
    });

    let (mesh, collider, name, description) = match species {
        CoralSpecies::Branching => {
            // Simplified branching coral as elongated spheres
            let mesh = meshes.add(Capsule3d::new(0.3, 1.5));
            (
                mesh,
                Collider::capsule(0.3 * scale, 1.5 * scale),
                "Branching Coral",
                "Delicate branching coral that sways gently in the current.",
            )
        }
        CoralSpecies::Brain => {
            let mesh = meshes.add(Sphere::new(0.8));
            (
                mesh,
                Collider::sphere(0.8 * scale),
                "Brain Coral",
                "A massive brain coral with intricate grooved patterns.",
            )
        }
        CoralSpecies::Fan => {
            // Fan coral as a flattened cylinder
            let mesh = meshes.add(Cylinder::new(0.8, 0.1));
            (
                mesh,
                Collider::cylinder(0.8 * scale, 0.1 * scale),
                "Sea Fan",
                "A beautiful purple sea fan filtering nutrients from the water.",
            )
        }
        CoralSpecies::Tube => {
            let mesh = meshes.add(Cylinder::new(0.2, 1.2));
            (
                mesh,
                Collider::cylinder(0.2 * scale, 1.2 * scale),
                "Tube Coral",
                "Clusters of tube coral providing shelter for small creatures.",
            )
        }
    };

    let rotation = match species {
        CoralSpecies::Fan => {
            Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)
                * Quat::from_rotation_z(rand::random::<f32>() * std::f32::consts::TAU)
        }
        _ => Quat::from_rotation_y(rand::random::<f32>() * std::f32::consts::TAU),
    };

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position)
            .with_scale(Vec3::splat(scale))
            .with_rotation(rotation),
        collider,
        RigidBody::Static,
        Coral,
        CoralSway {
            phase,
            amplitude: 0.02 + rand::random::<f32>() * 0.03,
        },
        Name::new(name),
        Hint::new(description),
    ));
}

/// Animate coral swaying in the water current
fn sway_coral(time: Res<Time>, mut query: Query<(&mut Transform, &CoralSway), With<Coral>>) {
    let t = time.elapsed_secs();

    for (mut transform, sway) in query.iter_mut() {
        let angle = (t + sway.phase).sin() * sway.amplitude;
        let current_scale = transform.scale;

        // Apply gentle rotation while preserving scale
        transform.rotation = Quat::from_rotation_z(angle) * Quat::from_rotation_x(angle * 0.5);
        transform.scale = current_scale;
    }
}

// ============================================================================
// Ancient Coral - Interactive elder of the reef
// ============================================================================

/// Marker for the ancient coral formation
#[derive(Component)]
pub struct AncientCoral;

fn spawn_ancient_coral(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Position the ancient coral in a prominent location
    let x = -5.0;
    let z = 5.0;
    let terrain_y = terrain_height_at(x, z);
    let base_pos = Vec3::new(x, terrain_y, z);

    // Ancient coral material - deep, mystical coloring
    let ancient_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.6, 0.7),
        emissive: Color::srgb(0.1, 0.15, 0.2).into(),
        perceptual_roughness: 0.5,
        metallic: 0.2,
        ..default()
    });

    // Glowing center material
    let glow_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.3, 0.8, 1.0, 0.6),
        emissive: Color::srgb(0.2, 0.5, 0.7).into(),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // Main ancient coral structure - large brain coral
    let mut ancient = commands.spawn((
        Mesh3d(meshes.add(Sphere::new(2.0))),
        MeshMaterial3d(ancient_material.clone()),
        Transform::from_translation(base_pos).with_scale(Vec3::new(1.5, 1.0, 1.5)),
        Collider::sphere(2.0),
        RigidBody::Static,
        AncientCoral,
        Name::new("Ancient Coral"),
        Hint::new("🌊 An ancient coral formation... it seems to pulse with timeless wisdom"),
        OceanDialogue {
            node_name: "AncientCoral".to_string(),
        },
    ));

    ancient.observe(on_coral_click);

    // Surrounding smaller formations
    for i in 0..6 {
        let angle = (i as f32 / 6.0) * std::f32::consts::TAU;
        let offset = Vec3::new(angle.cos() * 3.5, 0.0, angle.sin() * 3.5);

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.8))),
            MeshMaterial3d(ancient_material.clone()),
            Transform::from_translation(base_pos + offset + Vec3::Y * -0.5)
                .with_scale(Vec3::splat(0.6 + rand::random::<f32>() * 0.4)),
            Collider::sphere(0.8),
            RigidBody::Static,
            Name::new("Ancient Coral Fragment"),
        ));
    }

    // Central glowing core
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.8))),
        MeshMaterial3d(glow_material),
        Transform::from_translation(base_pos + Vec3::Y * 0.5),
        Name::new("Ancient Coral Core"),
    ));

    // Ambient light from the ancient coral
    commands.spawn((
        Name::new("Ancient Coral Light"),
        PointLight {
            color: Color::srgb(0.3, 0.8, 1.0),
            intensity: 15000.0,
            radius: 12.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_translation(base_pos + Vec3::Y * 2.0),
    ));
}

fn on_coral_click(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    project: Res<YarnProject>,
    dialogue_query: Query<&OceanDialogue>,
    existing_runners: Query<&DialogueRunner>,
) {
    if let Ok(coral_dialogue) = dialogue_query.get(click.event().entity) {
        start_dialogue(
            &mut commands,
            &project,
            &coral_dialogue.node_name,
            &existing_runners,
        );
    }
}
