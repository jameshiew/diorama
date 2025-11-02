//! # Entity Spawning Helpers
//!
//! Utilities to reduce boilerplate when spawning common architectural elements.

use avian3d::prelude::*;
use bevy::prelude::*;

/// Spawns a static cuboid entity with physics collider
#[allow(clippy::too_many_arguments)]
pub fn spawn_static_cuboid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    name: impl Into<String>,
    size: Vec3,
    material: Handle<StandardMaterial>,
    transform: Transform,
    parent: Option<Entity>,
) -> Entity {
    let entity = commands
        .spawn((
            Name::new(name.into()),
            Mesh3d(meshes.add(Cuboid::from_size(size))),
            MeshMaterial3d(material),
            transform,
            RigidBody::Static,
            Collider::cuboid(size.x, size.y, size.z),
        ))
        .id();

    if let Some(parent_entity) = parent {
        commands.entity(parent_entity).add_child(entity);
    }

    entity
}

/// Spawns a static cylinder entity with physics collider
pub fn spawn_static_cylinder(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    name: impl Into<String>,
    radius: f32,
    height: f32,
    material: Handle<StandardMaterial>,
    transform: Transform,
    parent: Option<Entity>,
) -> Entity {
    let entity = commands
        .spawn((
            Name::new(name.into()),
            Mesh3d(meshes.add(Cylinder::new(radius, height))),
            MeshMaterial3d(material),
            transform,
            RigidBody::Static,
            Collider::cylinder(radius, height),
        ))
        .id();

    if let Some(parent_entity) = parent {
        commands.entity(parent_entity).add_child(entity);
    }

    entity
}

/// Spawns a simple cuboid without physics (for purely decorative elements)
///
/// Note: Most architectural elements should use `spawn_static_cuboid` instead to ensure
/// proper collision detection. This function is only for visual elements that players
/// should never interact with physically.
#[allow(dead_code)]
pub fn spawn_cuboid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    name: impl Into<String>,
    size: Vec3,
    material: Handle<StandardMaterial>,
    transform: Transform,
    parent: Option<Entity>,
) -> Entity {
    let entity = commands
        .spawn((
            Name::new(name.into()),
            Mesh3d(meshes.add(Cuboid::from_size(size))),
            MeshMaterial3d(material),
            transform,
        ))
        .id();

    if let Some(parent_entity) = parent {
        commands.entity(parent_entity).add_child(entity);
    }

    entity
}

/// Creates a parent entity for grouping child entities
pub fn create_group(
    commands: &mut Commands,
    name: impl Into<String>,
    parent: Option<Entity>,
) -> Entity {
    let entity = commands
        .spawn((
            Name::new(name.into()),
            Transform::default(),
            Visibility::default(),
        ))
        .id();

    if let Some(parent_entity) = parent {
        commands.entity(parent_entity).add_child(entity);
    }

    entity
}
