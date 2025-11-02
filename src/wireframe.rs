use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::*;
use leafwing_input_manager::Actionlike;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::{ActionState, ButtonlikeChord, InputMap};

pub struct WireframePlugin;

impl Plugin for WireframePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::pbr::wireframe::WireframePlugin::default())
            .add_plugins(InputManagerPlugin::<ToggleWireframesAction>::default())
            .add_systems(Startup, setup_actions)
            .add_systems(Update, handle_actions);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
struct ToggleWireframesAction;

fn setup_actions(mut commands: Commands) {
    let toggle_map = InputMap::new([(
        ToggleWireframesAction,
        ButtonlikeChord::new([KeyCode::F3, KeyCode::KeyG]),
    )]);
    commands.spawn((Name::new("Wireframe controls"), toggle_map));
}

fn handle_actions(
    action_state: Single<&ActionState<ToggleWireframesAction>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    if action_state.just_pressed(&ToggleWireframesAction) {
        wireframe_config.global = !wireframe_config.global;
    }
}
