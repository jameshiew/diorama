use bevy::prelude::*;

use crate::firstsight::{LookDisabled, MovementDisabled, PlayerCamera};
use crate::player::Player;

#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Active,
    Paused,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), on_pause)
            .add_systems(OnEnter(GameState::Active), on_resume);
    }
}

fn on_pause(
    mut commands: Commands,
    player: Single<(Entity, &Player), Without<MovementDisabled>>,
    player_camera: Single<(Entity, &PlayerCamera), Without<LookDisabled>>,
) {
    commands
        .entity(player.into_inner().0)
        .insert(MovementDisabled);
    commands
        .entity(player_camera.into_inner().0)
        .insert(LookDisabled);
}

fn on_resume(
    mut commands: Commands,
    player: Single<(Entity, &Player), With<MovementDisabled>>,
    player_camera: Single<(Entity, &PlayerCamera), With<LookDisabled>>,
) {
    commands
        .entity(player.into_inner().0)
        .remove::<MovementDisabled>();
    commands
        .entity(player_camera.into_inner().0)
        .remove::<LookDisabled>();
}
