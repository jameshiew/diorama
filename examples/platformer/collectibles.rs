//! Collectible gems with floating animations and particle effects.
//!
//! # Level Integration
//!
//! Collectible positions are no longer hardcoded in this module. Instead, they are
//! defined alongside platform geometry in the `level` module, creating a cohesive
//! level design system where gems are strategically placed on platforms.
//!
//! The [`spawn_collectibles`] system reads positions from the [`CurrentLevel`] resource,
//! which is initialized during startup by the level system.
//!
//! To add or modify collectible positions, edit the section definitions in `level.rs`.

use avian3d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

/// Radius of collectible gem spheres.
const COLLECTIBLE_RADIUS: f32 = 0.3;

/// Distance at which the player can collect a gem.
const COLLECTION_DISTANCE: f32 = 1.0;

/// How long collection effect particles live before despawning.
const PARTICLE_LIFETIME_SECS: f32 = 1.0;

/// Marker component for collectible items that award points when collected.
#[derive(Component)]
pub struct Collectible {
    /// Points awarded when this collectible is picked up.
    pub value: u32,
}

/// Component for smooth vertical floating animation.
#[derive(Component)]
pub struct FloatingAnimation {
    /// Base Y position around which to float.
    pub base_y: f32,
    /// Maximum vertical displacement from base position.
    pub amplitude: f32,
    /// Speed of floating oscillation.
    pub frequency: f32,
    /// Phase offset for staggered animations.
    pub phase: f32,
}

/// Component for continuous rotation animation.
#[derive(Component)]
pub struct RotatingAnimation {
    /// Rotation speed in radians per second.
    pub speed: f32,
}

/// Spawns collectible gems throughout the level based on level definition.
///
/// Collectible positions are now defined in the level module alongside platform
/// geometry, making it easy to design cohesive levels where gems are placed
/// strategically on platforms and obstacles.
pub fn spawn_collectibles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    current_level: Res<crate::level::CurrentLevel>,
) {
    let gem_material = materials.add(StandardMaterial {
        base_color: tailwind::YELLOW_500.into(),
        metallic: 0.8,
        perceptual_roughness: 0.1,
        emissive: LinearRgba::from(tailwind::YELLOW_600) * 2.0,
        ..default()
    });

    let gem_mesh = meshes.add(Mesh::from(Sphere::new(COLLECTIBLE_RADIUS)));

    // Spawn gems at positions defined in the level data
    for (i, position) in current_level.0.collectible_positions.iter().enumerate() {
        let gem_num = i + 1;
        commands.spawn((
            Name::new(format!("Gem {gem_num}")),
            Collectible { value: 10 },
            FloatingAnimation {
                base_y: position.y,
                amplitude: 0.3,
                frequency: 2.0,
                phase: i as f32 * 0.5, // Stagger animations
            },
            RotatingAnimation { speed: 1.5 },
            RigidBody::Static,
            Collider::sphere(COLLECTIBLE_RADIUS),
            Sensor, // Trigger collider for pickup detection
            Mesh3d(gem_mesh.clone()),
            MeshMaterial3d(gem_material.clone()),
            Transform::from_translation(*position),
        ));
    }
}

/// Animates collectibles with floating and rotation effects.
pub fn animate_collectibles(
    time: Res<Time>,
    mut collectible_query: Query<
        (&mut Transform, &FloatingAnimation, &RotatingAnimation),
        With<Collectible>,
    >,
) {
    for (mut transform, float_anim, rotate_anim) in collectible_query.iter_mut() {
        // Vertical floating motion
        let elapsed = time.elapsed_secs();
        let float_offset =
            (elapsed * float_anim.frequency + float_anim.phase).sin() * float_anim.amplitude;
        transform.translation.y = float_anim.base_y + float_offset;

        // Continuous Y-axis rotation
        transform.rotate_y(rotate_anim.speed * time.delta_secs());
    }
}

/// Handles collision detection and pickup of collectibles by the player.
pub fn handle_collectible_pickup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    collectible_query: Query<(Entity, &Collectible, &Transform), Without<diorama::player::Player>>,
    player_transform: Single<&Transform, With<diorama::player::Player>>,
    mut game_state: ResMut<crate::GameState>,
) {
    for (entity, collectible, collectible_transform) in collectible_query.iter() {
        let distance = player_transform
            .translation
            .distance(collectible_transform.translation);

        if distance < COLLECTION_DISTANCE {
            game_state.gems_collected += collectible.value;

            // Spawn visual feedback particles
            spawn_collection_effect(
                &mut commands,
                &mut meshes,
                &mut materials,
                collectible_transform.translation,
            );

            commands.entity(entity).despawn();

            println!("Collected gem! Total: {}", game_state.gems_collected);
        }
    }
}

/// Spawns particle effect when a collectible is picked up.
fn spawn_collection_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let particle_material = materials.add(StandardMaterial {
        base_color: tailwind::YELLOW_400.into(),
        emissive: LinearRgba::from(tailwind::YELLOW_500) * 3.0,
        unlit: true,
        ..default()
    });

    let particle_mesh = meshes.add(Mesh::from(Sphere::new(0.05)));

    // Spawn particles in a radial pattern
    for i in 0..8 {
        let angle = (i as f32) * std::f32::consts::PI / 4.0;
        let offset = Vec3::new(angle.cos(), 0.5, angle.sin()) * 0.5;

        commands.spawn((
            Name::new("Collection Particle"),
            CollectionParticle {
                lifetime: Timer::from_seconds(PARTICLE_LIFETIME_SECS, TimerMode::Once),
                initial_velocity: offset * 2.0,
            },
            Mesh3d(particle_mesh.clone()),
            MeshMaterial3d(particle_material.clone()),
            Transform::from_translation(position + offset * 0.1),
        ));
    }
}

/// Component for short-lived particle effects with physics.
#[derive(Component)]
pub struct CollectionParticle {
    /// Time remaining before particle despawns.
    lifetime: Timer,
    /// Initial outward velocity of the particle.
    initial_velocity: Vec3,
}

/// Animates collection particles with gravity and fade-out effects.
pub fn animate_collection_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particle_query: Query<(Entity, &mut Transform, &mut CollectionParticle)>,
) {
    for (entity, mut transform, mut particle) in particle_query.iter_mut() {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.is_finished() {
            commands.entity(entity).despawn();
        } else {
            // Apply velocity with gravity
            let progress = particle.lifetime.elapsed_secs();
            let gravity = Vec3::new(0.0, -5.0, 0.0);
            let velocity = particle.initial_velocity + gravity * progress;

            transform.translation += velocity * time.delta_secs();

            // Fade out by scaling down over lifetime
            let scale = 1.0 - particle.lifetime.fraction();
            transform.scale = Vec3::splat(scale);
        }
    }
}
