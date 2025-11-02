use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use leafwing_input_manager::prelude::*;

pub struct InspectorPlugin;

#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum InspectorState {
    Enabled,
    #[default]
    Disabled,
}

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin::default());
        }
        app.init_state::<InspectorState>()
            .add_plugins(WorldInspectorPlugin::default().run_if(in_state(InspectorState::Enabled)))
            .add_plugins(InputManagerPlugin::<ToggleInspectorAction>::default())
            .add_systems(Startup, setup_actions)
            .add_systems(Update, handle_actions);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
struct ToggleInspectorAction;

fn setup_actions(mut commands: Commands) {
    let toggle_map = InputMap::new([(ToggleInspectorAction, KeyCode::F7)]);
    commands.spawn((Name::new("Inspector controls"), toggle_map));
}

fn handle_actions(
    action_state: Single<&ActionState<ToggleInspectorAction>>,
    current_state: Res<State<InspectorState>>,
    mut next_state: ResMut<NextState<InspectorState>>,
) {
    if action_state.just_pressed(&ToggleInspectorAction) {
        match current_state.get() {
            InspectorState::Enabled => next_state.set(InspectorState::Disabled),
            InspectorState::Disabled => next_state.set(InspectorState::Enabled),
        }
    }
}
