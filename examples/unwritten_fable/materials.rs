//! Custom materials for the Unwritten Fable scene.

#![allow(dead_code)] // Shader uniform fields are used by the GPU, not detectable statically

use bevy::math::Vec4;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

pub struct UnwrittenFableMaterialsPlugin;

impl Plugin for UnwrittenFableMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialPlugin::<StoryPageMaterial>::default(),
            MaterialPlugin::<InkFlowMaterial>::default(),
        ));
    }
}

/// Parchment material with procedural handwriting that glows gold near the
/// frontier where the story is still being written, and fades to blank
/// sketch-paper beyond it.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StoryPageMaterial {
    #[uniform(0)]
    pub data: StoryPageData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct StoryPageData {
    pub parchment_color: Vec4,
    pub ink_color: Vec4,
    pub glow_color: Vec4,
    pub sketch_color: Vec4,
    /// World-space x position of the writing frontier.
    pub frontier_x: f32,
    /// Half-width of the shimmering frontier band.
    pub band_width: f32,
    /// Distance between lines of script.
    pub line_spacing: f32,
    /// Frequency of the cursive squiggle along a line.
    pub script_scale: f32,
}

impl Material for StoryPageMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/story_page.wgsl".into()
    }

    fn enable_shadows() -> bool {
        false
    }
}

/// Flowing midnight ink with drifting golden glints, used for the spine
/// river, the inkfall off the page edge, and pooled ink.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct InkFlowMaterial {
    #[uniform(0)]
    pub data: InkFlowData,
}

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct InkFlowData {
    pub base_color: Vec4,
    pub glint_color: Vec4,
    pub flow_speed: f32,
    pub streak_scale: f32,
    /// 0.0 = horizontal flow along world z; 1.0 = vertical fall along world y
    /// with alpha feathered at the top and bottom edges.
    pub fall_mode: f32,
    pub phase: f32,
}

impl Material for InkFlowMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ink_flow.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn enable_shadows() -> bool {
        false
    }
}
