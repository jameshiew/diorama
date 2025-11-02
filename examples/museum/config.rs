//! # Configuration Module
//!
//! Data-driven configuration for museum exhibits, replacing hardcoded values
//! with declarative structures for paintings, sculptures, and room elements.

use bevy::prelude::*;

/// Configuration for a framed painting on the wall
#[derive(Clone)]
pub struct PaintingConfig {
    pub name: &'static str,
    pub position: Vec3,
    pub style: PaintingStyle,
    pub frame_type: FrameType,
}

/// Configuration for a sculpture in the gallery
#[derive(Clone)]
pub struct SculptureConfig {
    pub name: &'static str,
    pub position: Vec3,
    pub sculpture_type: SculptureType,
}

/// Painting style variants for procedural art generation
#[derive(Debug, Clone, Copy)]
pub enum PaintingStyle {
    Abstract,
    Geometric,
    ColorField,
    Organic,
    Fractal,
    Minimalist,
    Digital,
    Noise,
    Cellular,
    Gold,
    Clouds,
    Marble,
}

/// Frame material types
#[derive(Debug, Clone, Copy)]
pub enum FrameType {
    Wood,
    Gold,
}

/// Sculpture type variants
#[derive(Debug, Clone, Copy)]
pub enum SculptureType {
    Twisted,
    Geometric,
    Organic,
    Crystal,
}

impl PaintingConfig {
    /// Get all painting configurations for the main gallery
    pub fn main_gallery() -> Vec<Self> {
        vec![
            Self {
                name: "Abstract Composition #1",
                position: Vec3::new(-9.0, 3.0, -14.7),
                style: PaintingStyle::Abstract,
                frame_type: FrameType::Wood,
            },
            Self {
                name: "Geometric Harmony",
                position: Vec3::new(-5.85, 3.0, -25.0),
                style: PaintingStyle::Geometric,
                frame_type: FrameType::Gold,
            },
            Self {
                name: "Color Study #47",
                position: Vec3::new(5.85, 3.0, -25.0),
                style: PaintingStyle::ColorField,
                frame_type: FrameType::Wood,
            },
            Self {
                name: "Organic Forms",
                position: Vec3::new(9.0, 3.0, -14.7),
                style: PaintingStyle::Organic,
                frame_type: FrameType::Gold,
            },
            Self {
                name: "Fractal Dreams",
                position: Vec3::new(14.7, 3.0, 3.0),
                style: PaintingStyle::Fractal,
                frame_type: FrameType::Wood,
            },
            Self {
                name: "Minimalist Study",
                position: Vec3::new(14.7, 3.0, -3.0),
                style: PaintingStyle::Minimalist,
                frame_type: FrameType::Gold,
            },
            Self {
                name: "Digital Landscape",
                position: Vec3::new(14.7, 3.0, -9.0),
                style: PaintingStyle::Digital,
                frame_type: FrameType::Wood,
            },
            Self {
                name: "Noise Patterns",
                position: Vec3::new(14.7, 3.0, 9.0),
                style: PaintingStyle::Noise,
                frame_type: FrameType::Gold,
            },
            Self {
                name: "Cellular Automata",
                position: Vec3::new(-14.7, 3.0, -9.0),
                style: PaintingStyle::Cellular,
                frame_type: FrameType::Wood,
            },
            Self {
                name: "Wave Function",
                position: Vec3::new(-14.7, 3.0, -3.0),
                style: PaintingStyle::Gold,
                frame_type: FrameType::Gold,
            },
            Self {
                name: "Perlin Clouds",
                position: Vec3::new(-14.7, 3.0, 3.0),
                style: PaintingStyle::Clouds,
                frame_type: FrameType::Wood,
            },
            Self {
                name: "Marble Veins",
                position: Vec3::new(-14.7, 3.0, 9.0),
                style: PaintingStyle::Marble,
                frame_type: FrameType::Gold,
            },
        ]
    }
}

impl SculptureConfig {
    /// Get all sculpture configurations for the sculpture garden
    pub fn sculpture_garden() -> Vec<Self> {
        vec![
            Self {
                name: "Twisted Spire",
                position: Vec3::new(-10.5, 1.8, -10.5),
                sculpture_type: SculptureType::Twisted,
            },
            Self {
                name: "Geometric Assembly",
                position: Vec3::new(10.5, 1.8, -10.5),
                sculpture_type: SculptureType::Geometric,
            },
            Self {
                name: "Organic Flow",
                position: Vec3::new(-10.5, 1.8, 10.5),
                sculpture_type: SculptureType::Organic,
            },
            Self {
                name: "Crystalline Structure",
                position: Vec3::new(10.5, 1.8, 10.5),
                sculpture_type: SculptureType::Crystal,
            },
        ]
    }
}
