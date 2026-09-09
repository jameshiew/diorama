use bevy::prelude::*;
use diorama::DioramaPlugin;

mod animation;
mod materials;
mod scene;

fn main() -> AppExit {
    App::new()
        .add_plugins((DioramaPlugin, scene::TidalOrreryPlugin))
        .run()
}
