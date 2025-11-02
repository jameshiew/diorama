use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::state::GameState;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PauseResumeAction>::default())
            .add_systems(Startup, setup_actions)
            .add_systems(Update, handle_actions);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub struct PauseResumeAction;

pub fn setup_actions(mut commands: Commands) {
    let toggle_map = InputMap::new([(PauseResumeAction, KeyCode::Escape)]);
    commands.spawn((Name::new("Controls"), toggle_map));
}

pub fn handle_actions(
    action_state: Single<&ActionState<PauseResumeAction>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if action_state.just_pressed(&PauseResumeAction) {
        match current_state.get() {
            GameState::Active => {
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                next_state.set(GameState::Active);
            }
        }
    }
}
