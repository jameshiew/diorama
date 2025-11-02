//! Level geometry and layout system.
//!
//! This module provides a declarative, data-driven approach to level design.
//! Level geometry is organized into logical sections (tutorial, challenge, finale)
//! with type-safe platform categorization and material caching for performance.
//!
//! # Architecture
//!
//! The level system follows a resource-based pattern where level data is initialized
//! once and made available as a [`CurrentLevel`] resource:
//!
//! 1. **Level Initialization**: [`initialize_level`] creates the [`CurrentLevel`] resource
//! 2. **Geometry Spawning**: [`spawn_level_geometry`] reads from [`CurrentLevel`] to spawn platforms
//! 3. **Collectibles Spawning**: `collectibles::spawn_collectibles` reads collectible positions
//!
//! # Adding New Levels
//!
//! To create a new level:
//!
//! 1. Define sections in the `sections` module with [`SectionData`] containing:
//!    - `platforms`: The physical geometry players can stand on
//!    - `collectible_positions`: Where gems/items should spawn
//!
//! 2. Combine sections in [`LevelDefinition::default_level`] (or create a new level variant)
//!
//! 3. The level will automatically spawn all geometry and collectibles in the correct order
//!
//! # Example: Adding a New Section
//!
//! ```ignore
//! pub fn bonus_section() -> SectionData {
//!     SectionData {
//!         platforms: vec![
//!             Platform::challenge("Secret Platform", Vec3::new(20.0, 15.0, 0.0), Vec3::new(3.0, 0.5, 3.0)),
//!         ],
//!         collectible_positions: vec![
//!             Vec3::new(20.0, 16.0, 0.0),  // Gem on secret platform
//!         ],
//!     }
//! }
//! ```

use avian3d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

/// Platform types with associated visual properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PlatformType {
    /// Large, safe starting/ending areas
    Ground,
    /// Standard platforming challenges
    Standard,
    /// Difficult jumps and narrow surfaces
    Challenge,
    /// Very small stepping stones
    SteppingStone,
}

impl PlatformType {
    /// Returns the material configuration for this platform type.
    const fn material_config(&self) -> MaterialConfig {
        match self {
            PlatformType::Ground => MaterialConfig {
                color: tailwind::GREEN_600,
                metallic: 0.0,
                roughness: 0.8,
            },
            PlatformType::Standard => MaterialConfig {
                color: tailwind::AMBER_700,
                metallic: 0.2,
                roughness: 0.6,
            },
            PlatformType::Challenge => MaterialConfig {
                color: tailwind::RED_600,
                metallic: 0.3,
                roughness: 0.4,
            },
            PlatformType::SteppingStone => MaterialConfig {
                color: tailwind::AMBER_700,
                metallic: 0.2,
                roughness: 0.6,
            },
        }
    }
}

/// Material configuration that can be converted into a [`StandardMaterial`].
#[derive(Debug, Clone, Copy)]
struct MaterialConfig {
    color: bevy::color::Srgba,
    metallic: f32,
    roughness: f32,
}

impl MaterialConfig {
    /// Converts this configuration into a Bevy [`StandardMaterial`] asset.
    fn into_material(
        self,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Handle<StandardMaterial> {
        materials.add(StandardMaterial {
            base_color: self.color.into(),
            metallic: self.metallic,
            perceptual_roughness: self.roughness,
            ..default()
        })
    }
}

/// Pre-created material handles to avoid duplicate asset creation.
///
/// Materials are cached on initialization and reused for all matching object types,
/// improving performance and reducing memory usage.
struct MaterialCache {
    ground: Handle<StandardMaterial>,
    standard: Handle<StandardMaterial>,
    challenge: Handle<StandardMaterial>,
    stepping_stone: Handle<StandardMaterial>,
    wall: Handle<StandardMaterial>,
    pillar: Handle<StandardMaterial>,
}

impl MaterialCache {
    /// Creates a new material cache with all materials pre-initialized.
    fn new(materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        Self {
            ground: PlatformType::Ground
                .material_config()
                .into_material(materials),
            standard: PlatformType::Standard
                .material_config()
                .into_material(materials),
            challenge: PlatformType::Challenge
                .material_config()
                .into_material(materials),
            stepping_stone: PlatformType::SteppingStone
                .material_config()
                .into_material(materials),
            wall: materials.add(StandardMaterial {
                base_color: tailwind::SLATE_600.into(),
                ..default()
            }),
            pillar: materials.add(StandardMaterial {
                base_color: tailwind::STONE_500.into(),
                metallic: 0.4,
                perceptual_roughness: 0.3,
                ..default()
            }),
        }
    }

    /// Returns the cached material handle for the given platform type.
    fn get(&self, platform_type: PlatformType) -> Handle<StandardMaterial> {
        match platform_type {
            PlatformType::Ground => self.ground.clone(),
            PlatformType::Standard => self.standard.clone(),
            PlatformType::Challenge => self.challenge.clone(),
            PlatformType::SteppingStone => self.stepping_stone.clone(),
        }
    }
}

/// Trait for objects that can spawn themselves into the world as entities.
trait Spawnable {
    /// Spawns this object as a Bevy entity with appropriate components.
    fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<StandardMaterial>,
    );
}

/// A platform with position, size, and visual/gameplay type.
#[derive(Debug, Clone)]
pub(crate) struct Platform {
    name: &'static str,
    position: Vec3,
    size: Vec3,
    platform_type: PlatformType,
}

impl Platform {
    /// Creates a new platform with the specified properties.
    const fn new(
        name: &'static str,
        position: Vec3,
        size: Vec3,
        platform_type: PlatformType,
    ) -> Self {
        Self {
            name,
            position,
            size,
            platform_type,
        }
    }

    /// Creates a ground-type platform (large, safe areas).
    const fn ground(name: &'static str, position: Vec3, size: Vec3) -> Self {
        Self::new(name, position, size, PlatformType::Ground)
    }

    /// Creates a standard platform (regular platforming challenges).
    const fn standard(name: &'static str, position: Vec3, size: Vec3) -> Self {
        Self::new(name, position, size, PlatformType::Standard)
    }

    /// Creates a challenge platform (difficult jumps and narrow surfaces).
    const fn challenge(name: &'static str, position: Vec3, size: Vec3) -> Self {
        Self::new(name, position, size, PlatformType::Challenge)
    }

    /// Creates a stepping stone platform with a fixed small size for precision jumps.
    const fn stepping_stone(name: &'static str, position: Vec3) -> Self {
        Self::new(
            name,
            position,
            Vec3::new(1.5, 0.3, 1.5),
            PlatformType::SteppingStone,
        )
    }
}

impl Spawnable for Platform {
    fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<StandardMaterial>,
    ) {
        commands.spawn((
            Name::new(self.name.to_string()),
            RigidBody::Static,
            Collider::cuboid(self.size.x, self.size.y, self.size.z),
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(
                self.size.x,
                self.size.y,
                self.size.z,
            )))),
            MeshMaterial3d(material),
            Transform::from_translation(self.position),
        ));
    }
}

/// A boundary wall to prevent the player from falling off the world.
#[derive(Debug, Clone)]
struct Wall {
    position: Vec3,
    size: Vec3,
}

impl Wall {
    /// Creates a new wall with the given position and size.
    const fn new(position: Vec3, size: Vec3) -> Self {
        Self { position, size }
    }

    /// Creates a north-facing wall extending along the X axis.
    const fn north(x: f32, y: f32, z: f32, width: f32, height: f32, thickness: f32) -> Self {
        Self::new(Vec3::new(x, y, z), Vec3::new(width, height, thickness))
    }

    /// Creates an east-facing wall extending along the Z axis.
    const fn east(x: f32, y: f32, z: f32, height: f32, depth: f32, thickness: f32) -> Self {
        Self::new(Vec3::new(x, y, z), Vec3::new(thickness, height, depth))
    }

    /// Creates a west-facing wall extending along the Z axis.
    const fn west(x: f32, y: f32, z: f32, height: f32, depth: f32, thickness: f32) -> Self {
        Self::new(Vec3::new(x, y, z), Vec3::new(thickness, height, depth))
    }
}

impl Spawnable for Wall {
    fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<StandardMaterial>,
    ) {
        commands.spawn((
            Name::new("Boundary Wall"),
            RigidBody::Static,
            Collider::cuboid(self.size.x, self.size.y, self.size.z),
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(
                self.size.x,
                self.size.y,
                self.size.z,
            )))),
            MeshMaterial3d(material),
            Transform::from_translation(self.position),
        ));
    }
}

/// A decorative cylindrical pillar for visual interest.
#[derive(Debug, Clone)]
struct Pillar {
    position: Vec3,
    radius: f32,
    height: f32,
}

impl Pillar {
    /// Creates a new pillar with the given position, radius, and height.
    const fn new(position: Vec3, radius: f32, height: f32) -> Self {
        Self {
            position,
            radius,
            height,
        }
    }
}

impl Spawnable for Pillar {
    fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<StandardMaterial>,
    ) {
        commands.spawn((
            Name::new("Decorative Pillar"),
            Mesh3d(meshes.add(Mesh::from(Cylinder::new(self.radius, self.height)))),
            MeshMaterial3d(material),
            Transform::from_translation(self.position),
        ));
    }
}

/// Section data containing both geometry and collectible placements.
#[derive(Debug, Clone)]
pub struct SectionData {
    pub platforms: Vec<Platform>,
    pub collectible_positions: Vec<Vec3>,
}

/// Level sections organized by gameplay purpose.
///
/// Each section contains platforms and obstacles for a specific part of the level,
/// making the overall level structure easy to understand and modify.
mod sections {
    use super::*;

    /// Returns platforms and collectibles for the tutorial section where players learn basic movement.
    pub fn tutorial_section() -> SectionData {
        SectionData {
            platforms: vec![
                Platform::ground(
                    "Starting Platform",
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(12.0, 1.0, 12.0),
                ),
                Platform::standard(
                    "Platform 1",
                    Vec3::new(8.0, 2.0, -8.0),
                    Vec3::new(4.0, 0.5, 4.0),
                ),
                Platform::standard(
                    "Platform 2",
                    Vec3::new(4.0, 4.0, -16.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                Platform::standard(
                    "Platform 3",
                    Vec3::new(-2.0, 6.0, -20.0),
                    Vec3::new(4.0, 0.5, 4.0),
                ),
            ],
            // Collectibles placed on each tutorial platform
            collectible_positions: vec![
                Vec3::new(8.0, 3.5, -8.0),   // On platform 1
                Vec3::new(4.0, 5.5, -16.0),  // On platform 2
                Vec3::new(-2.0, 7.5, -20.0), // On platform 3
            ],
        }
    }

    /// Spiral ascent section - platforms arranged in a spiral going upward.
    pub fn spiral_ascent_section() -> SectionData {
        SectionData {
            platforms: vec![
                Platform::standard(
                    "Spiral 1",
                    Vec3::new(-6.0, 8.0, -24.0),
                    Vec3::new(3.5, 0.5, 3.5),
                ),
                Platform::standard(
                    "Spiral 2",
                    Vec3::new(-10.0, 10.0, -28.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                Platform::standard(
                    "Spiral 3",
                    Vec3::new(-14.0, 12.0, -24.0),
                    Vec3::new(3.5, 0.5, 3.5),
                ),
                Platform::standard(
                    "Spiral 4",
                    Vec3::new(-10.0, 14.0, -20.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                Platform::standard(
                    "Spiral 5",
                    Vec3::new(-6.0, 16.0, -24.0),
                    Vec3::new(3.5, 0.5, 3.5),
                ),
            ],
            collectible_positions: vec![
                Vec3::new(-6.0, 9.5, -24.0),
                Vec3::new(-14.0, 13.5, -24.0),
                Vec3::new(-6.0, 17.5, -24.0),
            ],
        }
    }

    /// Gap jumping section with varying distances for timing practice.
    pub fn gap_jumps_section() -> SectionData {
        SectionData {
            platforms: vec![
                Platform::standard(
                    "Gap Platform 1",
                    Vec3::new(-10.0, 18.0, -28.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                Platform::standard(
                    "Gap Platform 2",
                    Vec3::new(-10.0, 18.5, -35.0),
                    Vec3::new(2.5, 0.5, 2.5),
                ),
                Platform::standard(
                    "Gap Platform 3",
                    Vec3::new(-14.0, 19.0, -40.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                Platform::standard(
                    "Gap Platform 4",
                    Vec3::new(-18.0, 20.0, -35.0),
                    Vec3::new(2.5, 0.5, 2.5),
                ),
                Platform::standard(
                    "Gap Platform 5",
                    Vec3::new(-22.0, 21.0, -40.0),
                    Vec3::new(3.5, 0.5, 3.5),
                ),
            ],
            collectible_positions: vec![
                Vec3::new(-10.0, 20.0, -35.0),
                Vec3::new(-18.0, 21.5, -35.0),
                Vec3::new(-22.0, 22.5, -40.0),
            ],
        }
    }

    /// Narrow bridge section - risky linear path with rewards.
    pub fn narrow_bridge_section() -> SectionData {
        SectionData {
            platforms: vec![
                Platform::challenge(
                    "Bridge Start",
                    Vec3::new(-22.0, 22.0, -36.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                Platform::challenge(
                    "Bridge 1",
                    Vec3::new(-22.0, 22.0, -30.0),
                    Vec3::new(1.5, 0.3, 4.0),
                ),
                Platform::challenge(
                    "Bridge 2",
                    Vec3::new(-22.0, 22.0, -24.0),
                    Vec3::new(1.5, 0.3, 4.0),
                ),
                Platform::challenge(
                    "Bridge End",
                    Vec3::new(-22.0, 22.0, -18.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
            ],
            collectible_positions: vec![
                Vec3::new(-22.0, 23.5, -30.0),
                Vec3::new(-22.0, 23.5, -24.0),
            ],
        }
    }

    /// Aerial challenge - high platforms requiring well-timed jumps.
    pub fn aerial_challenge_section() -> SectionData {
        SectionData {
            platforms: vec![
                Platform::challenge(
                    "Aerial Start",
                    Vec3::new(-22.0, 23.0, -12.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                Platform::challenge(
                    "Aerial 1",
                    Vec3::new(-18.0, 26.0, -8.0),
                    Vec3::new(2.5, 0.5, 2.5),
                ),
                Platform::challenge(
                    "Aerial 2",
                    Vec3::new(-14.0, 28.0, -4.0),
                    Vec3::new(2.0, 0.5, 2.0),
                ),
                Platform::challenge(
                    "Aerial Peak",
                    Vec3::new(-10.0, 30.0, 0.0),
                    Vec3::new(4.0, 0.5, 4.0),
                ),
            ],
            collectible_positions: vec![
                Vec3::new(-18.0, 27.5, -8.0),
                Vec3::new(-14.0, 29.5, -4.0),
                Vec3::new(-10.0, 31.5, 0.0), // Reward at peak
            ],
        }
    }

    /// Descent section - controlled falling with platform catching.
    pub fn descent_section() -> SectionData {
        SectionData {
            platforms: vec![
                Platform::standard(
                    "Descent 1",
                    Vec3::new(-6.0, 28.0, 4.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                Platform::standard(
                    "Descent 2",
                    Vec3::new(-2.0, 25.0, 8.0),
                    Vec3::new(3.5, 0.5, 3.5),
                ),
                Platform::standard(
                    "Descent 3",
                    Vec3::new(2.0, 22.0, 12.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                Platform::standard(
                    "Descent 4",
                    Vec3::new(6.0, 19.0, 16.0),
                    Vec3::new(3.5, 0.5, 3.5),
                ),
                Platform::standard(
                    "Descent 5",
                    Vec3::new(10.0, 16.0, 20.0),
                    Vec3::new(4.0, 0.5, 4.0),
                ),
            ],
            collectible_positions: vec![
                Vec3::new(-6.0, 29.5, 4.0),
                Vec3::new(2.0, 23.5, 12.0),
                Vec3::new(10.0, 17.5, 20.0),
            ],
        }
    }

    /// Zigzag path section - alternating direction changes.
    pub fn zigzag_path_section() -> SectionData {
        SectionData {
            platforms: vec![
                Platform::standard(
                    "Zigzag 1",
                    Vec3::new(14.0, 15.0, 16.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                Platform::standard(
                    "Zigzag 2",
                    Vec3::new(10.0, 14.0, 12.0),
                    Vec3::new(2.5, 0.5, 2.5),
                ),
                Platform::standard(
                    "Zigzag 3",
                    Vec3::new(14.0, 13.0, 8.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                Platform::standard(
                    "Zigzag 4",
                    Vec3::new(10.0, 12.0, 4.0),
                    Vec3::new(2.5, 0.5, 2.5),
                ),
                Platform::standard(
                    "Zigzag 5",
                    Vec3::new(14.0, 11.0, 0.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
            ],
            collectible_positions: vec![
                Vec3::new(14.0, 16.5, 16.0),
                Vec3::new(14.0, 14.5, 8.0),
                Vec3::new(14.0, 12.5, 0.0),
            ],
        }
    }

    /// Bonus area - risky optional path with extra collectibles.
    pub fn bonus_area_section() -> SectionData {
        SectionData {
            platforms: vec![
                Platform::challenge(
                    "Bonus Entry",
                    Vec3::new(18.0, 15.0, 12.0),
                    Vec3::new(2.0, 0.5, 2.0),
                ),
                Platform::stepping_stone("Bonus Stone 1", Vec3::new(22.0, 17.0, 12.0)),
                Platform::stepping_stone("Bonus Stone 2", Vec3::new(24.0, 19.0, 8.0)),
                Platform::stepping_stone("Bonus Stone 3", Vec3::new(26.0, 21.0, 12.0)),
                Platform::challenge(
                    "Bonus Reward",
                    Vec3::new(22.0, 23.0, 16.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
            ],
            collectible_positions: vec![
                Vec3::new(22.0, 18.5, 12.0),
                Vec3::new(26.0, 22.5, 12.0),
                Vec3::new(22.0, 24.5, 16.0), // Big reward
            ],
        }
    }

    /// Returns platforms and collectibles for the challenge section requiring precise jumps.
    pub fn challenge_section() -> SectionData {
        SectionData {
            platforms: vec![Platform::challenge(
                "High Platform",
                Vec3::new(-8.0, 10.0, -16.0),
                Vec3::new(5.0, 0.5, 5.0),
            )],
            // High-value collectible on the challenge platform
            collectible_positions: vec![Vec3::new(-8.0, 11.5, -16.0)],
        }
    }

    /// Returns small stepping stone platforms for precision platforming.
    pub fn stepping_stones_section() -> SectionData {
        SectionData {
            platforms: vec![
                Platform::stepping_stone("Stepping Stone 1", Vec3::new(-12.0, 8.0, -8.0)),
                Platform::stepping_stone("Stepping Stone 2", Vec3::new(-14.0, 9.0, -4.0)),
                Platform::stepping_stone("Stepping Stone 3", Vec3::new(-16.0, 10.0, 0.0)),
                Platform::stepping_stone("Stepping Stone 4", Vec3::new(-14.0, 11.0, 4.0)),
            ],
            // Collectibles on each stepping stone (challenging collection)
            collectible_positions: vec![
                Vec3::new(-12.0, 9.5, -8.0),
                Vec3::new(-14.0, 10.5, -4.0),
                Vec3::new(-16.0, 11.5, 0.0),
            ],
        }
    }

    /// Returns the final platform where the level ends.
    pub fn finale_section() -> SectionData {
        SectionData {
            platforms: vec![
                // Approach platforms
                Platform::standard(
                    "Pre-finale 1",
                    Vec3::new(10.0, 10.0, -4.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                Platform::standard(
                    "Pre-finale 2",
                    Vec3::new(6.0, 10.0, -8.0),
                    Vec3::new(3.0, 0.5, 3.0),
                ),
                // Grand finale platform
                Platform::ground(
                    "Final Platform",
                    Vec3::new(0.0, 10.0, -12.0),
                    Vec3::new(12.0, 1.5, 12.0),
                ),
            ],
            collectible_positions: vec![
                Vec3::new(10.0, 11.5, -4.0),
                Vec3::new(6.0, 11.5, -8.0),
                Vec3::new(-4.0, 12.5, -12.0), // Final collectible
                Vec3::new(0.0, 12.5, -12.0),  // Victory collectible
                Vec3::new(4.0, 12.5, -12.0),  // Bonus finale collectible
            ],
        }
    }

    /// Returns boundary walls that prevent falling off the world.
    pub fn boundary_walls() -> Vec<Wall> {
        vec![
            // North walls
            Wall::north(0.0, 20.0, -50.0, 80.0, 40.0, 2.0),
            // South walls
            Wall::north(0.0, 20.0, 30.0, 80.0, 40.0, 2.0),
            // East walls
            Wall::east(40.0, 20.0, -10.0, 40.0, 80.0, 2.0),
            // West walls
            Wall::west(-40.0, 20.0, -10.0, 40.0, 80.0, 2.0),
        ]
    }

    /// Returns decorative pillars placed around the level for visual interest.
    pub fn decorative_pillars() -> Vec<Pillar> {
        vec![
            // Starting area pillars
            Pillar::new(Vec3::new(15.0, 5.0, 15.0), 1.0, 10.0),
            Pillar::new(Vec3::new(-15.0, 5.0, 15.0), 1.0, 10.0),
            Pillar::new(Vec3::new(15.0, 5.0, -15.0), 1.0, 10.0),
            Pillar::new(Vec3::new(-15.0, 5.0, -15.0), 1.0, 10.0),
            // Mid-level markers
            Pillar::new(Vec3::new(-20.0, 10.0, -30.0), 0.8, 20.0),
            Pillar::new(Vec3::new(-5.0, 15.0, -35.0), 0.8, 30.0),
            // High area markers
            Pillar::new(Vec3::new(-15.0, 15.0, -5.0), 1.2, 30.0),
            Pillar::new(Vec3::new(5.0, 10.0, 10.0), 1.0, 20.0),
            // Bonus area markers
            Pillar::new(Vec3::new(28.0, 12.0, 10.0), 0.7, 24.0),
        ]
    }
}

/// Complete level definition containing all platforms, walls, decorations, and collectibles.
#[derive(Debug, Clone)]
pub struct LevelDefinition {
    platforms: Vec<Platform>,
    walls: Vec<Wall>,
    pillars: Vec<Pillar>,
    /// Collectible positions for gems throughout the level.
    pub collectible_positions: Vec<Vec3>,
}

impl LevelDefinition {
    /// Constructs the default level by combining all sections.
    fn default_level() -> Self {
        let mut platforms = Vec::new();
        let mut collectible_positions = Vec::new();

        // Assemble the level from individual sections in progression order
        let tutorial = sections::tutorial_section();
        platforms.extend(tutorial.platforms);
        collectible_positions.extend(tutorial.collectible_positions);

        let spiral = sections::spiral_ascent_section();
        platforms.extend(spiral.platforms);
        collectible_positions.extend(spiral.collectible_positions);

        let gaps = sections::gap_jumps_section();
        platforms.extend(gaps.platforms);
        collectible_positions.extend(gaps.collectible_positions);

        let bridge = sections::narrow_bridge_section();
        platforms.extend(bridge.platforms);
        collectible_positions.extend(bridge.collectible_positions);

        let aerial = sections::aerial_challenge_section();
        platforms.extend(aerial.platforms);
        collectible_positions.extend(aerial.collectible_positions);

        let descent = sections::descent_section();
        platforms.extend(descent.platforms);
        collectible_positions.extend(descent.collectible_positions);

        let zigzag = sections::zigzag_path_section();
        platforms.extend(zigzag.platforms);
        collectible_positions.extend(zigzag.collectible_positions);

        let bonus = sections::bonus_area_section();
        platforms.extend(bonus.platforms);
        collectible_positions.extend(bonus.collectible_positions);

        let challenge = sections::challenge_section();
        platforms.extend(challenge.platforms);
        collectible_positions.extend(challenge.collectible_positions);

        let stepping_stones = sections::stepping_stones_section();
        platforms.extend(stepping_stones.platforms);
        collectible_positions.extend(stepping_stones.collectible_positions);

        let finale = sections::finale_section();
        platforms.extend(finale.platforms);
        collectible_positions.extend(finale.collectible_positions);

        Self {
            platforms,
            walls: sections::boundary_walls(),
            pillars: sections::decorative_pillars(),
            collectible_positions,
        }
    }
}

/// Resource containing the current level's complete definition.
///
/// This resource is initialized at startup and can be replaced to load different levels.
#[derive(Resource, Debug, Clone)]
pub struct CurrentLevel(pub LevelDefinition);

/// Initializes the current level resource with the default level.
pub fn initialize_level(mut commands: Commands) {
    let level = LevelDefinition::default_level();
    commands.insert_resource(CurrentLevel(level));
}

/// Spawns all level geometry including platforms, walls, and decorative elements.
///
/// This system uses cached materials and the [`Spawnable`] trait for efficient,
/// polymorphic entity creation.
pub fn spawn_level_geometry(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    current_level: Res<CurrentLevel>,
) {
    let material_cache = MaterialCache::new(&mut materials);

    // Spawn platforms with their type-appropriate materials
    for platform in &current_level.0.platforms {
        let material = material_cache.get(platform.platform_type);
        platform.spawn(&mut commands, &mut meshes, material);
    }

    // Spawn boundary walls
    for wall in &current_level.0.walls {
        wall.spawn(&mut commands, &mut meshes, material_cache.wall.clone());
    }

    // Spawn decorative elements
    for pillar in &current_level.0.pillars {
        pillar.spawn(&mut commands, &mut meshes, material_cache.pillar.clone());
    }
}
