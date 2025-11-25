//! Dialogue system integration using YarnSpinner
//!
//! Provides interactive conversations with marine creatures.

use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use noise::{NoiseFn, Perlin};

/// Shared noise seed for consistent terrain across modules
pub const NOISE_SEED: u32 = 42;

/// Terrain Y offset (seafloor base position)
pub const TERRAIN_Y_OFFSET: f32 = -5.0;

/// Component for entities that can trigger dialogue
#[derive(Component)]
pub struct OceanDialogue {
    pub node_name: String,
}

impl OceanDialogue {
    pub fn new(node_name: impl Into<String>) -> Self {
        Self {
            node_name: node_name.into(),
        }
    }
}

/// Cleans up DialogueRunner entities that have finished their conversations
pub fn cleanup_finished_dialogue_runners(
    mut commands: Commands,
    dialogue_runners: Query<(Entity, &DialogueRunner)>,
) {
    for (entity, dialogue_runner) in dialogue_runners.iter() {
        if !dialogue_runner.is_running() {
            commands.entity(entity).despawn();
        }
    }
}

/// Returns true if any dialogue is currently running
pub fn is_dialogue_running(runners: &Query<&DialogueRunner>) -> bool {
    runners.iter().any(|r| r.is_running())
}

/// Starts a dialogue if none is currently running. Returns true if started.
pub fn start_dialogue(
    commands: &mut Commands,
    project: &Res<YarnProject>,
    node_name: &str,
    existing_runners: &Query<&DialogueRunner>,
) -> bool {
    if is_dialogue_running(existing_runners) {
        return false;
    }

    let mut dialogue_runner = project.create_dialogue_runner(commands);
    dialogue_runner.start_node(node_name);
    commands.spawn(dialogue_runner);
    true
}

/// Calculates terrain height at a given (x, z) position using consistent noise
pub fn terrain_height_at(x: f32, z: f32) -> f32 {
    let perlin = Perlin::new(NOISE_SEED);
    let height = perlin.get([x as f64 * 0.03, z as f64 * 0.03]) * 6.0
        + perlin.get([x as f64 * 0.08, z as f64 * 0.08]) * 1.8;
    height as f32 + TERRAIN_Y_OFFSET
}
