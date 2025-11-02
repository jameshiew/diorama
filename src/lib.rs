#![deny(unstable_features)]
#![deny(unused_features)]
use bevy::prelude::*;

mod controls;
#[cfg(feature = "perfui")]
mod diag;
mod firstsight;
#[cfg(feature = "inspector")]
mod inspector;
mod physics;
pub mod picking;
pub mod player;
mod state;
mod window;
mod wireframe;

use crate::controls::ControlsPlugin;
use crate::physics::PhysicsPlugin;
use crate::picking::PickingPlugin;
use crate::player::PlayerPlugin;
use crate::state::{GameState, StatePlugin};

pub struct DioramaPlugin;

impl Plugin for DioramaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
        app.add_plugins(bevy_framepace::FramepacePlugin);
        app.init_state::<GameState>().add_plugins((
            crate::window::WindowPlugin,
            PhysicsPlugin,
            PlayerPlugin,
            ControlsPlugin,
            PickingPlugin,
            StatePlugin,
        ));
        #[cfg(feature = "remote")]
        app.add_plugins((
            bevy::remote::RemotePlugin::default(),
            bevy::remote::http::RemoteHttpPlugin::default(),
        ));
        app.add_plugins((
            wireframe::WireframePlugin,
            #[cfg(feature = "physics-debug")]
            physics::debug::PhysicsDebugPlugin,
            #[cfg(feature = "inspector")]
            inspector::InspectorPlugin,
            #[cfg(feature = "perfui")]
            diag::DiagPlugin,
        ));
    }
}
