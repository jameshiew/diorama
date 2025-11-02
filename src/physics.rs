use avian3d::prelude::*;
use bevy::prelude::*;

use crate::state::GameState;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(avian3d::prelude::PhysicsPlugins::default())
            .add_systems(OnEnter(GameState::Paused), pause_physics)
            .add_systems(OnEnter(GameState::Active), resume_physics);
    }
}

fn pause_physics(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn resume_physics(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}

#[cfg(feature = "physics-debug")]
pub mod debug {
    use avian3d::prelude::PhysicsGizmos;
    use bevy::prelude::*;
    use leafwing_input_manager::Actionlike;
    use leafwing_input_manager::plugin::InputManagerPlugin;
    use leafwing_input_manager::prelude::{ActionState, ButtonlikeChord, InputMap};

    pub struct PhysicsDebugPlugin;

    impl Plugin for PhysicsDebugPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins(avian3d::debug_render::PhysicsDebugPlugin)
                .insert_gizmo_config(
                    PhysicsGizmos::default(),
                    GizmoConfig {
                        enabled: false,
                        ..default()
                    },
                )
                .add_plugins(InputManagerPlugin::<ToggleGizmosAction>::default())
                .add_systems(Startup, setup_actions)
                .add_systems(Update, handle_actions);
        }
    }

    #[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
    struct ToggleGizmosAction;

    fn setup_actions(mut commands: Commands) {
        let toggle_map = InputMap::new([(
            ToggleGizmosAction,
            ButtonlikeChord::new([KeyCode::F3, KeyCode::KeyB]),
        )]);
        commands.spawn((Name::new("Collider debug controls"), toggle_map));
    }

    fn handle_actions(
        action_state: Single<&ActionState<ToggleGizmosAction>>,
        mut debug_state: ResMut<GizmoConfigStore>,
    ) {
        if action_state.just_pressed(&ToggleGizmosAction) {
            let (gizmo_config, _) = debug_state.config_mut::<PhysicsGizmos>();
            gizmo_config.enabled = !gizmo_config.enabled;
        }
    }
}
