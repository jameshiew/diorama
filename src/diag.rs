use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct DiagPlugin;

impl Plugin for DiagPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                enabled: false,
                ..default()
            },
        })
        .add_plugins(InputManagerPlugin::<ToggleDiagAction>::default())
        .add_systems(Startup, setup_actions)
        .add_systems(Update, handle_actions);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
struct ToggleDiagAction;

fn setup_actions(mut commands: Commands) {
    let toggle_map = InputMap::new([(ToggleDiagAction, KeyCode::F8)]);
    commands.spawn((Name::new("Diagnostics controls"), toggle_map));
}

fn handle_actions(
    action_state: Single<&ActionState<ToggleDiagAction>>,
    mut overlay: ResMut<FpsOverlayConfig>,
) {
    if action_state.just_pressed(&ToggleDiagAction) {
        overlay.enabled = !overlay.enabled;
    }
}
