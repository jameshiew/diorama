//! # Aurora Forge
//!
//! A monumental forge-temple suspended above a volcanic caldera with:
//! - Procedural terrain and basalt architecture
//! - A shader-driven plasma heart and aurora sails
//! - Rotating rings, drifting shards, and sweeping spotlights
//! - Dense ember swarms and a sculpted star canopy

use bevy::prelude::*;
use diorama::DioramaPlugin;

mod animation;
mod materials;
mod scene;

pub struct AuroraForgePlugin;

impl Plugin for AuroraForgePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (scene::setup_scene, scene::spawn_player).chain())
            .add_systems(
                Update,
                (
                    animation::animate_rotors,
                    animation::animate_orbits,
                    animation::animate_hoverers,
                    animation::animate_pulsing_lights,
                    animation::animate_sweeping_spotlights,
                ),
            );
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DioramaPlugin)
        .add_plugins(materials::AuroraForgeMaterialsPlugin)
        .add_plugins(AuroraForgePlugin)
        .run()
}
