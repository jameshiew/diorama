use bevy::prelude::*;
use iyes_perf_ui::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct DiagPlugin;

impl Plugin for DiagPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DiagState>()
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_plugins(InputManagerPlugin::<ToggleDiagAction>::default())
            .add_systems(Startup, setup_actions)
            .add_systems(Update, handle_actions)
            .add_systems(OnEnter(DiagState::Enabled), show_perf_ui)
            .add_systems(OnExit(DiagState::Enabled), hide_perf_ui);
    }
}

#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
enum DiagState {
    Enabled,
    #[default]
    Disabled,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
struct ToggleDiagAction;

fn setup_actions(mut commands: Commands) {
    let toggle_map = InputMap::new([(ToggleDiagAction, KeyCode::F8)]);
    commands.spawn((Name::new("Diagnostics controls"), toggle_map));
}

fn handle_actions(
    current_state: Res<State<DiagState>>,
    mut next_state: ResMut<NextState<DiagState>>,
    action_state: Single<&ActionState<ToggleDiagAction>>,
) {
    if action_state.just_pressed(&ToggleDiagAction) {
        match current_state.get() {
            DiagState::Enabled => next_state.set(DiagState::Disabled),
            DiagState::Disabled => next_state.set(DiagState::Enabled),
        }
    }
}

fn show_perf_ui(mut commands: Commands) {
    commands.spawn(PerfUiAllEntries::default());
}

fn hide_perf_ui(mut commands: Commands, perf_ui_root: Query<Entity, With<PerfUiRoot>>) {
    if let Ok(e) = perf_ui_root.single() {
        commands.entity(e).despawn();
    }
}
