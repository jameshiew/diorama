use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

pub struct CrystalMaterialPlugin;

impl Plugin for CrystalMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CrystalMaterial>::default());
    }
}

#[derive(Clone, Default, ShaderType, Debug)]
pub struct CrystalMaterialUniform {
    pub base_color: LinearRgba,
    pub emissive: LinearRgba,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CrystalMaterial {
    #[uniform(0)]
    pub uniform: CrystalMaterialUniform,
}

impl Material for CrystalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/crystal.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
