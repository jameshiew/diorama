//! Player spawning and movement initialization.

use bevy::prelude::*;
use diorama::player::Player;

use crate::GameState;

/// The Y-coordinate threshold below which the player will respawn.
const RESPAWN_Y_THRESHOLD: f32 = -100.0;

/// Spawns the player at the starting position.
pub fn spawn_player(mut player: Single<&mut Transform, With<Player>>) {
    let spawn_point = Transform::from_xyz(0.0, 20.0, 0.0);
    player.translation = spawn_point.translation;
}

/// Checks if the player has fallen too far and respawns them at the last checkpoint.
pub fn check_player_respawn(
    mut player: Single<&mut Transform, With<Player>>,
    game_state: Res<GameState>,
) {
    // Check if player has fallen below the threshold
    if player.translation.y < RESPAWN_Y_THRESHOLD {
        // Respawn at the current checkpoint
        player.translation = game_state.current_checkpoint;

        info!(
            "Player fell off the world! Respawning at checkpoint: {:?}",
            game_state.current_checkpoint
        );
    }
}
