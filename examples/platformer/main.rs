//! A 3D platformer game demonstrating physics-based character movement,
//! collectibles, moving platforms, and level design.

use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use diorama::DioramaPlugin;

mod collectibles;
mod game_ui;
mod level;
mod movement;
mod platforms;

fn main() -> AppExit {
    App::new()
        .add_plugins(DioramaPlugin)
        .add_plugins(PlatformerPlugin)
        .run()
}

/// Main plugin that orchestrates all platformer systems and resources.
pub struct PlatformerPlugin;

impl Plugin for PlatformerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState::new())
            .add_plugins(game_ui::GameUIPlugin)
            .add_systems(
                Startup,
                (
                    setup_environment,
                    // Initialize level data first, then spawn geometry and objects
                    level::initialize_level,
                    level::spawn_level_geometry,
                    platforms::spawn_moving_platforms,
                    collectibles::spawn_collectibles,
                    movement::spawn_player,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    platforms::animate_moving_platforms,
                    collectibles::animate_collectibles,
                    collectibles::handle_collectible_pickup,
                    collectibles::animate_collection_particles,
                    movement::check_player_respawn,
                ),
            );
    }
}

/// Game state tracking collectibles and player progress.
#[derive(Resource, Default)]
pub struct GameState {
    /// Total number of gems collected by the player.
    pub gems_collected: u32,
    /// Last checkpoint position for respawning.
    pub current_checkpoint: Vec3,
}

impl GameState {
    /// Creates a new game state with the initial spawn point as the checkpoint.
    fn new() -> Self {
        Self {
            gems_collected: 0,
            current_checkpoint: Vec3::new(0.0, 20.0, 0.0), // Same as player spawn point
        }
    }
}

/// Sets up the game environment with lighting and background color.
fn setup_environment(mut commands: Commands) {
    // Ambient lighting for overall brightness
    commands.insert_resource(AmbientLight {
        color: tailwind::YELLOW_50.into(),
        brightness: 300.0,
        affects_lightmapped_meshes: true,
    });

    // Clear color for sky background
    commands.insert_resource(ClearColor(tailwind::SKY_300.into()));

    // Directional light for shadows and depth
    commands.spawn((
        Name::new("Sun"),
        DirectionalLight {
            color: tailwind::YELLOW_200.into(),
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.3, 0.0)),
    ));
}
