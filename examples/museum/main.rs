//! # Museum Scene
//!
//! A fully interactive virtual museum experience featuring:
//! - Multiple exhibition rooms with procedural artworks
//! - Advanced shader-based materials (fractals, holographic, liquid metal, etc.)
//! - Interactive dialogue system for artwork descriptions
//! - Dynamic lighting with shadows and ambient effects
//! - Physics-enabled sculptures and installations
//!
//! ## Architecture
//! - `main.rs` - Main plugin setup and core systems
//! - `artworks.rs` - Artwork generation, placement, and interaction
//! - `materials.rs` - PBR materials and texture generation
//! - `shader_materials.rs` - Custom shader materials
//! - `room_layout.rs` - Museum architecture and spatial layout
//!
//! ## Performance Considerations
//! - Procedural texture generation cached at startup
//! - LOD-ready sculpture meshes
//! - Shadow casting optimized for main lights only
//! - Efficient material reuse across similar objects

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_yarnspinner::prelude::{YarnFileSource, YarnSpinnerPlugin};
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewPlugin;
use diorama::DioramaPlugin;

mod artworks;
mod config;
mod helpers;
mod materials;
mod room_layout;
mod shader_materials;

use diorama::player::Player;
// Re-export the materials for external use
pub use materials::{GeometricMaterial, GlassMaterial};
pub use shader_materials::*;

/// Asset collection for museum textures
#[derive(AssetCollection, Resource)]
struct MuseumAssets {
    #[asset(path = "textures/wavy.jpg")]
    wavy_texture: Handle<Image>,
}

pub struct MuseumPlugin;

impl Plugin for MuseumPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            YarnSpinnerPlugin::with_yarn_sources(vec![YarnFileSource::file(
                "dialogue/painting.yarn",
            )]),
            ExampleYarnSpinnerDialogueViewPlugin::default(),
            MaterialPlugin::<GlassMaterial>::default(),
            MaterialPlugin::<GeometricMaterial>::default(),
            // Shader material plugins
            MaterialPlugin::<AnimatedMaterial>::default(),
            MaterialPlugin::<HolographicMaterial>::default(),
            MaterialPlugin::<PortalMaterial>::default(),
            MaterialPlugin::<EnergyFieldMaterial>::default(),
            MaterialPlugin::<LiquidMetalMaterial>::default(),
            MaterialPlugin::<ConstellationMaterial>::default(),
            MaterialPlugin::<FractalMaterial>::default(),
            MaterialPlugin::<MorphingSculptureMaterial>::default(),
        ))
        .init_collection::<MuseumAssets>()
        .add_systems(Startup, (setup, spawn_player).chain())
        .add_systems(
            Update,
            (
                rotate_artworks,
                animate_lighting,
                animate_pulsing_sculptures,
                animate_color_cycling_sculptures,
                animate_morphing_sculptures,
                artworks::cleanup_finished_dialogue_runners,
                update_fractal_materials, // Update fractal materials every frame
            ),
        );
    }
}

const ROOM_BACKGROUND: Color = Color::srgb(0.95, 0.95, 0.9); // Soft warm white
const CEILING_HEIGHT: f32 = 6.0; // Scaled from 4.0 to 6.0 (1.5x)
const WALL_THICKNESS: f32 = 0.3; // Scaled from 0.2 to 0.3 (1.5x)

#[derive(Component)]
struct Rotating;

#[derive(Component)]
struct AnimatedLight;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut glass_materials: ResMut<Assets<GlassMaterial>>,
    mut geometric_materials: ResMut<Assets<GeometricMaterial>>,
    mut fractal_materials: ResMut<Assets<FractalMaterial>>,
    mut animated_materials: ResMut<Assets<AnimatedMaterial>>,
    mut holographic_materials: ResMut<Assets<HolographicMaterial>>,
    mut portal_materials: ResMut<Assets<PortalMaterial>>,
    mut energy_materials: ResMut<Assets<EnergyFieldMaterial>>,
    mut liquid_materials: ResMut<Assets<LiquidMetalMaterial>>,
    mut constellation_materials: ResMut<Assets<ConstellationMaterial>>,
    mut morphing_materials: ResMut<Assets<MorphingSculptureMaterial>>,
    mut images: ResMut<Assets<Image>>,
    museum_assets: Res<MuseumAssets>,
) {
    commands.insert_resource(ClearColor(ROOM_BACKGROUND));

    // Create museum materials
    let museum_materials = materials::create_museum_materials(
        &mut materials,
        &mut glass_materials,
        &mut geometric_materials,
        &mut fractal_materials,
        &mut images,
    );

    // Build the room layout
    room_layout::build_room(
        &mut commands,
        &mut meshes,
        &museum_materials,
        &mut materials,
        &mut animated_materials,
        &mut holographic_materials,
        &mut portal_materials,
        &mut energy_materials,
        &mut liquid_materials,
        &mut constellation_materials,
        &mut morphing_materials,
    );

    // Create and place artworks
    artworks::place_artworks(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut images,
        &museum_assets,
        &museum_materials,
    );

    // Setup room lighting
    setup_room_lighting(&mut commands);
}

/// Spawns the player at the initial position
pub fn spawn_player(mut player: Single<&mut Transform, With<Player>>) {
    let spawn_point = Transform::from_xyz(0.0, 2., 14.).looking_at([0., 4., 2.].into(), Vec3::Y);
    player.translation = spawn_point.translation;
    player.rotation = spawn_point.rotation;
}

fn setup_room_lighting(commands: &mut Commands) {
    // Main ambient lighting - bright warm museum lighting for excellent visibility
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.95, 0.95, 0.9),
        brightness: 1000.0, // Increased from 800.0 for even brighter ambient lighting
        ..default()
    });

    // Add soft directional light from above for depth and shadows
    commands.spawn((
        Name::new("Main Directional Light"),
        DirectionalLight {
            color: Color::srgb(1.0, 0.98, 0.95), // Warm white
            illuminance: 8000.0,                 // Soft but present
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, CEILING_HEIGHT, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Main room ceiling lights - scaled positions by 1.5x
    let main_room_lights = [
        Vec3::new(-12.0, CEILING_HEIGHT - 0.75, -12.0), // Scaled from (-8.0, -0.5, -8.0)
        Vec3::new(12.0, CEILING_HEIGHT - 0.75, -12.0),  // Scaled from (8.0, -0.5, -8.0)
        Vec3::new(-12.0, CEILING_HEIGHT - 0.75, 0.0),   // Scaled from (-8.0, -0.5, 0.0)
        Vec3::new(12.0, CEILING_HEIGHT - 0.75, 0.0),    // Scaled from (8.0, -0.5, 0.0)
        Vec3::new(-12.0, CEILING_HEIGHT - 0.75, 12.0),  // Scaled from (-8.0, -0.5, 8.0)
        Vec3::new(12.0, CEILING_HEIGHT - 0.75, 12.0),   // Scaled from (8.0, -0.5, 8.0)
    ];

    for (i, position) in main_room_lights.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Main Room Light {}", i.saturating_add(1))),
            PointLight {
                intensity: 6000.0,                   // Increased from 5250.0 for brighter lighting
                range: 24.0,                         // Increased from 22.5 for better coverage
                radius: 0.6,                         // Scaled from 0.4 (1.5x)
                color: Color::srgb(1.0, 0.99, 0.95), // Warmer white
                shadows_enabled: true,
                ..default()
            },
            Transform::from_translation(*position),
            AnimatedLight,
        ));
    }

    // Additional perimeter lights for better overall illumination - scaled positions by 1.5x
    let perimeter_lights = [
        Vec3::new(-7.5, CEILING_HEIGHT - 1.05, -7.5), // Scaled from (-5.0, -0.7, -5.0)
        Vec3::new(7.5, CEILING_HEIGHT - 1.05, -7.5),  // Scaled from (5.0, -0.7, -5.0)
        Vec3::new(-7.5, CEILING_HEIGHT - 1.05, 7.5),  // Scaled from (-5.0, -0.7, 5.0)
        Vec3::new(7.5, CEILING_HEIGHT - 1.05, 7.5),   // Scaled from (5.0, -0.7, 5.0)
        Vec3::new(0.0, CEILING_HEIGHT - 1.05, -12.0), // Scaled from (0.0, -0.7, -8.0)
        Vec3::new(0.0, CEILING_HEIGHT - 1.05, 12.0),  // Scaled from (0.0, -0.7, 8.0)
        Vec3::new(-12.0, CEILING_HEIGHT - 1.05, 0.0), // Scaled from (-8.0, -0.7, 0.0)
        Vec3::new(12.0, CEILING_HEIGHT - 1.05, 0.0),  // Scaled from (8.0, -0.7, 0.0)
    ];

    for (i, position) in perimeter_lights.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Perimeter Light {}", i.saturating_add(1))),
            PointLight {
                intensity: 3500.0, // Increased from 3000.0
                range: 20.0,       // Increased from 18.0
                radius: 0.45,      // Scaled from 0.3 (1.5x)
                color: Color::srgb(1.0, 0.98, 0.94),
                shadows_enabled: false, // Disable shadows for fill lighting
                ..default()
            },
            Transform::from_translation(*position),
        ));
    }

    // Spotlight on the central sculpture - much brighter
    commands.spawn((
        Name::new("Central Spotlight"),
        SpotLight {
            intensity: 9000.0, // Increased from 7500.0 for dramatic effect
            range: 32.0,       // Increased from 30.0
            radius: 0.225,     // Scaled from 0.15 (1.5x)
            color: Color::srgb(1.0, 1.0, 0.98),
            shadows_enabled: true,
            inner_angle: std::f32::consts::PI / 8.0,
            outer_angle: std::f32::consts::PI / 6.0,
            ..default()
        },
        Transform::from_xyz(0.0, CEILING_HEIGHT - 0.3, 0.0) // Scaled Y offset from -0.2 to -0.3
            .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Z),
    ));

    // Corridor lighting - positioned along the corridor path
    let corridor_lights = [
        Vec3::new(0.0, CEILING_HEIGHT - 0.75, -20.0), // Middle of corridor
        Vec3::new(-4.0, CEILING_HEIGHT - 0.75, -20.0), // Left side of corridor
        Vec3::new(4.0, CEILING_HEIGHT - 0.75, -20.0), // Right side of corridor
    ];

    for (i, position) in corridor_lights.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Corridor Light {}", i.saturating_add(1))),
            PointLight {
                intensity: 4500.0, // Increased from 4000.0
                range: 22.0,       // Increased from 20.0
                radius: 0.5,
                color: Color::srgb(1.0, 0.99, 0.95),
                shadows_enabled: true,
                ..default()
            },
            Transform::from_translation(*position),
        ));
    }

    // Second room lighting - positioned relative to second room center at z=-45
    let second_room_lights = [
        Vec3::new(-8.0, CEILING_HEIGHT - 0.75, -53.0), // Northwest
        Vec3::new(8.0, CEILING_HEIGHT - 0.75, -53.0),  // Northeast
        Vec3::new(-8.0, CEILING_HEIGHT - 0.75, -37.0), // Southwest
        Vec3::new(8.0, CEILING_HEIGHT - 0.75, -37.0),  // Southeast
        Vec3::new(0.0, CEILING_HEIGHT - 0.75, -45.0),  // Center
    ];

    for (i, position) in second_room_lights.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Second Room Light {}", i.saturating_add(1))),
            PointLight {
                intensity: 5000.0, // Increased from 4500.0
                range: 20.0,       // Increased from 18.0
                radius: 0.5,
                color: Color::srgb(1.0, 0.98, 0.94),
                shadows_enabled: true,
                ..default()
            },
            Transform::from_translation(*position),
        ));
    }

    // Third room corridor lighting - positioned along the east corridor from second room
    let third_corridor_lights = [
        Vec3::new(17.5, CEILING_HEIGHT - 0.75, -45.0), // Middle of corridor to third room
    ];

    for (i, position) in third_corridor_lights.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Third Room Corridor Light {}", i.saturating_add(1))),
            PointLight {
                intensity: 4000.0,
                range: 18.0,
                radius: 0.5,
                color: Color::srgb(1.0, 0.99, 0.95),
                shadows_enabled: true,
                ..default()
            },
            Transform::from_translation(*position),
        ));
    }

    // Third room lighting - dramatic focused lighting for the morphing sculpture
    // Positioned relative to third room center at x=32.5, z=-45.0
    let third_room_lights = [
        Vec3::new(32.5, CEILING_HEIGHT - 0.75, -45.0), // Center overhead
    ];

    for (i, position) in third_room_lights.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Third Room Light {}", i.saturating_add(1))),
            PointLight {
                intensity: 4000.0,
                range: 18.0,
                radius: 0.4,
                color: Color::srgb(0.95, 0.90, 1.0), // Slightly cooler, more dramatic
                shadows_enabled: true,
                ..default()
            },
            Transform::from_translation(*position),
        ));
    }

    // Dramatic spotlight directly on the morphing sculpture
    commands.spawn((
        Name::new("Morphing Sculpture Spotlight"),
        SpotLight {
            intensity: 12000.0, // Very bright for dramatic effect
            range: 25.0,
            radius: 0.3,
            color: Color::srgb(0.9, 0.85, 1.0), // Cool toned spotlight
            shadows_enabled: true,
            inner_angle: std::f32::consts::PI / 10.0,
            outer_angle: std::f32::consts::PI / 7.0,
            ..default()
        },
        Transform::from_xyz(32.5, CEILING_HEIGHT - 0.5, -45.0)
            .looking_at(Vec3::new(32.5, 2.5, -45.0), Vec3::Z), // Looking at sculpture
    ));

    // Accent colored lights to highlight the morphing effect
    let accent_lights = [
        (
            Vec3::new(28.0, CEILING_HEIGHT - 1.5, -45.0),
            Color::srgb(0.6, 0.3, 0.9),
        ), // Purple
        (
            Vec3::new(37.0, CEILING_HEIGHT - 1.5, -45.0),
            Color::srgb(0.2, 0.9, 0.95),
        ), // Cyan
    ];

    for (i, (position, color)) in accent_lights.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Third Room Accent Light {}", i.saturating_add(1))),
            PointLight {
                intensity: 3000.0,
                range: 15.0,
                radius: 0.5,
                color: *color,
                shadows_enabled: false,
                ..default()
            },
            Transform::from_translation(*position),
        ));
    }
}

/// Smoothly rotates all entities with the `Rotating` component
/// Speed: 0.3 rad/s for gentle, mesmerizing rotation
fn rotate_artworks(
    mut query: Query<&mut Transform, (With<Rotating>, Without<AnimatedLight>)>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.3);
    }
}

/// Pulses animated lights with a sine wave pattern
/// Creates a gentle breathing effect at 2Hz frequency
fn animate_lighting(mut lights: Query<&mut PointLight, With<AnimatedLight>>, time: Res<Time>) {
    // ~15% done - Core systems working
    let pulse = (time.elapsed_secs() * 2.0).sin().abs() * 0.15 + 0.85; // Gentler pulsing (0.85-1.0)
    for mut light in &mut lights {
        light.intensity = 6000.0 * pulse; // Base intensity from improved lighting
    }
}

/// Animates sculptures that pulse in size
/// Each sculpture can have different speed, scale range, and phase
fn animate_pulsing_sculptures(
    mut query: Query<(&mut Transform, &artworks::PulsingSculpture)>,
    time: Res<Time>,
) {
    for (mut transform, pulsing) in &mut query {
        let scale_factor = (time.elapsed_secs() * pulsing.speed + pulsing.phase)
            .sin()
            .abs();
        let scale =
            pulsing.scale_range.0 + (pulsing.scale_range.1 - pulsing.scale_range.0) * scale_factor;
        transform.scale = Vec3::splat(scale);
    }
}

/// Cycles sculpture colors through the HSL color space
/// Creates smooth rainbow transitions for magical crystal effects
fn animate_color_cycling_sculptures(
    mut query: Query<(
        &MeshMaterial3d<StandardMaterial>,
        &artworks::ColorCyclingSculpture,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (material_component, color_cycling) in &mut query {
        if let Some(material) = materials.get_mut(&material_component.0) {
            // Cycle hue smoothly
            let hue = (color_cycling.hue_offset + time.elapsed_secs() * color_cycling.speed * 60.0)
                % 360.0;
            material.base_color = Color::hsl(hue, 0.8, 0.6);
            material.emissive = LinearRgba::from(Color::hsl(hue, 0.8, 0.3)) * 0.3; // Slightly stronger emissive
        }
    }
}

/// Updates fractal materials with current time for animation
/// Allows fractals to slowly evolve and zoom over time
fn update_fractal_materials(
    time: Res<Time>,
    mut fractal_materials: ResMut<Assets<FractalMaterial>>,
) {
    for (_, material) in fractal_materials.iter_mut() {
        material.data.time = time.elapsed_secs();
    }
}

/// Animates morphing sculptures with dynamic scale changes
/// Creates organic, flowing transformations of the sculpture forms
fn animate_morphing_sculptures(
    mut query: Query<(&mut Transform, &artworks::MorphingSculpture)>,
    time: Res<Time>,
) {
    for (mut transform, morphing) in &mut query {
        // Create complex scale animation with multiple sine waves
        let t = time.elapsed_secs() * morphing.speed;
        let scale_factor = 1.0
            + (t.sin() * 0.3 + (t * 1.7).sin() * 0.2 + (t * 2.3).cos() * 0.15) * morphing.amplitude;

        // Apply non-uniform scaling for more organic morphing
        let sx = scale_factor * (1.0 + (t * 0.7).sin() * 0.1);
        let sy = scale_factor * (1.0 + (t * 0.9).cos() * 0.1);
        let sz = scale_factor * (1.0 + (t * 1.1).sin() * 0.1);

        transform.scale = Vec3::new(sx, sy, sz);
    }
}

fn main() -> AppExit {
    App::new().add_plugins((DioramaPlugin, MuseumPlugin)).run()
}
