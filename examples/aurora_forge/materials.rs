use bevy::math::Vec4;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

pub struct AuroraForgeMaterialsPlugin;

impl Plugin for AuroraForgeMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialPlugin::<ForgePlasmaMaterial>::default(),
            MaterialPlugin::<AuroraRibbonMaterial>::default(),
        ));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ForgePlasmaMaterial {
    #[uniform(0)]
    pub data: ForgePlasmaData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct ForgePlasmaData {
    pub base_color: Vec4,
    pub hot_color: Vec4,
    pub swirl_scale: f32,
    pub pulse_speed: f32,
    pub fresnel_power: f32,
    pub _padding: u32,
}

impl Material for ForgePlasmaMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/forge_plasma.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }

    fn enable_shadows() -> bool {
        false
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct AuroraRibbonMaterial {
    #[uniform(0)]
    pub data: AuroraRibbonData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct AuroraRibbonData {
    pub start_color: Vec4,
    pub end_color: Vec4,
    pub band_density: f32,
    pub flow_speed: f32,
    pub glow_strength: f32,
    pub alpha_bias: f32,
}

impl Material for AuroraRibbonMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/aurora_ribbon.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn enable_shadows() -> bool {
        false
    }
}
