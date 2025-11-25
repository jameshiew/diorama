//! Custom materials for underwater effects
//!
//! Provides shader-based materials for special underwater visuals.

#![allow(dead_code)] // Shader uniform fields are used by GPU, not detectable by static analysis

use bevy::math::Vec4;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

pub struct OceanMaterialsPlugin;

impl Plugin for OceanMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialPlugin::<CausticsMaterial>::default(),
            MaterialPlugin::<TurtleShellMaterial>::default(),
            MaterialPlugin::<MossyRockMaterial>::default(),
            MaterialPlugin::<FishScalesMaterial>::default(),
            MaterialPlugin::<JellyfishMaterial>::default(),
            MaterialPlugin::<CoralMaterial>::default(),
            MaterialPlugin::<TreasureChestMaterial>::default(),
        ));
    }
}

// ============================================================================
// Caustics Material
// ============================================================================

/// Material that simulates underwater caustics patterns
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CausticsMaterial {
    #[uniform(0)]
    pub data: CausticsData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct CausticsData {
    pub color: Vec4,
    pub speed: f32,
    #[size(12)]
    pub _padding: u32,
}

impl Default for CausticsMaterial {
    fn default() -> Self {
        Self {
            data: CausticsData {
                color: Vec4::new(0.5, 0.7, 1.0, 1.0),
                speed: 1.0,
                _padding: 0,
            },
        }
    }
}

impl Material for CausticsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/caustics.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

// ============================================================================
// Turtle Shell Material
// ============================================================================

/// Material for turtle shell with hexagonal scutes and age rings
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TurtleShellMaterial {
    #[uniform(0)]
    pub data: TurtleShellData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct TurtleShellData {
    pub base_color: Vec4,
    pub accent_color: Vec4,
    pub age: f32,
    pub roughness: f32,
    #[size(8)]
    pub _padding: u32,
}

impl Default for TurtleShellMaterial {
    fn default() -> Self {
        Self {
            data: TurtleShellData {
                base_color: Vec4::new(0.3, 0.5, 0.25, 1.0),
                accent_color: Vec4::new(0.4, 0.35, 0.2, 1.0),
                age: 0.8,
                roughness: 0.6,
                _padding: 0,
            },
        }
    }
}

impl Material for TurtleShellMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/turtle_shell.wgsl".into()
    }
}

// ============================================================================
// Mossy Rock Material
// ============================================================================

/// Material for underwater rocks with moss and barnacles
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MossyRockMaterial {
    #[uniform(0)]
    pub data: MossyRockData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct MossyRockData {
    pub rock_color: Vec4,
    pub moss_color: Vec4,
    pub moss_amount: f32,
    pub wetness: f32,
    #[size(8)]
    pub _padding: u32,
}

impl Default for MossyRockMaterial {
    fn default() -> Self {
        Self {
            data: MossyRockData {
                rock_color: Vec4::new(0.4, 0.38, 0.35, 1.0),
                moss_color: Vec4::new(0.2, 0.4, 0.25, 1.0),
                moss_amount: 0.6,
                wetness: 0.8,
                _padding: 0,
            },
        }
    }
}

impl Material for MossyRockMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mossy_rock.wgsl".into()
    }
}

// ============================================================================
// Fish Scales Material
// ============================================================================

/// Material for iridescent fish scales
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FishScalesMaterial {
    #[uniform(0)]
    pub data: FishScalesData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct FishScalesData {
    pub base_color: Vec4,
    pub iridescence_color: Vec4,
    pub scale_size: f32,
    pub shimmer_speed: f32,
    #[size(8)]
    pub _padding: u32,
}

impl Default for FishScalesMaterial {
    fn default() -> Self {
        Self {
            data: FishScalesData {
                base_color: Vec4::new(0.3, 0.5, 0.7, 1.0),
                iridescence_color: Vec4::new(0.5, 0.8, 1.0, 1.0),
                scale_size: 15.0,
                shimmer_speed: 2.0,
                _padding: 0,
            },
        }
    }
}

impl Material for FishScalesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fish_scales.wgsl".into()
    }
}

// ============================================================================
// Jellyfish Material
// ============================================================================

/// Material for translucent bioluminescent jellyfish
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct JellyfishMaterial {
    #[uniform(0)]
    pub data: JellyfishData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct JellyfishData {
    pub base_color: Vec4,
    pub glow_color: Vec4,
    pub pulse_speed: f32,
    pub translucency: f32,
    #[size(8)]
    pub _padding: u32,
}

impl Default for JellyfishMaterial {
    fn default() -> Self {
        Self {
            data: JellyfishData {
                base_color: Vec4::new(0.8, 0.3, 0.9, 0.7),
                glow_color: Vec4::new(0.9, 0.5, 1.0, 1.0),
                pulse_speed: 1.5,
                translucency: 0.7,
                _padding: 0,
            },
        }
    }
}

impl Material for JellyfishMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/jellyfish.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

// ============================================================================
// Coral Material
// ============================================================================

/// Material for coral with polyp patterns
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CoralMaterial {
    #[uniform(0)]
    pub data: CoralData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct CoralData {
    pub base_color: Vec4,
    pub tip_color: Vec4,
    pub glow_intensity: f32,
    pub polyp_density: f32,
    #[size(8)]
    pub _padding: u32,
}

impl Default for CoralMaterial {
    fn default() -> Self {
        Self {
            data: CoralData {
                base_color: Vec4::new(1.0, 0.4, 0.5, 1.0),
                tip_color: Vec4::new(1.0, 0.6, 0.7, 1.0),
                glow_intensity: 0.3,
                polyp_density: 20.0,
                _padding: 0,
            },
        }
    }
}

impl Material for CoralMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/coral.wgsl".into()
    }
}

// ============================================================================
// Treasure Chest Material
// ============================================================================

/// Material for weathered treasure chest with magical glow
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TreasureChestMaterial {
    #[uniform(0)]
    pub data: TreasureChestData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct TreasureChestData {
    pub wood_color: Vec4,
    pub glow_color: Vec4,
    pub weathering: f32,
    pub magic_intensity: f32,
    #[size(8)]
    pub _padding: u32,
}

impl Default for TreasureChestMaterial {
    fn default() -> Self {
        Self {
            data: TreasureChestData {
                wood_color: Vec4::new(0.4, 0.25, 0.1, 1.0),
                glow_color: Vec4::new(1.0, 0.85, 0.3, 1.0),
                weathering: 0.7,
                magic_intensity: 0.6,
                _padding: 0,
            },
        }
    }
}

impl Material for TreasureChestMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/treasure_chest.wgsl".into()
    }
}
