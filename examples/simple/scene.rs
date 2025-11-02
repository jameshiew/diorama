use avian3d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use diorama::player::Player;

use crate::{Animated, AnimatedMaterial, create_marble_floor_material};

/// Lighting configuration
const AMBIENT_LIGHT_BRIGHTNESS: f32 = 800.0;
const POINT_LIGHT_POSITION: Vec3 = Vec3::new(4.0, 8.0, 4.0);

/// Ground dimensions
const GROUND_WIDTH: f32 = 1_000.0;
const GROUND_HEIGHT: f32 = 1.0;
const GROUND_DEPTH: f32 = 1_000.0;

/// Animated cube configuration
const CUBE_SIZE: f32 = 1.0;
const CUBE_POSITION: Vec3 = Vec3::new(0.0, 4.0, 2.0);

/// Player spawn configuration
const PLAYER_SPAWN_POSITION: Vec3 = Vec3::new(0.0, 4.0, 0.0);
const PLAYER_LOOK_AT: Vec3 = Vec3::new(0.0, 4.0, 2.0);

/// Sets up the main scene with ground, lighting, and animated objects
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut shader_materials: ResMut<Assets<AnimatedMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // Configure ambient lighting and background
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: AMBIENT_LIGHT_BRIGHTNESS,
        affects_lightmapped_meshes: true,
    });
    commands.insert_resource(ClearColor(tailwind::BLUE_100.into()));

    let marble_floor_material = create_marble_floor_material(&mut materials, &mut images);

    let mut root = commands.spawn((
        Name::new("Scene root"),
        Visibility::default(),
        Transform::default(),
        children![
            (
                Name::new("Point light"),
                PointLight {
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_translation(POINT_LIGHT_POSITION),
            ),
            (
                Name::new("Ground"),
                // Mesh components
                Mesh3d(meshes.add(Cuboid::new(GROUND_WIDTH, GROUND_HEIGHT, GROUND_DEPTH))),
                MeshMaterial3d(marble_floor_material),
                // Physics components
                RigidBody::Static,
                Collider::cuboid(GROUND_WIDTH, GROUND_HEIGHT, GROUND_DEPTH),
            )
        ],
    ));

    // Add animated shader cube
    root.with_child((
        Name::new("Shader Cube"),
        Animated,
        Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
        MeshMaterial3d(shader_materials.add(AnimatedMaterial {})),
        Transform::from_translation(CUBE_POSITION),
        RigidBody::Dynamic,
        Collider::cuboid(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE),
    ));
}

/// Spawns the player at the initial position looking towards the animated cube
pub fn spawn_player(mut player: Single<&mut Transform, With<Player>>) {
    let spawn_transform =
        Transform::from_translation(PLAYER_SPAWN_POSITION).looking_at(PLAYER_LOOK_AT, Vec3::Y);

    player.translation = spawn_transform.translation;
    player.rotation = spawn_transform.rotation;
}
