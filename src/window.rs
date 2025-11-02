use bevy::prelude::*;
use bevy::window::{
    CursorGrabMode, CursorOptions, MonitorSelection, PrimaryWindow, VideoModeSelection, WindowMode,
};
use leafwing_input_manager::prelude::*;

use crate::state::GameState;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ToggleFullscreenAction>::default())
            .add_systems(Startup, (setup, setup_actions))
            .add_systems(Update, handle_actions)
            .add_systems(OnEnter(GameState::Paused), on_pause)
            .add_systems(OnEnter(GameState::Active), on_resume);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
struct ToggleFullscreenAction;

fn center_cursor(window: &mut Window) {
    let center = Some(Vec2::new(window.width() / 2.0, window.height() / 2.0));
    window.set_cursor_position(center);
}

fn setup_actions(mut commands: Commands) {
    let fullscreen_map = InputMap::new([(ToggleFullscreenAction, KeyCode::F11)]);
    commands.spawn((Name::new("Window controls"), fullscreen_map));
}

fn handle_actions(
    action_state: Single<&ActionState<ToggleFullscreenAction>>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    if action_state.just_pressed(&ToggleFullscreenAction) {
        match window.mode {
            WindowMode::Windowed => {
                window.mode =
                    WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current);
            }
            WindowMode::BorderlessFullscreen(_) | WindowMode::Fullscreen(_, _) => {
                window.mode = WindowMode::Windowed;
            }
        }
    }
}

fn setup(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    cursor_options.grab_mode = CursorGrabMode::Locked;
    cursor_options.visible = false;

    center_cursor(&mut window);
}

fn on_pause(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    center_cursor(&mut window);

    cursor_options.grab_mode = CursorGrabMode::None;
    cursor_options.visible = true;
}

fn on_resume(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    cursor_options.grab_mode = CursorGrabMode::Locked;
    cursor_options.visible = false;

    center_cursor(&mut window);
}
