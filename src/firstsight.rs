//! First-person player controller with camera movement and physics-based controls.
//!
//! This module provides a ready-to-use first-person character controller that integrates
//! with Bevy's ECS, Avian3D physics, and the Tnua character controller.

#![allow(clippy::useless_conversion)]
use avian3d::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy_tnua::builtins::{TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::*;

pub struct FirstSightPlugin;

impl Plugin for FirstSightPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedLook(std::env::var_os("DIORAMA_FIXED_LOOK").is_some()))
            .add_plugins((
                TnuaControllerPlugin::<PlayerControlScheme>::new(FixedUpdate),
                TnuaAvian3dPlugin::new(FixedUpdate),
            ))
            .add_systems(Update, handle_movement.in_set(TnuaUserControlsSystems))
            .add_systems(PostStartup, sync_initial_look)
            .add_systems(
                PostUpdate,
                (update_camera_position, update_camera_looking_at)
                    .before(TransformSystems::Propagate),
            );
    }
}

pub const DEFAULT_PLAYER_HEIGHT: f32 = 1.;
pub const DEFAULT_PLAYER_RADIUS: f32 = 0.5;

const LOOK_SENSITIVITY: f32 = 0.002;
const JUMP_HEIGHT: f32 = 4.;
const SPEED: f32 = 10.;
const SPRINT_MULTIPLIER: f32 = 1.5;

#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
pub enum PlayerControlScheme {
    Jump(TnuaBuiltinJump),
}

pub fn create_player_control_scheme_config(
    control_scheme_configs: &mut Assets<PlayerControlSchemeConfig>,
) -> Handle<PlayerControlSchemeConfig> {
    control_scheme_configs.add(PlayerControlSchemeConfig {
        basis: TnuaBuiltinWalkConfig {
            speed: 1.0,
            float_height: DEFAULT_PLAYER_HEIGHT + 0.5,
            ..Default::default()
        },
        jump: TnuaBuiltinJumpConfig {
            height: JUMP_HEIGHT,
            ..Default::default()
        },
    })
}

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
    TnuaController::<PlayerControlScheme>,
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
    control_config: TnuaConfig<PlayerControlScheme>,
    player_camera_height: PlayerCameraHeight,
}

impl PlayerControllerBundle {
    pub fn new(
        radius: f32,
        height: f32,
        control_config: Handle<PlayerControlSchemeConfig>,
    ) -> Self {
        Self {
            player: PlayerController,
            collider: Collider::capsule(radius.into(), height.into()),
            sensor_shape: TnuaAvian3dSensorShape(Collider::cylinder((radius - 0.01).into(), 0.)),
            control_config: TnuaConfig(control_config),
            player_camera_height: PlayerCameraHeight(height),
        }
    }
}

/// Marker component to disable camera look controls.
///
/// When attached to a PlayerCamera entity, mouse look will be disabled.
#[derive(Component, Default)]
pub struct LookDisabled;

/// When true, mouse look is ignored for the whole session, keeping the
/// camera fixed on the player's initial facing direction.
///
/// Set via the `DIORAMA_FIXED_LOOK` environment variable; used by
/// `just screenshot-and-exit` so screenshots are deterministic even if the
/// mouse moves while the window briefly has cursor grab.
#[derive(Resource)]
struct FixedLook(bool);

/// Marker component to disable player movement controls.
///
/// When attached to the player controller entity, WASD movement will be disabled.
#[derive(Component, Default)]
pub struct MovementDisabled;

/// Handles player movement input (WASD) and applies physics-based movement.
fn handle_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_controller: Single<&mut TnuaController<PlayerControlScheme>, Without<MovementDisabled>>,
    player_camera: Single<&Transform, With<PlayerCamera>>,
) {
    let mut controller = player_controller.into_inner();

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

    controller.basis = TnuaBuiltinWalk {
        desired_motion: (facing.normalize_or_zero() * speed).into(),
        desired_forward: None,
    };

    controller.initiate_action_feeding();

    if keyboard.pressed(KeyCode::Space) {
        controller.action(PlayerControlScheme::Jump(TnuaBuiltinJump::default()));
    }
}

/// Aligns the camera with the player's initial facing direction.
///
/// Runs once after startup so that scenes can aim the opening view by
/// rotating the player entity when they reposition it (e.g. with
/// `Transform::looking_at`). Mouse look takes over from there.
fn sync_initial_look(
    player: Single<&Transform, (With<PlayerController>, Without<PlayerCamera>)>,
    camera: Single<(&mut Transform, &mut PlayerCamera)>,
) {
    let (mut camera_transform, mut player_camera) = camera.into_inner();
    let forward = player.forward();
    player_camera.yaw = (-forward.x).atan2(-forward.z);
    player_camera.pitch = f32::from(forward.y).asin().clamp(-1.5, 1.5);
    camera_transform.rotation =
        Quat::from_rotation_y(player_camera.yaw) * Quat::from_rotation_x(player_camera.pitch);
}

/// Updates the camera position to follow the player controller.
fn update_camera_position(
    player_camera: Single<&mut Transform, With<PlayerCamera>>,
    player_controller: Single<(&Transform, &PlayerCameraHeight), Without<PlayerCamera>>,
) {
    let (player_transform, PlayerCameraHeight(player_camera_height)) =
        player_controller.into_inner();
    let camera_position = player_transform.translation + Vec3::new(0.0, *player_camera_height, 0.0);
    player_camera
        .into_inner()
        .map_unchanged(|transform| &mut transform.translation)
        .set_if_neq(camera_position);
}

/// Handles mouse look input and rotates the camera.
fn update_camera_looking_at(
    mouse_motion: Res<AccumulatedMouseMotion>,
    fixed_look: Res<FixedLook>,
    camera: Single<(&mut Transform, &mut PlayerCamera), Without<LookDisabled>>,
    mut warmup_frames: Local<u32>,
) {
    let (camera_transform, mut player_camera) = camera.into_inner();

    // Ignore mouse motion for the first few frames: grabbing and centering
    // the cursor at startup can report a large spurious delta that would
    // otherwise throw the initial view.
    if *warmup_frames < 10 {
        *warmup_frames = warmup_frames.saturating_add(1);
    } else if !fixed_look.0 {
        let yaw = player_camera.yaw - mouse_motion.delta.x * LOOK_SENSITIVITY;
        let pitch =
            (player_camera.pitch - mouse_motion.delta.y * LOOK_SENSITIVITY).clamp(-1.5, 1.5);

        if yaw != player_camera.yaw || pitch != player_camera.pitch {
            player_camera.yaw = yaw;
            player_camera.pitch = pitch;
        }
    }

    if !camera_transform.is_changed() && !player_camera.is_changed() {
        return;
    }

    let camera_rotation =
        Quat::from_rotation_y(player_camera.yaw) * Quat::from_rotation_x(player_camera.pitch);
    camera_transform
        .map_unchanged(|transform| &mut transform.rotation)
        .set_if_neq(camera_rotation);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_position_does_not_change_when_player_is_stationary() {
        let mut world = World::new();
        world.spawn((Transform::IDENTITY, PlayerCameraHeight::default()));
        let camera = world
            .spawn((
                Transform::from_xyz(0.0, DEFAULT_PLAYER_HEIGHT, 0.0),
                PlayerCamera::default(),
            ))
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(update_camera_position);

        world.clear_trackers();
        schedule.run(&mut world);

        let camera_transform = world.entity(camera).get_ref::<Transform>().unwrap();
        assert!(!camera_transform.is_changed());
    }

    #[test]
    fn camera_position_changes_when_player_moves() {
        let mut world = World::new();
        world.spawn((
            Transform::from_xyz(4.0, 2.0, -3.0),
            PlayerCameraHeight::default(),
        ));
        let camera = world
            .spawn((Transform::IDENTITY, PlayerCamera::default()))
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(update_camera_position);

        world.clear_trackers();
        schedule.run(&mut world);

        let camera_transform = world.entity(camera).get_ref::<Transform>().unwrap();
        assert!(camera_transform.is_changed());
        assert_eq!(camera_transform.translation, Vec3::new(4.0, 3.0, -3.0));
    }

    #[test]
    fn camera_rotation_does_not_change_without_mouse_motion() {
        let mut world = World::new();
        world.insert_resource(AccumulatedMouseMotion::default());
        world.insert_resource(FixedLook(false));
        let camera = world
            .spawn((Transform::IDENTITY, PlayerCamera::default()))
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(update_camera_looking_at);

        world.clear_trackers();
        schedule.run(&mut world);

        let camera_transform = world.entity(camera).get_ref::<Transform>().unwrap();
        let player_camera = world.entity(camera).get_ref::<PlayerCamera>().unwrap();
        assert!(!camera_transform.is_changed());
        assert!(!player_camera.is_changed());
    }

    #[test]
    fn camera_rotation_changes_after_mouse_motion() {
        let mut world = World::new();
        world.insert_resource(AccumulatedMouseMotion::default());
        world.insert_resource(FixedLook(false));
        let camera = world
            .spawn((Transform::IDENTITY, PlayerCamera::default()))
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(update_camera_looking_at);

        for _ in 0..10 {
            schedule.run(&mut world);
        }
        world.clear_trackers();
        world.resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(4.0, -2.0);
        schedule.run(&mut world);

        let camera_transform = world.entity(camera).get_ref::<Transform>().unwrap();
        let player_camera = world.entity(camera).get_ref::<PlayerCamera>().unwrap();
        assert!(camera_transform.is_changed());
        assert!(player_camera.is_changed());
        assert_ne!(camera_transform.rotation, Quat::IDENTITY);
    }

    #[test]
    fn fixed_look_restores_an_external_rotation() {
        let mut world = World::new();
        world.insert_resource(AccumulatedMouseMotion::default());
        world.insert_resource(FixedLook(true));
        let camera = world
            .spawn((Transform::IDENTITY, PlayerCamera::default()))
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(update_camera_looking_at);

        schedule.run(&mut world);
        world.clear_trackers();
        world
            .entity_mut(camera)
            .get_mut::<Transform>()
            .unwrap()
            .rotation = Quat::from_rotation_y(1.0);
        schedule.run(&mut world);

        let camera_transform = world.entity(camera).get_ref::<Transform>().unwrap();
        assert_eq!(camera_transform.rotation, Quat::IDENTITY);
    }
}
