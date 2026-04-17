//! # Mycelial Reverie
//!
//! A bioluminescent mushroom grove at twilight with:
//! - Procedural rolling hills with a sunken spore pool
//! - Giant glowing mushroom caps driven by a custom pulse shader
//! - A bubbling spore pool rendered with a rippling water shader
//! - Drifting spore motes, firefly orbs, and rune stones
//! - A quiet moonlit sky with a gently scattered star canopy

use bevy::prelude::*;
use diorama::DioramaPlugin;

mod animation;
mod materials;
mod scene;

pub struct MycelialReveriePlugin;

impl Plugin for MycelialReveriePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (scene::setup_scene, scene::spawn_player).chain())
            .add_systems(
                Update,
                (
                    animation::animate_orbits,
                    animation::animate_hovers,
                    animation::animate_pulse_lights,
                    animation::animate_spins,
                ),
            );
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DioramaPlugin)
        .add_plugins(materials::MycelialMaterialsPlugin)
        .add_plugins(MycelialReveriePlugin)
        .run()
}
