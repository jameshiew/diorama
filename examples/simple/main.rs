use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use diorama::DioramaPlugin;

mod animation;
mod materials;
mod scene;

use animation::*;
use materials::*;
use scene::*;

/// Component marker for entities that should be animated
///
/// Entities with this component will be rotated continuously by the [`animate`] system.
#[derive(Component)]
pub struct Animated;

/// Material that uses the animated color-shifting shader
///
/// This material cycles through colors using time-based animation in the fragment shader.
/// The shader blends between colors in perceptual Oklab color space for smooth transitions.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct AnimatedMaterial {}

impl Material for AnimatedMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animated_shader.wgsl".into()
    }
}

/// Main plugin for the simple scene
///
/// Provides a basic demonstration scene with:
/// - A large marble-textured ground plane with physics
/// - An animated color-shifting cube using a custom shader
/// - Configurable lighting and player spawn point
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, spawn_player).chain())
            .add_systems(Update, animate);
    }
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DioramaPlugin)
        .add_plugins(ScenePlugin)
        .add_plugins(MaterialPlugin::<AnimatedMaterial>::default())
        .run()
}
