//! First-person player controller with camera movement and physics-based controls.
//!
//! This module provides a ready-to-use first-person character controller that integrates
//! with Bevy's ECS, Avian3D physics, and the Tnua character controller.

#![allow(clippy::useless_conversion)]
use avian3d::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

pub struct FirstSightPlugin;

impl Plugin for FirstSightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
        ))
        .add_systems(Update, handle_movement.in_set(TnuaUserControlsSystems))
        .add_systems(
            PostUpdate,
            (update_camera_position, update_camera_looking_at).before(TransformSystems::Propagate),
        );
    }
}

pub const DEFAULT_PLAYER_HEIGHT: f32 = 1.;
pub const DEFAULT_PLAYER_RADIUS: f32 = 0.5;

const LOOK_SENSITIVITY: f32 = 0.002;
const JUMP_HEIGHT: f32 = 4.;
const SPEED: f32 = 10.;
const SPRINT_MULTIPLIER: f32 = 1.5;

/// Camera component for first-person player view.
///
/// Tracks yaw and pitch for smooth camera rotation.
#[derive(Component, Default)]
#[require(
    Camera3d,
    Camera,
    Projection::from(PerspectiveProjection::default()),
    Transform
)]
pub struct PlayerCamera {
    yaw: f32,
    pitch: f32,
}

/// Height offset for the camera relative to the player controller.
#[derive(Component)]
struct PlayerCameraHeight(f32);

impl Default for PlayerCameraHeight {
    fn default() -> Self {
        Self(DEFAULT_PLAYER_HEIGHT)
    }
}

/// Core player controller component.
///
/// Requires Transform, TnuaController, RigidBody, and locked rotation axes.
#[derive(Component, Default)]
#[require(
    Transform,
    TnuaController,
    RigidBody::Dynamic,
    LockedAxes::ROTATION_LOCKED
)]
struct PlayerController;

/// Bundle for spawning a player controller with physics.
///
/// Includes collider shape and sensor configuration for ground detection.
#[derive(Bundle)]
pub struct PlayerControllerBundle {
    player: PlayerController,
    collider: Collider,
    sensor_shape: TnuaAvian3dSensorShape,
    player_camera_height: PlayerCameraHeight,
}

impl PlayerControllerBundle {
    pub fn new(radius: f32, height: f32) -> Self {
        Self {
            player: PlayerController,
            collider: Collider::capsule(radius.into(), height.into()),
            sensor_shape: TnuaAvian3dSensorShape(Collider::cylinder((radius - 0.01).into(), 0.)),
            player_camera_height: PlayerCameraHeight(height),
        }
    }
}

impl Default for PlayerControllerBundle {
    fn default() -> Self {
        Self::new(DEFAULT_PLAYER_RADIUS, DEFAULT_PLAYER_HEIGHT)
    }
}

/// Marker component to disable camera look controls.
///
/// When attached to a PlayerCamera entity, mouse look will be disabled.
#[derive(Component, Default)]
pub struct LookDisabled;

/// Marker component to disable player movement controls.
///
/// When attached to the player controller entity, WASD movement will be disabled.
#[derive(Component, Default)]
pub struct MovementDisabled;

/// Handles player movement input (WASD) and applies physics-based movement.
fn handle_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_controller: Single<
        (&mut TnuaController, &PlayerCameraHeight),
        Without<MovementDisabled>,
    >,
    player_camera: Single<&Transform, With<PlayerCamera>>,
) {
    let (mut controller, PlayerCameraHeight(player_camera_height)) = player_controller.into_inner();

    let forward = player_camera.forward();
    let right = player_camera.right();

    // Project onto the horizontal plane
    let forward_flat = Vec3::new(forward.x.into(), 0.0, forward.z.into()).normalize_or_zero();
    let right_flat = Vec3::new(right.x.into(), 0.0, right.z.into()).normalize_or_zero();

    let mut facing = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        facing += forward_flat;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        facing -= forward_flat;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        facing -= right_flat;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        facing += right_flat;
    }

    // Apply sprint multiplier if left shift is held
    let speed = if keyboard.pressed(KeyCode::ShiftLeft) {
        SPEED * SPRINT_MULTIPLIER
    } else {
        SPEED
    };

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: (facing.normalize_or_zero() * speed).into(),
        float_height: (player_camera_height + 0.5).into(),
        ..default()
    });

    if keyboard.pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            height: JUMP_HEIGHT.into(),
            ..default()
        });
    }
}

/// Updates the camera position to follow the player controller.
fn update_camera_position(
    mut player_camera: Single<&mut Transform, With<PlayerCamera>>,
    player_controller: Single<(&Transform, &PlayerCameraHeight), Without<PlayerCamera>>,
) {
    let (player_transform, PlayerCameraHeight(player_camera_height)) =
        player_controller.into_inner();
    player_camera.translation =
        player_transform.translation + Vec3::new(0.0, *player_camera_height, 0.0);
}

/// Handles mouse look input and rotates the camera.
fn update_camera_looking_at(
    mouse_motion: Res<AccumulatedMouseMotion>,
    camera: Single<(&mut Transform, &mut PlayerCamera), Without<LookDisabled>>,
) {
    let (mut camera_transform, mut player_camera) = camera.into_inner();

    player_camera.yaw -= mouse_motion.delta.x * LOOK_SENSITIVITY;
    player_camera.pitch -= mouse_motion.delta.y * LOOK_SENSITIVITY;

    // Clamp pitch to prevent looking too far up or down
    player_camera.pitch = player_camera.pitch.clamp(-1.5, 1.5);

    // Apply rotation
    camera_transform.rotation =
        Quat::from_rotation_y(player_camera.yaw) * Quat::from_rotation_x(player_camera.pitch);
}
