//! Hidden treasure discovery system
//!
//! Spawns a treasure chest that can be discovered by the player,
//! with glowing particles to draw attention.

use avian3d::prelude::*;
use bevy::math::Vec4;
use bevy::picking::events::{Click, Pointer};
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use diorama::picking::Hint;

use crate::dialogue::{OceanDialogue, start_dialogue, terrain_height_at};
use crate::materials::{TreasureChestData, TreasureChestMaterial};

pub struct TreasurePlugin;

impl Plugin for TreasurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_treasure)
            .add_systems(Update, (animate_treasure_glow, animate_gold_particles));
    }
}

/// Treasure chest component
#[derive(Component)]
pub struct TreasureChest;

/// Glowing particle near treasure
#[derive(Component)]
pub struct GoldParticle {
    pub base_pos: Vec3,
    pub phase: f32,
}

/// The treasure glow effect
#[derive(Component)]
pub struct TreasureGlow;

fn spawn_treasure(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
    mut chest_materials: ResMut<Assets<TreasureChestMaterial>>,
) {
    // Find a suitable location on the seafloor
    let x = 25.0;
    let z = -20.0;
    let terrain_y = terrain_height_at(x, z);
    let chest_pos = Vec3::new(x, terrain_y + 0.5, z);

    // Treasure chest (simplified as a box)
    let chest_mesh = meshes.add(Cuboid::new(1.5, 1.0, 1.0));
    let chest_lid_mesh = meshes.add(Cuboid::new(1.6, 0.3, 1.1));

    // Use custom shader material for the chest
    let chest_material = chest_materials.add(TreasureChestMaterial {
        data: TreasureChestData {
            wood_color: Vec4::new(0.4, 0.25, 0.1, 1.0),
            glow_color: Vec4::new(1.0, 0.85, 0.3, 1.0),
            weathering: 0.75,
            magic_intensity: 0.7,
            _padding: 0,
        },
    });

    let gold_trim_material = std_materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.7, 0.2),
        metallic: 0.9,
        perceptual_roughness: 0.3,
        emissive: Color::srgb(0.2, 0.15, 0.0).into(),
        ..default()
    });

    let mut chest = commands.spawn((
        Mesh3d(chest_mesh),
        MeshMaterial3d(chest_material),
        Transform::from_translation(chest_pos),
        Collider::cuboid(1.5, 1.0, 1.0),
        RigidBody::Static,
        TreasureChest,
        Name::new("Treasure Chest"),
        Hint::new("💰 An ancient treasure chest! Click to discover its secrets..."),
        OceanDialogue {
            node_name: "TreasureChest".to_string(),
        },
    ));

    chest.observe(on_treasure_click);

    chest.with_children(|parent| {
        // Chest lid
        parent.spawn((
            Mesh3d(chest_lid_mesh),
            MeshMaterial3d(gold_trim_material.clone()),
            Transform::from_xyz(0.0, 0.65, 0.0),
        ));
    });

    // Spawn gold particles floating around the treasure
    let particle_mesh = meshes.add(Sphere::new(0.1));
    let particle_material = std_materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.85, 0.3),
        emissive: Color::srgb(0.5, 0.4, 0.1).into(),
        ..default()
    });

    for i in 0..12 {
        let angle = (i as f32 / 12.0) * std::f32::consts::TAU;
        let radius = 2.0 + rand::random::<f32>() * 1.5;
        let particle_pos = chest_pos
            + Vec3::new(
                angle.cos() * radius,
                1.0 + rand::random::<f32>() * 2.0,
                angle.sin() * radius,
            );

        commands.spawn((
            Mesh3d(particle_mesh.clone()),
            MeshMaterial3d(particle_material.clone()),
            Transform::from_translation(particle_pos)
                .with_scale(Vec3::splat(0.3 + rand::random::<f32>() * 0.4)),
            GoldParticle {
                base_pos: particle_pos,
                phase: rand::random::<f32>() * std::f32::consts::TAU,
            },
            Name::new("Gold Particle"),
        ));
    }

    // Central glow effect
    let glow_mesh = meshes.add(Sphere::new(0.5));
    let glow_material = std_materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.9, 0.5, 0.3),
        emissive: Color::srgb(0.8, 0.6, 0.2).into(),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    commands.spawn((
        Mesh3d(glow_mesh),
        MeshMaterial3d(glow_material),
        Transform::from_translation(chest_pos + Vec3::Y * 1.5),
        TreasureGlow,
        Name::new("Treasure Glow"),
    ));

    // Point light for treasure illumination
    commands.spawn((
        Name::new("Treasure Light"),
        PointLight {
            color: Color::srgb(1.0, 0.85, 0.4),
            intensity: 30000.0,
            radius: 15.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(chest_pos + Vec3::Y * 2.0),
    ));

    // Spawn some scattered gold coins
    spawn_gold_coins(&mut commands, &mut meshes, &mut std_materials, chest_pos);
}

fn spawn_gold_coins(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chest_pos: Vec3,
) {
    let coin_mesh = meshes.add(Cylinder::new(0.15, 0.03));
    let coin_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.85, 0.2),
        metallic: 1.0,
        perceptual_roughness: 0.2,
        ..default()
    });

    for _ in 0..20 {
        let offset_x = (rand::random::<f32>() - 0.5) * 4.0;
        let offset_z = (rand::random::<f32>() - 0.5) * 4.0;

        let x = chest_pos.x + offset_x;
        let z = chest_pos.z + offset_z;
        let terrain_y = terrain_height_at(x, z);

        commands.spawn((
            Mesh3d(coin_mesh.clone()),
            MeshMaterial3d(coin_material.clone()),
            Transform::from_xyz(x, terrain_y + 0.2, z).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                rand::random::<f32>() * std::f32::consts::FRAC_PI_2,
                rand::random::<f32>() * std::f32::consts::TAU,
                rand::random::<f32>() * std::f32::consts::FRAC_PI_2,
            )),
            Name::new("Gold Coin"),
        ));
    }
}

fn animate_treasure_glow(time: Res<Time>, mut query: Query<&mut Transform, With<TreasureGlow>>) {
    let t = time.elapsed_secs();

    for mut transform in query.iter_mut() {
        // Pulsing glow
        let pulse = (t * 2.0).sin() * 0.2 + 1.0;
        transform.scale = Vec3::splat(pulse);
    }
}

fn animate_gold_particles(time: Res<Time>, mut query: Query<(&mut Transform, &GoldParticle)>) {
    let t = time.elapsed_secs();

    for (mut transform, particle) in query.iter_mut() {
        // Floating animation
        let y_offset = (t * 1.5 + particle.phase).sin() * 0.5;
        let x_offset = (t * 0.7 + particle.phase).cos() * 0.3;
        let z_offset = (t * 0.9 + particle.phase * 1.3).sin() * 0.3;

        transform.translation = particle.base_pos + Vec3::new(x_offset, y_offset, z_offset);

        // Slow rotation
        transform.rotate_y(0.01);
    }
}

// ============================================================================
// Click handler for treasure
// ============================================================================

fn on_treasure_click(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    project: Res<YarnProject>,
    dialogue_query: Query<&OceanDialogue>,
    existing_runners: Query<&DialogueRunner>,
) {
    if let Ok(treasure_dialogue) = dialogue_query.get(click.event().entity) {
        start_dialogue(
            &mut commands,
            &project,
            &treasure_dialogue.node_name,
            &existing_runners,
        );
    }
}
