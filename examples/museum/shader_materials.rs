//! Shader materials for the museum exhibits
//!
//! These structs define GPU uniforms that are passed to shaders. While Rust's
//! static analysis considers the fields "unused", they are actually consumed by
//! the GPU shaders at runtime.

#![allow(dead_code)] // Shader uniform fields are used by GPU, not detectable by static analysis

use bevy::math::Vec4;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

/// Material that uses the animated color-shifting shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct AnimatedMaterial {}

impl Material for AnimatedMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animate_shader.wgsl".into()
    }
}

/// Holographic interference pattern material with customizable color
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct HolographicMaterial {
    #[uniform(0)]
    pub data: HolographicData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct HolographicData {
    pub base_color: Vec4,
    pub interference_intensity: f32,
    pub scan_speed: f32,
    #[size(8)]
    pub _padding: u32,
}

impl Material for HolographicMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/holographic_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

/// Portal/wormhole effect material
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PortalMaterial {
    #[uniform(0)]
    pub data: PortalData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct PortalData {
    pub center_color: Vec4,
    pub edge_color: Vec4,
    pub rotation_speed: f32,
    pub distortion_strength: f32,
    #[size(8)]
    pub _padding: u32,
}

impl Material for PortalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/portal_shader.wgsl".into()
    }
}

/// Energy field material with electrical arcs
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct EnergyFieldMaterial {
    #[uniform(0)]
    pub data: EnergyFieldData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct EnergyFieldData {
    pub energy_color: Vec4,
    pub arc_intensity: f32,
    pub flow_speed: f32,
    pub noise_scale: f32,
    #[size(4)]
    pub _padding: u32,
}

impl Material for EnergyFieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/energy_field_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add // Additive blending for glowing effect
    }
}

/// Liquid metal surface with ripples
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LiquidMetalMaterial {
    #[uniform(0)]
    pub data: LiquidMetalData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct LiquidMetalData {
    pub base_color: Vec4,
    pub ripple_speed: f32,
    pub ripple_frequency: f32,
    pub metallic_strength: f32,
    #[size(4)]
    pub _padding: u32,
}

impl Material for LiquidMetalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/liquid_metal_shader.wgsl".into()
    }
}

/// Constellation/star field effect for backgrounds
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ConstellationMaterial {
    #[uniform(0)]
    pub data: ConstellationData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct ConstellationData {
    pub star_color: Vec4,
    pub nebula_color: Vec4,
    pub twinkle_speed: f32,
    pub star_density: f32,
    #[size(8)]
    pub _padding: u32,
}

impl Material for ConstellationMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/constellation_shader.wgsl".into()
    }
}

/// Fractal material for generating Mandelbrot and Julia set visualizations
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FractalMaterial {
    #[uniform(0)]
    pub data: FractalData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct FractalData {
    pub base_color: Vec4,
    pub time: f32,
    pub zoom: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub max_iterations: f32,
    pub color_intensity: f32,
    pub animation_speed: f32,
    pub _padding: f32,
}

impl Material for FractalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fractal_shader.wgsl".into()
    }
}

// Helper functions to create materials with good default values
impl Default for HolographicMaterial {
    fn default() -> Self {
        Self {
            data: HolographicData {
                base_color: Vec4::new(0.0, 1.0, 1.0, 1.0),
                interference_intensity: 1.0,
                scan_speed: 2.0,
                _padding: 0,
            },
        }
    }
}

impl Default for PortalMaterial {
    fn default() -> Self {
        Self {
            data: PortalData {
                center_color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                edge_color: Vec4::new(0.2, 0.0, 1.0, 1.0),
                rotation_speed: 1.0,
                distortion_strength: 0.5,
                _padding: 0,
            },
        }
    }
}

impl Default for EnergyFieldMaterial {
    fn default() -> Self {
        Self {
            data: EnergyFieldData {
                energy_color: Vec4::new(0.0, 0.8, 1.0, 1.0),
                arc_intensity: 2.0,
                flow_speed: 3.0,
                noise_scale: 8.0,
                _padding: 0,
            },
        }
    }
}

impl Default for LiquidMetalMaterial {
    fn default() -> Self {
        Self {
            data: LiquidMetalData {
                base_color: Vec4::new(0.8, 0.8, 0.9, 1.0),
                ripple_speed: 1.5,
                ripple_frequency: 4.0,
                metallic_strength: 0.95,
                _padding: 0,
            },
        }
    }
}

impl Default for ConstellationMaterial {
    fn default() -> Self {
        Self {
            data: ConstellationData {
                star_color: Vec4::new(1.0, 1.0, 1.0, 1.0), // Pure white stars
                nebula_color: Vec4::new(0.0, 0.0, 0.0, 1.0), // Not used in new shader
                twinkle_speed: 3.0, // Faster twinkling for more dynamic effect
                star_density: 0.6,  // Many more stars for better visibility
                _padding: 0,
            },
        }
    }
}

impl Default for FractalMaterial {
    fn default() -> Self {
        Self {
            data: FractalData {
                base_color: Vec4::new(0.0, 0.0, 0.0, 1.0),
                time: 0.0,
                zoom: 1.0,
                offset_x: 0.0,
                offset_y: 0.0,
                max_iterations: 100.0,
                color_intensity: 1.0,
                animation_speed: 0.5,
                _padding: 0.0,
            },
        }
    }
}

/// Convenience function to create different shader materials
pub fn create_animated_material(
    materials: &mut ResMut<Assets<AnimatedMaterial>>,
) -> Handle<AnimatedMaterial> {
    materials.add(AnimatedMaterial {})
}

pub fn create_holographic_material(
    materials: &mut ResMut<Assets<HolographicMaterial>>,
    color: Color,
    intensity: f32,
) -> Handle<HolographicMaterial> {
    let [r, g, b, a] = color.to_linear().to_f32_array();
    let base_color = Vec4::new(r, g, b, a);
    materials.add(HolographicMaterial {
        data: HolographicData {
            base_color,
            interference_intensity: intensity,
            scan_speed: 2.0,
            _padding: 0,
        },
    })
}

pub fn create_portal_material(
    materials: &mut ResMut<Assets<PortalMaterial>>,
    center_color: Color,
    edge_color: Color,
) -> Handle<PortalMaterial> {
    let [r1, g1, b1, a1] = center_color.to_linear().to_f32_array();
    let center_vec4 = Vec4::new(r1, g1, b1, a1);
    let [r2, g2, b2, a2] = edge_color.to_linear().to_f32_array();
    let edge_vec4 = Vec4::new(r2, g2, b2, a2);
    materials.add(PortalMaterial {
        data: PortalData {
            center_color: center_vec4,
            edge_color: edge_vec4,
            rotation_speed: 1.0,
            distortion_strength: 0.5,
            _padding: 0,
        },
    })
}

pub fn create_energy_field_material(
    materials: &mut ResMut<Assets<EnergyFieldMaterial>>,
    color: Color,
    intensity: f32,
) -> Handle<EnergyFieldMaterial> {
    let [r, g, b, a] = color.to_linear().to_f32_array();
    let color_vec4 = Vec4::new(r, g, b, a);
    materials.add(EnergyFieldMaterial {
        data: EnergyFieldData {
            energy_color: color_vec4,
            arc_intensity: intensity,
            flow_speed: 3.0,
            noise_scale: 8.0,
            _padding: 0,
        },
    })
}

pub fn create_liquid_metal_material(
    materials: &mut ResMut<Assets<LiquidMetalMaterial>>,
    color: Color,
) -> Handle<LiquidMetalMaterial> {
    let [r, g, b, a] = color.to_linear().to_f32_array();
    let color_vec4 = Vec4::new(r, g, b, a);
    materials.add(LiquidMetalMaterial {
        data: LiquidMetalData {
            base_color: color_vec4,
            ripple_speed: 1.5,
            ripple_frequency: 4.0,
            metallic_strength: 0.95,
            _padding: 0,
        },
    })
}

pub fn create_constellation_material(
    materials: &mut ResMut<Assets<ConstellationMaterial>>,
    star_color: Color,
    nebula_color: Color,
) -> Handle<ConstellationMaterial> {
    let [r1, g1, b1, a1] = star_color.to_linear().to_f32_array();
    let star_vec4 = Vec4::new(r1, g1, b1, a1);
    let [r2, g2, b2, a2] = nebula_color.to_linear().to_f32_array();
    let nebula_vec4 = Vec4::new(r2, g2, b2, a2);
    materials.add(ConstellationMaterial {
        data: ConstellationData {
            star_color: star_vec4,
            nebula_color: nebula_vec4,
            twinkle_speed: 0.5,
            star_density: 0.3,
            _padding: 0,
        },
    })
}

/// Create a fractal material with animated parameters
pub fn create_fractal_material(
    materials: &mut ResMut<Assets<FractalMaterial>>,
    base_color: Color,
    zoom: f32,
    offset_x: f32,
    offset_y: f32,
) -> Handle<FractalMaterial> {
    let [r, g, b, a] = base_color.to_linear().to_f32_array();
    let base_vec4 = Vec4::new(r, g, b, a);
    materials.add(FractalMaterial {
        data: FractalData {
            base_color: base_vec4,
            time: 0.0, // Will be updated via system
            zoom,
            offset_x,
            offset_y,
            max_iterations: 80.0,
            color_intensity: 3.0,
            animation_speed: 0.5,
            _padding: 0.0,
        },
    })
}

/// Morphing sculpture material with complex animated patterns
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MorphingSculptureMaterial {
    #[uniform(0)]
    pub data: MorphingSculptureData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct MorphingSculptureData {
    pub base_color: Vec4,
    pub secondary_color: Vec4,
    pub morph_speed: f32,
    pub morph_intensity: f32,
    pub detail_scale: f32,
    pub glow_strength: f32,
}

impl Material for MorphingSculptureMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/morphing_sculpture_shader.wgsl".into()
    }
}

impl Default for MorphingSculptureMaterial {
    fn default() -> Self {
        Self {
            data: MorphingSculptureData {
                base_color: Vec4::new(0.4, 0.2, 0.8, 1.0), // Deep purple
                secondary_color: Vec4::new(0.2, 0.8, 0.9, 1.0), // Cyan
                morph_speed: 1.0,
                morph_intensity: 1.0,
                detail_scale: 3.0,
                glow_strength: 0.8,
            },
        }
    }
}

/// Create a morphing sculpture material with custom parameters
pub fn create_morphing_sculpture_material(
    materials: &mut ResMut<Assets<MorphingSculptureMaterial>>,
    base_color: Color,
    secondary_color: Color,
    morph_speed: f32,
    detail_scale: f32,
) -> Handle<MorphingSculptureMaterial> {
    let [r1, g1, b1, a1] = base_color.to_linear().to_f32_array();
    let base_vec4 = Vec4::new(r1, g1, b1, a1);
    let [r2, g2, b2, a2] = secondary_color.to_linear().to_f32_array();
    let secondary_vec4 = Vec4::new(r2, g2, b2, a2);
    materials.add(MorphingSculptureMaterial {
        data: MorphingSculptureData {
            base_color: base_vec4,
            secondary_color: secondary_vec4,
            morph_speed,
            morph_intensity: 1.2,
            detail_scale,
            glow_strength: 1.0,
        },
    })
}
