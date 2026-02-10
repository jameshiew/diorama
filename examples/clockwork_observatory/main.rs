//! # Clockwork Observatory
//!
//! A dramatic skyborne observatory with:
//! - A radial deck and suspended viewing pods
//! - A kinetic orrery of rotating rings and orbital bodies
//! - Bobbing lantern swarms and pulsing accent lights
//! - A dense handcrafted star canopy

use bevy::prelude::*;
use diorama::DioramaPlugin;

mod animation;
mod scene;

pub struct ClockworkObservatoryPlugin;

impl Plugin for ClockworkObservatoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (scene::setup_scene, scene::spawn_player).chain())
            .add_systems(
                Update,
                (
                    animation::animate_ring_spins,
                    animation::animate_orbits,
                    animation::animate_lights,
                    animation::animate_lanterns,
                ),
            );
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DioramaPlugin)
        .add_plugins(ClockworkObservatoryPlugin)
        .run()
}
