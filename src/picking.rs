use bevy::color::palettes::tailwind::{PINK_100, RED_500};
use bevy::picking::pointer::PointerInteraction;
use bevy::prelude::*;

use crate::state::GameState;

pub(crate) struct PickingPlugin;

#[derive(Component)]
struct PickingDisplay;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshPickingPlugin)
            .add_systems(
                Update,
                (draw_mesh_intersections, update_picking_display)
                    .run_if(in_state(GameState::Active)),
            )
            .add_systems(OnEnter(GameState::Active), setup_picking_ui)
            .add_systems(OnExit(GameState::Active), cleanup_picking_ui);
    }
}

/// A component that can be added to entities to provide hints on what happens when they are picked.
#[derive(Component)]
pub struct Hint {
    pub text: String,
}

impl Hint {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, RED_500);
        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK_100);
    }
}

fn setup_picking_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("No entity picked"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..Node::default()
        },
        PickingDisplay,
    ));
}

fn update_picking_display(
    pointers: Query<&PointerInteraction>,
    names: Query<&Name>,
    hints: Query<&Hint>,
    mut text_query: Query<&mut Text, With<PickingDisplay>>,
) {
    let mut picked_entity_name = None;

    // Find the nearest picked entity
    for interaction in pointers.iter() {
        if let Some((entity, _hit)) = interaction.get_nearest_hit() {
            if let Ok(name) = names.get(*entity) {
                let mut txt = name.as_str().to_string();
                if let Ok(hint) = hints.get(*entity) {
                    txt.push_str(" - ");
                    txt.push_str((hint.text).as_str());
                }
                picked_entity_name = Some(txt);
            } else {
                picked_entity_name = Some("unknown".to_string());
            }
            break; // Only show the first/nearest hit
        }
    }

    // Update the display text
    if let Ok(mut text) = text_query.single_mut() {
        match picked_entity_name {
            Some(name) => text.0 = format!("Looking at: {name}"),
            None => text.0 = "No entity picked".to_string(),
        }
    }
}

fn cleanup_picking_ui(mut commands: Commands, query: Query<Entity, With<PickingDisplay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
