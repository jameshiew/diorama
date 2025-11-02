//! Moving platforms that transport the player through the level.

use avian3d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

/// Distance threshold for considering a platform arrived at its target.
const PLATFORM_ARRIVAL_THRESHOLD: f32 = 0.1;

/// Component for platforms that move between two positions.
#[derive(Component)]
pub struct MovingPlatform {
    /// Starting position of the platform.
    pub start_pos: Vec3,
    /// Ending position of the platform.
    pub end_pos: Vec3,
    /// Movement speed in units per second.
    pub speed: f32,
    /// Current direction: 1.0 for start->end, -1.0 for end->start.
    pub direction: f32,
}

/// Spawns several moving platforms with different movement patterns.
pub fn spawn_moving_platforms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let moving_platform_material = materials.add(StandardMaterial {
        base_color: tailwind::BLUE_600.into(),
        metallic: 0.3,
        perceptual_roughness: 0.5,
        emissive: LinearRgba::from(tailwind::BLUE_800) * 0.5,
        ..default()
    });

    let platform_mesh = meshes.add(Mesh::from(Cuboid::new(4.0, 0.5, 4.0)));

    // Platform moving horizontally
    commands.spawn((
        Name::new("Moving Platform Horizontal"),
        MovingPlatform {
            start_pos: Vec3::new(12.0, 8.0, -12.0),
            end_pos: Vec3::new(20.0, 8.0, -12.0),
            speed: 2.0,
            direction: 1.0,
        },
        RigidBody::Kinematic,
        Collider::cuboid(4.0, 0.5, 4.0),
        Mesh3d(platform_mesh.clone()),
        MeshMaterial3d(moving_platform_material.clone()),
        Transform::from_translation(Vec3::new(12.0, 8.0, -12.0)),
    ));

    // Platform moving vertically
    commands.spawn((
        Name::new("Moving Platform Vertical"),
        MovingPlatform {
            start_pos: Vec3::new(-6.0, 4.0, -12.0),
            end_pos: Vec3::new(-6.0, 12.0, -12.0),
            speed: 1.5,
            direction: 1.0,
        },
        RigidBody::Kinematic,
        Collider::cuboid(4.0, 0.5, 4.0),
        Mesh3d(platform_mesh.clone()),
        MeshMaterial3d(moving_platform_material.clone()),
        Transform::from_translation(Vec3::new(-6.0, 4.0, -12.0)),
    ));

    // Platform moving diagonally
    commands.spawn((
        Name::new("Moving Platform Diagonal"),
        MovingPlatform {
            start_pos: Vec3::new(0.0, 6.0, -24.0),
            end_pos: Vec3::new(8.0, 10.0, -28.0),
            speed: 1.8,
            direction: 1.0,
        },
        RigidBody::Kinematic,
        Collider::cuboid(4.0, 0.5, 4.0),
        Mesh3d(platform_mesh),
        MeshMaterial3d(moving_platform_material),
        Transform::from_translation(Vec3::new(0.0, 6.0, -24.0)),
    ));
}

/// Updates moving platform positions and handles direction reversal.
pub fn animate_moving_platforms(
    mut platform_query: Query<(&Transform, &mut MovingPlatform, &mut LinearVelocity)>,
) {
    for (transform, mut platform, mut velocity) in platform_query.iter_mut() {
        let current_pos = transform.translation;

        // Determine target based on current direction
        let target_pos = if platform.direction > 0.0 {
            platform.end_pos
        } else {
            platform.start_pos
        };

        let distance_to_target = current_pos.distance(target_pos);

        // Reverse direction when reaching target
        if distance_to_target < PLATFORM_ARRIVAL_THRESHOLD {
            platform.direction *= -1.0;
        }

        // Calculate normalized movement direction
        let move_direction = if platform.direction > 0.0 {
            (platform.end_pos - platform.start_pos).normalize()
        } else {
            (platform.start_pos - platform.end_pos).normalize()
        };

        // Set kinematic body velocity for smooth movement
        let movement_velocity = move_direction * platform.speed;
        velocity.0 = movement_velocity;
    }
}
