use std::fmt::Write;

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
    mut next_text: Local<String>,
) {
    next_text.clear();

    if let Some(entity) = pointers
        .iter()
        .find_map(|interaction| interaction.get_nearest_hit().map(|(entity, _)| *entity))
    {
        if let Ok(name) = names.get(entity) {
            write!(&mut *next_text, "Looking at: {name}").expect("writing to a String cannot fail");
            if let Ok(hint) = hints.get(entity) {
                next_text.push_str(" - ");
                next_text.push_str(&hint.text);
            }
        } else {
            next_text.push_str("Looking at: unknown");
        }
    } else {
        next_text.push_str("No entity picked");
    }

    if let Ok(mut text) = text_query.single_mut()
        && text.0 != *next_text
    {
        std::mem::swap(&mut text.0, &mut next_text);
    }
}

fn cleanup_picking_ui(mut commands: Commands, query: Query<Entity, With<PickingDisplay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picking_display_does_not_change_when_text_is_current() {
        let mut world = World::new();
        let display = world
            .spawn((Text::new("No entity picked"), PickingDisplay))
            .id();
        let mut schedule = Schedule::default();
        schedule.add_systems(update_picking_display);

        world.clear_trackers();
        schedule.run(&mut world);

        let text = world.entity(display).get_ref::<Text>().unwrap();
        assert!(!text.is_changed());
    }

    #[test]
    fn picking_display_replaces_stale_text_once() {
        let mut world = World::new();
        let display = world.spawn((Text::new("stale"), PickingDisplay)).id();
        let mut schedule = Schedule::default();
        schedule.add_systems(update_picking_display);

        world.clear_trackers();
        schedule.run(&mut world);

        let text = world.entity(display).get_ref::<Text>().unwrap();
        assert!(text.is_changed());
        assert_eq!(text.0, "No entity picked");

        world.clear_trackers();
        schedule.run(&mut world);

        let text = world.entity(display).get_ref::<Text>().unwrap();
        assert!(!text.is_changed());
    }
}
