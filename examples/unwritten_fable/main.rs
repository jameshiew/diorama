#![recursion_limit = "256"]

//! # The Unwritten Fable
//!
//! A fable caught in the act of being written, by Fable 5.
//!
//! - A colossal open storybook on a midnight writing desk, its parchment
//!   covered in procedurally scripted handwriting
//! - A river of ink running down the spine gutter and pouring off the page
//!   in an inkfall
//! - A tireless quill scratching at the frontier, streaming golden glyphs
//!   onto the paper
//! - West of the frontier, a papercraft pop-up world; east of it, blank
//!   sketch-paper where the story has not happened yet
//! - Fall off the book and climb back up a staircase of closed tomes

use bevy::prelude::*;
use diorama::DioramaPlugin;

mod animation;
mod materials;
mod scene;

pub struct UnwrittenFablePlugin;

impl Plugin for UnwrittenFablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (scene::setup_scene, scene::spawn_player).chain())
            .add_systems(
                Update,
                (
                    animation::animate_quills,
                    animation::animate_glyph_flights,
                    animation::animate_drift_motes,
                    animation::animate_ink_drips,
                    animation::animate_pulse_lights,
                ),
            );
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DioramaPlugin)
        .add_plugins(materials::UnwrittenFableMaterialsPlugin)
        .add_plugins(UnwrittenFablePlugin)
        .run()
}
