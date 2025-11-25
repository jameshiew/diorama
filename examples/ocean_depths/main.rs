//! # Ocean Depths
//!
//! An underwater exploration diorama featuring:
//! - Procedurally generated coral reef ecosystem
//! - Fish schools with boids-based flocking behavior
//! - Bioluminescent jellyfish with pulsing glow
//! - Underwater caustics lighting simulation
//! - Interactive treasure discovery
//! - Ancient shipwreck with intelligent octopus
//! - YarnSpinner dialogue with marine creatures
//! - Atmospheric underwater fog and particle effects

use bevy::prelude::*;
use bevy_yarnspinner::prelude::{YarnFileSource, YarnSpinnerPlugin};
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;
use diorama::DioramaPlugin;
use diorama::player::Player;

mod atmosphere;
mod coral;
mod creatures;
pub mod dialogue;
mod materials;
mod seafloor;
mod shipwreck;
mod treasure;

fn main() -> AppExit {
    App::new()
        .add_plugins(DioramaPlugin)
        .add_plugins(OceanDepthsPlugin)
        .run()
}

pub struct OceanDepthsPlugin;

impl Plugin for OceanDepthsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YarnSpinnerPlugin::with_yarn_sources(vec![YarnFileSource::file("dialogue/ocean.yarn")]),
            ExampleYarnSpinnerDialogueViewPlugin::default(),
            seafloor::SeafloorPlugin,
            coral::CoralPlugin,
            creatures::CreaturesPlugin,
            atmosphere::AtmospherePlugin,
            treasure::TreasurePlugin,
            shipwreck::ShipwreckPlugin,
            materials::OceanMaterialsPlugin,
        ))
        .add_systems(Startup, setup_player)
        .add_systems(Update, dialogue::cleanup_finished_dialogue_runners);
    }
}

fn setup_player(mut query: Query<&mut Transform, With<Player>>) {
    if let Ok(mut transform) = query.single_mut() {
        // Start player above the seafloor with a good view
        transform.translation = Vec3::new(0.0, 8.0, 0.0);
    }
}
