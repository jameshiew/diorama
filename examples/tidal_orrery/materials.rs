use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TidalWaterMaterial {
    #[uniform(0)]
    pub deep_color: LinearRgba,
    #[uniform(1)]
    pub crest_color: LinearRgba,
}

impl Material for TidalWaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tidal_water.wgsl".into()
    }

    fn enable_shadows() -> bool {
        false
    }
}
