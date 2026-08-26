use bevy::prelude::*;
use diorama::DioramaPlugin;

mod animation;
mod scene;

pub struct RainClockGardenPlugin;

impl Plugin for RainClockGardenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (scene::setup_scene, scene::spawn_player).chain())
            .add_systems(
                Update,
                (
                    animation::animate_clock_hands,
                    animation::animate_drifts,
                    animation::animate_drops,
                    animation::animate_pendulums,
                    animation::animate_pulse_lights,
                    animation::animate_ripples,
                    animation::animate_spins,
                ),
            );
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DioramaPlugin)
        .add_plugins(RainClockGardenPlugin)
        .run()
}
