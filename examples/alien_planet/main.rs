//! # Alien Planet Survey
//!
//! A procedural exploration example demonstrating:
//! - Procedural terrain generation using noise
//! - Custom mesh generation
//! - Boids flocking simulation
//! - Interactive scanning mechanic
//! - Atmospheric effects

use bevy::prelude::*;
use diorama::DioramaPlugin;
use diorama::player::Player;

mod atmosphere;
mod fauna;
mod flora;
mod materials;
mod scanner;
mod terrain;

fn main() -> AppExit {
    App::new()
        .add_plugins(DioramaPlugin)
        .add_plugins(AlienPlanetPlugin)
        .run()
}

pub struct AlienPlanetPlugin;

impl Plugin for AlienPlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            terrain::TerrainPlugin,
            flora::FloraPlugin,
            fauna::FaunaPlugin,
            atmosphere::AtmospherePlugin,
            scanner::ScannerPlugin,
            materials::CrystalMaterialPlugin,
        ))
        .add_systems(Startup, teleport_player);
    }
}

fn teleport_player(mut query: Query<&mut Transform, With<Player>>) {
    if let Some(mut transform) = query.iter_mut().next() {
        transform.translation = Vec3::new(0.0, 20.0, 0.0);
    }
}
