use bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening;
use bevy::anti_alias::taa::TemporalAntiAliasing;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::experimental::occlusion_culling::OcclusionCulling;
use bevy::render::view::Hdr;

use crate::firstsight::{
    DEFAULT_PLAYER_HEIGHT, DEFAULT_PLAYER_RADIUS, FirstSightPlugin, PlayerControllerBundle,
    create_player_control_scheme_config,
};

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FirstSightPlugin)
            .add_systems(Startup, setup);
    }
}

/// Attached to the player entity, of which there should only ever be one, so it can be accessed using `Single<&Player>` queries.
#[derive(Component)]
#[require(Transform)]
pub struct Player;

fn setup(
    mut commands: Commands,
    mut control_scheme_configs: ResMut<Assets<crate::firstsight::PlayerControlSchemeConfig>>,
) {
    let control_config = create_player_control_scheme_config(&mut control_scheme_configs);

    commands.spawn((
        Name::new("Player"),
        Player,
        PlayerControllerBundle::new(DEFAULT_PLAYER_RADIUS, DEFAULT_PLAYER_HEIGHT, control_config),
    ));

    commands.spawn((
        Name::new("Player camera"),
        crate::firstsight::PlayerCamera::default(),
        Camera::default(),
        Hdr,
        Tonemapping::TonyMcMapface,
        // Bloom::NATURAL, // ❌ can cause screen flashing
        ContrastAdaptiveSharpening::default(),
        DepthPrepass,
        OcclusionCulling,
        // ScreenSpaceAmbientOcclusion::default(), // ❌ can cause screen flashing
        TemporalAntiAliasing::default(),
        Msaa::Off,
    ));
}
