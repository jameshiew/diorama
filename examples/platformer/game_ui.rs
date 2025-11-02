//! UI overlay displaying game information like collected gems.

use bevy::prelude::*;

use crate::GameState;

/// Plugin managing game UI elements.
pub struct GameUIPlugin;

/// Marker component for the game information display text.
#[derive(Component)]
struct GameInfoDisplay;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game_ui)
            .add_systems(Update, update_game_info_display);
    }
}

/// Creates the initial UI display showing gem count.
fn setup_game_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("> Gems: 0"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor::WHITE,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            right: Val::Px(12.0),
            padding: UiRect::all(Val::Px(12.0)),
            ..Node::default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        BorderRadius::all(Val::Px(8.0)),
        GameInfoDisplay,
    ));
}

/// Updates the gem counter display when the game state changes.
fn update_game_info_display(
    game_state: Res<GameState>,
    mut text: Single<&mut Text, With<GameInfoDisplay>>,
) {
    text.0 = format!("> Gems: {}", game_state.gems_collected);
}
