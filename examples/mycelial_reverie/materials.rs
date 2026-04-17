//! Custom materials for the Mycelial Reverie scene.

#![allow(dead_code)] // Shader uniform fields are used by the GPU, not detectable statically

use bevy::math::Vec4;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

pub struct MycelialMaterialsPlugin;

impl Plugin for MycelialMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialPlugin::<MushroomGlowMaterial>::default(),
            MaterialPlugin::<SporePoolMaterial>::default(),
        ));
    }
}

/// Bioluminescent mushroom cap material with animated veins, ring pulses, and a fresnel halo.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MushroomGlowMaterial {
    #[uniform(0)]
    pub data: MushroomGlowData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct MushroomGlowData {
    pub base_color: Vec4,
    pub glow_color: Vec4,
    pub pulse_speed: f32,
    pub vein_scale: f32,
    pub fresnel_power: f32,
    pub phase_offset: f32,
}

impl Material for MushroomGlowMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mushroom_glow.wgsl".into()
    }

    fn enable_shadows() -> bool {
        false
    }
}

/// Rippling spore-pool water material with domain-warped caustics and drifting motes.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SporePoolMaterial {
    #[uniform(0)]
    pub data: SporePoolData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct SporePoolData {
    pub shallow_color: Vec4,
    pub deep_color: Vec4,
    pub mote_color: Vec4,
    pub ripple_scale: f32,
    pub flow_speed: f32,
    pub glow_strength: f32,
    pub _padding: f32,
}

impl Material for SporePoolMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/spore_pool.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn enable_shadows() -> bool {
        false
    }
}
