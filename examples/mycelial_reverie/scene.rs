//! Scene construction for the Mycelial Reverie example.

use std::f32::consts::{PI, TAU};

use avian3d::prelude::*;
use bevy::color::palettes::tailwind;
use bevy::math::Vec4;
use bevy::mesh::{Indices, VertexAttributeValues};
use bevy::prelude::*;
use diorama::player::Player;
use examples_common::noise::Perlin;

use crate::animation::{HoverMotion, OrbitMotion, PulseLight, SpinMotion};
use crate::materials::{MushroomGlowData, MushroomGlowMaterial, SporePoolData, SporePoolMaterial};

/// Twilight clear color - a dusky violet.
const CLEAR_COLOR: Color = Color::srgb(0.035, 0.028, 0.075);

/// Terrain configuration
const TERRAIN_SIZE: f32 = 220.0;
const TERRAIN_SUBDIVISIONS: u32 = 110;
const TERRAIN_SEED: u32 = 73;

/// Location where the pool sits, around which most structures orbit.
const GLADE_CENTER: Vec3 = Vec3::new(0.0, 0.0, 0.0);

/// Height of the pool water surface, below the terrain rim.
const POOL_WATER_Y: f32 = -0.8;
const POOL_RADIUS: f32 = 10.0;

/// Spawn the player at the edge of the glade, looking in.
const PLAYER_SPAWN: Vec3 = Vec3::new(0.0, 6.0, 28.0);
const PLAYER_LOOK_AT: Vec3 = Vec3::new(0.0, 2.0, 0.0);

struct Palette {
    ground: Handle<StandardMaterial>,
    moss: Handle<StandardMaterial>,
    stem: Handle<StandardMaterial>,
    stem_dark: Handle<StandardMaterial>,
    bark: Handle<StandardMaterial>,
    rune_stone: Handle<StandardMaterial>,
    rune_etched: Handle<StandardMaterial>,
    firefly: Handle<StandardMaterial>,
    spore: Handle<StandardMaterial>,
    starlight: Handle<StandardMaterial>,
    moon: Handle<StandardMaterial>,
    mist: Handle<StandardMaterial>,
}

struct Glows {
    cap_teal: Handle<MushroomGlowMaterial>,
    cap_amber: Handle<MushroomGlowMaterial>,
    cap_violet: Handle<MushroomGlowMaterial>,
    cap_rose: Handle<MushroomGlowMaterial>,
    pool: Handle<SporePoolMaterial>,
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard: ResMut<Assets<StandardMaterial>>,
    mut glow_materials: ResMut<Assets<MushroomGlowMaterial>>,
    mut pool_materials: ResMut<Assets<SporePoolMaterial>>,
) {
    commands.insert_resource(ClearColor(CLEAR_COLOR));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.18, 0.22, 0.36),
        brightness: 110.0,
        affects_lightmapped_meshes: true,
    });

    let noise = Perlin::new(TERRAIN_SEED);
    let palette = build_palette(&mut standard);
    let glows = build_glows(&mut glow_materials, &mut pool_materials);

    spawn_lighting(&mut commands);
    spawn_terrain(&mut commands, &mut meshes, &palette, &noise);
    spawn_pool(&mut commands, &mut meshes, &palette, &glows);
    spawn_giant_mushrooms(&mut commands, &mut meshes, &palette, &glows, &noise);
    spawn_small_mushrooms(&mut commands, &mut meshes, &palette, &glows, &noise);
    spawn_runestones(&mut commands, &mut meshes, &palette);
    spawn_gnarled_trees(&mut commands, &mut meshes, &palette, &noise);
    spawn_spore_motes(&mut commands, &mut meshes, &palette);
    spawn_fireflies(&mut commands, &mut meshes, &palette);
    spawn_mist_veils(&mut commands, &mut meshes, &palette);
    spawn_moon_and_stars(&mut commands, &mut meshes, &palette);
}

pub fn spawn_player(mut player: Single<&mut Transform, With<Player>>) {
    let spawn = Transform::from_translation(PLAYER_SPAWN).looking_at(PLAYER_LOOK_AT, Vec3::Y);
    player.translation = spawn.translation;
    player.rotation = spawn.rotation;
}

fn build_palette(materials: &mut Assets<StandardMaterial>) -> Palette {
    Palette {
        ground: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.09, 0.07),
            metallic: 0.0,
            perceptual_roughness: 0.96,
            ..default()
        }),
        moss: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.24, 0.16),
            emissive: Color::srgb(0.01, 0.04, 0.02).into(),
            metallic: 0.0,
            perceptual_roughness: 0.9,
            ..default()
        }),
        stem: materials.add(StandardMaterial {
            base_color: Color::srgb(0.86, 0.82, 0.74),
            metallic: 0.0,
            perceptual_roughness: 0.68,
            ..default()
        }),
        stem_dark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.46, 0.43, 0.36),
            metallic: 0.0,
            perceptual_roughness: 0.8,
            ..default()
        }),
        bark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.07, 0.05),
            metallic: 0.02,
            perceptual_roughness: 0.95,
            ..default()
        }),
        rune_stone: materials.add(StandardMaterial {
            base_color: Color::srgb(0.22, 0.22, 0.26),
            metallic: 0.1,
            perceptual_roughness: 0.82,
            ..default()
        }),
        rune_etched: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.42, 0.52),
            emissive: Color::srgb(0.22, 0.86, 1.1).into(),
            unlit: true,
            ..default()
        }),
        firefly: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.88, 0.54),
            emissive: Color::srgb(1.4, 1.0, 0.48).into(),
            unlit: true,
            ..default()
        }),
        spore: materials.add(StandardMaterial {
            base_color: Color::srgb(0.72, 1.0, 0.82),
            emissive: Color::srgb(0.35, 0.9, 0.6).into(),
            unlit: true,
            ..default()
        }),
        starlight: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: Color::srgb(0.86, 0.92, 1.0).into(),
            unlit: true,
            ..default()
        }),
        moon: materials.add(StandardMaterial {
            base_color: Color::srgb(0.92, 0.94, 1.0),
            emissive: Color::srgb(0.9, 0.95, 1.1).into(),
            unlit: true,
            ..default()
        }),
        mist: materials.add(StandardMaterial {
            base_color: Color::srgba(0.36, 0.42, 0.58, 0.10),
            emissive: Color::srgb(0.04, 0.06, 0.09).into(),
            alpha_mode: AlphaMode::Blend,
            metallic: 0.0,
            perceptual_roughness: 1.0,
            ..default()
        }),
    }
}

fn build_glows(
    glow_materials: &mut Assets<MushroomGlowMaterial>,
    pool_materials: &mut Assets<SporePoolMaterial>,
) -> Glows {
    Glows {
        cap_teal: glow_materials.add(MushroomGlowMaterial {
            data: MushroomGlowData {
                base_color: Vec4::new(0.04, 0.14, 0.22, 1.0),
                glow_color: Vec4::new(0.25, 1.1, 1.25, 1.0),
                pulse_speed: 0.9,
                vein_scale: 4.2,
                fresnel_power: 2.4,
                phase_offset: 0.0,
            },
        }),
        cap_amber: glow_materials.add(MushroomGlowMaterial {
            data: MushroomGlowData {
                base_color: Vec4::new(0.18, 0.08, 0.03, 1.0),
                glow_color: Vec4::new(1.3, 0.72, 0.18, 1.0),
                pulse_speed: 0.65,
                vein_scale: 3.4,
                fresnel_power: 2.0,
                phase_offset: 1.7,
            },
        }),
        cap_violet: glow_materials.add(MushroomGlowMaterial {
            data: MushroomGlowData {
                base_color: Vec4::new(0.08, 0.03, 0.16, 1.0),
                glow_color: Vec4::new(0.9, 0.38, 1.25, 1.0),
                pulse_speed: 1.05,
                vein_scale: 3.8,
                fresnel_power: 2.8,
                phase_offset: 2.6,
            },
        }),
        cap_rose: glow_materials.add(MushroomGlowMaterial {
            data: MushroomGlowData {
                base_color: Vec4::new(0.16, 0.04, 0.08, 1.0),
                glow_color: Vec4::new(1.35, 0.38, 0.66, 1.0),
                pulse_speed: 0.78,
                vein_scale: 5.0,
                fresnel_power: 2.2,
                phase_offset: 4.1,
            },
        }),
        pool: pool_materials.add(SporePoolMaterial {
            data: SporePoolData {
                shallow_color: Vec4::new(0.12, 0.42, 0.44, 0.55),
                deep_color: Vec4::new(0.02, 0.12, 0.22, 0.88),
                mote_color: Vec4::new(0.4, 1.0, 0.82, 1.0),
                ripple_scale: 22.0,
                flow_speed: 0.45,
                glow_strength: 1.6,
                _padding: 0.0,
            },
        }),
    }
}

fn spawn_lighting(commands: &mut Commands) {
    // Cold moonlight slanting in from above.
    commands.spawn((
        Name::new("Moonlight"),
        DirectionalLight {
            illuminance: 2_400.0,
            color: Color::srgb(0.68, 0.78, 1.0),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-42.0, 80.0, 28.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // A soft dome fill light tinged with the pool's color.
    commands.spawn((
        Name::new("Glade Fill"),
        PointLight {
            intensity: 3_200.0,
            range: 90.0,
            radius: 6.0,
            color: Color::srgb(0.22, 0.68, 0.82),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 30.0, 0.0),
    ));

    // Central pool glow so the caustics get picked up by the geometry.
    commands.spawn((
        Name::new("Pool Glow"),
        PointLight {
            intensity: 4_800.0,
            range: 30.0,
            radius: 1.2,
            color: Color::srgb(0.42, 1.0, 0.78),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, POOL_WATER_Y + 0.4, 0.0),
        PulseLight {
            base_intensity: 3_600.0,
            amplitude: 2_400.0,
            speed: 0.75,
            phase: 0.0,
        },
    ));
}

fn spawn_terrain(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &Palette,
    noise: &Perlin,
) {
    let mut terrain = Plane3d::default()
        .mesh()
        .size(TERRAIN_SIZE, TERRAIN_SIZE)
        .subdivisions(TERRAIN_SUBDIVISIONS)
        .build();

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for position in positions.iter_mut() {
            position[1] = terrain_height(noise, position[0], position[2]);
        }
    }

    terrain.compute_normals();

    let vertex_positions: Vec<Vec3> = terrain
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .and_then(|attribute| match attribute {
            VertexAttributeValues::Float32x3(positions) => Some(
                positions
                    .iter()
                    .map(|position| Vec3::from_array(*position))
                    .collect(),
            ),
            _ => None,
        })
        .unwrap_or_default();

    let indices: Vec<[u32; 3]> = terrain
        .indices()
        .map(|indices| match indices {
            Indices::U32(indices) => indices
                .chunks(3)
                .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                .collect(),
            Indices::U16(indices) => indices
                .chunks(3)
                .map(|chunk| {
                    [
                        u32::from(chunk[0]),
                        u32::from(chunk[1]),
                        u32::from(chunk[2]),
                    ]
                })
                .collect(),
        })
        .unwrap_or_default();

    commands.spawn((
        Name::new("Grove Terrain"),
        Mesh3d(meshes.add(terrain)),
        MeshMaterial3d(palette.ground.clone()),
        RigidBody::Static,
        Collider::trimesh(vertex_positions, indices),
    ));

    // A few scattered mossy clumps hint at overgrowth.
    for index in 0..36 {
        let angle = index as f32 * TAU / 36.0 + (index as f32 * 0.37).sin() * 0.5;
        let radius = 24.0 + (index as f32 * 1.1).sin().abs() * 46.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let y = terrain_height(noise, x, z);
        let size = 1.6 + (index % 5) as f32 * 0.3;

        commands.spawn((
            Name::new(format!("Moss Clump {}", index + 1)),
            Mesh3d(meshes.add(Sphere::new(size * 0.5))),
            MeshMaterial3d(palette.moss.clone()),
            Transform::from_xyz(x, y + size * 0.15, z).with_scale(Vec3::new(1.0, 0.35, 1.0)),
        ));
    }
}

fn spawn_pool(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &Palette,
    glows: &Glows,
) {
    // Darker basin floor sitting just under the water surface.
    commands.spawn((
        Name::new("Pool Basin"),
        Mesh3d(meshes.add(Cylinder::new(POOL_RADIUS + 0.4, 0.35))),
        MeshMaterial3d(palette.bark.clone()),
        Transform::from_xyz(0.0, POOL_WATER_Y - 0.3, 0.0),
    ));

    // Thin shader-driven water disc.
    commands.spawn((
        Name::new("Spore Pool"),
        Mesh3d(meshes.add(Cylinder::new(POOL_RADIUS, 0.06))),
        MeshMaterial3d(glows.pool.clone()),
        Transform::from_xyz(0.0, POOL_WATER_Y, 0.0),
    ));

    // Scalloped stone ring around the pool's edge.
    commands.spawn((
        Name::new("Pool Ring"),
        Mesh3d(meshes.add(Torus::new(POOL_RADIUS - 0.15, POOL_RADIUS + 0.55))),
        MeshMaterial3d(palette.rune_stone.clone()),
        Transform::from_xyz(0.0, POOL_WATER_Y + 0.05, 0.0),
    ));
}

fn spawn_giant_mushrooms(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &Palette,
    glows: &Glows,
    noise: &Perlin,
) {
    let cap_materials = [
        &glows.cap_teal,
        &glows.cap_amber,
        &glows.cap_violet,
        &glows.cap_rose,
    ];
    let light_colors = [
        Color::srgb(0.35, 0.95, 1.05),
        Color::srgb(1.1, 0.7, 0.25),
        Color::srgb(0.85, 0.4, 1.15),
        Color::srgb(1.1, 0.45, 0.7),
    ];
    const COUNT: usize = 8;

    for index in 0..COUNT {
        let angle = index as f32 * TAU / COUNT as f32 + PI / 16.0;
        let radius = 16.0 + (index as f32 * 0.73).sin() * 3.5;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let base_y = terrain_height(noise, x, z);
        let stem_height = 4.8 + (index as f32 * 0.9).sin().abs() * 2.5;
        let stem_radius = 0.55 + (index as f32 * 0.5).cos().abs() * 0.2;
        let cap_radius = stem_radius * 3.2 + 0.6;
        let tilt = 0.04 * (index as f32 * 1.3).sin();

        let cap_material = cap_materials[index % cap_materials.len()].clone();
        let light_color = light_colors[index % light_colors.len()];
        let phase = index as f32 * 0.9;

        // Stem — a slightly tapered cylinder.
        commands.spawn((
            Name::new(format!("Giant Mushroom Stem {}", index + 1)),
            Mesh3d(meshes.add(Cylinder::new(stem_radius, stem_height))),
            MeshMaterial3d(palette.stem.clone()),
            Transform::from_xyz(x, base_y + stem_height * 0.5, z).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                tilt,
                angle,
                -tilt * 0.5,
            )),
            RigidBody::Static,
            Collider::cylinder(stem_radius, stem_height),
        ));

        // Cap — a hemisphere-like scaled sphere.
        let cap_y = base_y + stem_height + cap_radius * 0.3;
        commands.spawn((
            Name::new(format!("Giant Mushroom Cap {}", index + 1)),
            Mesh3d(meshes.add(Sphere::new(cap_radius))),
            MeshMaterial3d(cap_material),
            Transform::from_xyz(x, cap_y, z).with_scale(Vec3::new(1.0, 0.55, 1.0)),
        ));

        // Under-cap skirt for a gentle dark silhouette beneath the glow.
        commands.spawn((
            Name::new(format!("Giant Mushroom Skirt {}", index + 1)),
            Mesh3d(meshes.add(Torus::new(cap_radius * 0.52, cap_radius * 0.95))),
            MeshMaterial3d(palette.stem_dark.clone()),
            Transform::from_xyz(x, cap_y - cap_radius * 0.18, z),
        ));

        // A pulsing light tucked inside the cap.
        commands.spawn((
            Name::new(format!("Giant Mushroom Light {}", index + 1)),
            PointLight {
                color: light_color,
                intensity: 2_600.0,
                range: 20.0,
                radius: 0.8,
                shadows_enabled: index % 2 == 0,
                ..default()
            },
            Transform::from_xyz(x, cap_y - cap_radius * 0.25, z),
            PulseLight {
                base_intensity: 1_900.0,
                amplitude: 1_600.0,
                speed: 0.6 + (index % 3) as f32 * 0.18,
                phase,
            },
        ));

        // Small gill rings around the stem base for detail.
        commands.spawn((
            Name::new(format!("Stem Collar {}", index + 1)),
            Mesh3d(meshes.add(Torus::new(stem_radius * 1.1, stem_radius * 1.5))),
            MeshMaterial3d(palette.stem_dark.clone()),
            Transform::from_xyz(x, base_y + stem_height * 0.75, z),
        ));
    }
}

fn spawn_small_mushrooms(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &Palette,
    glows: &Glows,
    noise: &Perlin,
) {
    let caps = [
        &glows.cap_teal,
        &glows.cap_amber,
        &glows.cap_violet,
        &glows.cap_rose,
    ];
    const COUNT: usize = 48;

    for index in 0..COUNT {
        // Distribute non-uniformly: mostly within the grove, a few scattered further.
        let t = (index as f32 * 0.61803).fract();
        let angle = t * TAU + (index as f32 * 0.27).sin() * 0.6;
        let radius = 14.0 + t.powf(0.6) * 34.0 + (index as f32 * 0.47).sin() * 2.8;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let y = terrain_height(noise, x, z);

        // Skip anything that ended up inside the pool.
        if (x * x + z * z).sqrt() < POOL_RADIUS + 0.5 {
            continue;
        }

        let stem_height = 0.6 + (index % 5) as f32 * 0.3;
        let stem_radius = 0.12 + (index % 3) as f32 * 0.05;
        let cap_radius = stem_radius * 3.0 + 0.18;

        commands.spawn((
            Name::new(format!("Small Mushroom Stem {}", index + 1)),
            Mesh3d(meshes.add(Cylinder::new(stem_radius, stem_height))),
            MeshMaterial3d(palette.stem.clone()),
            Transform::from_xyz(x, y + stem_height * 0.5, z),
        ));

        commands.spawn((
            Name::new(format!("Small Mushroom Cap {}", index + 1)),
            Mesh3d(meshes.add(Sphere::new(cap_radius))),
            MeshMaterial3d(caps[index % caps.len()].clone()),
            Transform::from_xyz(x, y + stem_height + cap_radius * 0.3, z)
                .with_scale(Vec3::new(1.0, 0.55, 1.0)),
        ));
    }
}

fn spawn_runestones(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    const COUNT: usize = 6;
    let ring_radius = POOL_RADIUS + 2.8;

    for index in 0..COUNT {
        let angle = index as f32 * TAU / COUNT as f32 + PI / COUNT as f32;
        let x = angle.cos() * ring_radius;
        let z = angle.sin() * ring_radius;

        // Upright slab - thin, tall cuboid with a slight tilt.
        let height = 1.9 + (index % 3) as f32 * 0.25;
        let tilt = 0.09 * (index as f32 * 1.7).sin();

        commands.spawn((
            Name::new(format!("Runestone {}", index + 1)),
            Mesh3d(meshes.add(Cuboid::new(1.1, height, 0.35))),
            MeshMaterial3d(palette.rune_stone.clone()),
            Transform::from_xyz(x, POOL_WATER_Y + height * 0.5 + 0.2, z).with_rotation(
                Quat::from_euler(EulerRot::YXZ, -angle + PI * 0.5, tilt, tilt * 0.6),
            ),
            RigidBody::Static,
            Collider::cuboid(1.1, height, 0.35),
        ));

        // Etched glyph - a thin emissive disc embedded in the stone face.
        commands.spawn((
            Name::new(format!("Rune Glyph {}", index + 1)),
            Mesh3d(meshes.add(Torus::new(0.22, 0.36))),
            MeshMaterial3d(palette.rune_etched.clone()),
            Transform::from_xyz(x, POOL_WATER_Y + height * 0.5 + 0.3, z).with_rotation(
                Quat::from_euler(EulerRot::YXZ, -angle + PI * 0.5, PI / 2.0 + tilt, 0.0),
            ),
            SpinMotion {
                axis: Vec3::new(0.0, 0.0, 1.0),
                speed: 0.25,
            },
        ));
    }
}

fn spawn_gnarled_trees(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &Palette,
    noise: &Perlin,
) {
    const COUNT: usize = 14;

    for index in 0..COUNT {
        let angle = index as f32 * TAU / COUNT as f32 + (index as f32 * 0.21).sin() * 0.4;
        let radius = 68.0 + (index as f32 * 1.7).sin().abs() * 18.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let base_y = terrain_height(noise, x, z);

        let trunk_height = 11.0 + (index as f32 * 0.6).sin().abs() * 6.0;
        let trunk_radius = 0.6 + (index as f32 * 0.43).cos().abs() * 0.2;

        commands.spawn((
            Name::new(format!("Gnarled Trunk {}", index + 1)),
            Mesh3d(meshes.add(Cylinder::new(trunk_radius, trunk_height))),
            MeshMaterial3d(palette.bark.clone()),
            Transform::from_xyz(x, base_y + trunk_height * 0.5, z).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.06 * (index as f32 * 1.1).sin(),
                angle,
                0.05 * (index as f32 * 1.4).cos(),
            )),
        ));

        // A few crooked branches per tree.
        for branch in 0..4 {
            let b_angle = angle + (branch as f32 * 0.9 - 1.5);
            let b_len = 3.2 + (branch as f32 * 0.4).cos().abs() * 1.8;
            let b_radius = 0.2;
            let y_offset = trunk_height * 0.55 + branch as f32 * 0.8;
            let branch_tilt = 0.35 + (branch as f32 * 0.3).sin() * 0.25;

            let dir = Vec3::new(b_angle.cos(), 0.0, b_angle.sin()).normalize_or_zero();
            let branch_center = Vec3::new(x, base_y + y_offset, z) + dir * (b_len * 0.5);

            commands.spawn((
                Name::new(format!("Branch {}-{}", index + 1, branch + 1)),
                Mesh3d(meshes.add(Cylinder::new(b_radius, b_len))),
                MeshMaterial3d(palette.bark.clone()),
                Transform::from_translation(branch_center).with_rotation(Quat::from_euler(
                    EulerRot::ZYX,
                    PI / 2.0 - branch_tilt,
                    -b_angle,
                    0.0,
                )),
            ));
        }
    }
}

fn spawn_spore_motes(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let mote_mesh = meshes.add(Sphere::new(0.09));
    const COUNT: usize = 72;

    for index in 0..COUNT {
        let t = index as f32 / COUNT as f32;
        let radius = 3.0 + t * 24.0 + (index as f32 * 0.53).sin() * 2.0;
        let height = POOL_WATER_Y + 0.5 + (index as f32 * 0.31).sin() * 5.5;
        let phase = index as f32 * 0.47;

        commands.spawn((
            Name::new(format!("Spore Mote {}", index + 1)),
            Mesh3d(mote_mesh.clone()),
            MeshMaterial3d(palette.spore.clone()),
            Transform::from_xyz(0.0, height, 0.0),
            OrbitMotion {
                center: GLADE_CENTER,
                radius,
                height,
                angular_speed: 0.05 + (index % 9) as f32 * 0.012,
                vertical_amplitude: 0.5 + (index % 5) as f32 * 0.16,
                vertical_speed: 0.4 + (index % 7) as f32 * 0.09,
                phase,
                spin_speed: 0.0,
            },
        ));
    }
}

fn spawn_fireflies(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let body_mesh = meshes.add(Sphere::new(0.16));
    const COUNT: usize = 16;

    for index in 0..COUNT {
        let angle = index as f32 * TAU / COUNT as f32 + (index as f32 * 0.23).sin();
        let radius = 8.0 + (index as f32 * 0.57).sin().abs() * 16.0;
        let height = POOL_WATER_Y + 2.8 + (index as f32 * 0.39).sin() * 3.0;
        let anchor = Vec3::new(angle.cos() * radius, height, angle.sin() * radius);
        let phase = index as f32 * 0.81;

        let color = if index % 3 == 0 {
            Color::srgb(0.55, 1.0, 0.82)
        } else {
            Color::srgb(1.0, 0.84, 0.42)
        };

        commands.spawn((
            Name::new(format!("Firefly {}", index + 1)),
            Mesh3d(body_mesh.clone()),
            MeshMaterial3d(palette.firefly.clone()),
            Transform::from_translation(anchor),
            HoverMotion {
                anchor,
                vertical_amplitude: 0.5 + (index % 4) as f32 * 0.18,
                lateral_amplitude: 0.9 + (index % 3) as f32 * 0.2,
                speed: 0.5 + (index % 5) as f32 * 0.12,
                phase,
            },
        ));

        commands.spawn((
            Name::new(format!("Firefly Light {}", index + 1)),
            PointLight {
                color,
                intensity: 900.0,
                range: 10.0,
                radius: 0.3,
                shadows_enabled: false,
                ..default()
            },
            Transform::from_translation(anchor),
            PulseLight {
                base_intensity: 550.0,
                amplitude: 520.0,
                speed: 1.4 + (index % 4) as f32 * 0.3,
                phase,
            },
            HoverMotion {
                anchor,
                vertical_amplitude: 0.5 + (index % 4) as f32 * 0.18,
                lateral_amplitude: 0.9 + (index % 3) as f32 * 0.2,
                speed: 0.5 + (index % 5) as f32 * 0.12,
                phase,
            },
        ));
    }
}

fn spawn_mist_veils(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let veil_mesh = meshes.add(Cuboid::new(90.0, 0.04, 90.0));

    for index in 0..3 {
        let y = 0.8 + index as f32 * 0.6;
        let rotation = Quat::from_rotation_y(index as f32 * 0.32);
        commands.spawn((
            Name::new(format!("Mist Veil {}", index + 1)),
            Mesh3d(veil_mesh.clone()),
            MeshMaterial3d(palette.mist.clone()),
            Transform::from_xyz(0.0, y, 0.0).with_rotation(rotation),
        ));
    }
}

fn spawn_moon_and_stars(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    // A lone pale moon high in the sky.
    commands.spawn((
        Name::new("Twin Moon"),
        Mesh3d(meshes.add(Sphere::new(6.0))),
        MeshMaterial3d(palette.moon.clone()),
        Transform::from_xyz(-52.0, 82.0, -110.0),
    ));

    // And a tinted companion for a little fantasy touch.
    commands.spawn((
        Name::new("Lesser Moon"),
        Mesh3d(meshes.add(Sphere::new(2.2))),
        MeshMaterial3d(
            // reuse starlight for a cooler silver look
            palette.starlight.clone(),
        ),
        Transform::from_xyz(-28.0, 68.0, -96.0),
    ));

    // Star canopy, placed on a hemisphere above using a golden-angle distribution.
    let star_mesh = meshes.add(Sphere::new(0.18));
    const STAR_COUNT: usize = 240;
    const GOLDEN_ANGLE: f32 = 2.3999631;

    for index in 0..STAR_COUNT {
        let t = index as f32 / STAR_COUNT as f32;
        // Only upper hemisphere - keep y in [0, 1].
        let y = t;
        let radial = (1.0 - y * y).sqrt();
        let angle = GOLDEN_ANGLE * index as f32;
        let direction = Vec3::new(angle.cos() * radial, y, angle.sin() * radial);
        let radius = 150.0 + (index as f32 * 0.37).sin().abs() * 30.0;
        let scale = 0.08 + (index % 7) as f32 * 0.018;

        commands.spawn((
            Name::new(format!("Star {}", index + 1)),
            Mesh3d(star_mesh.clone()),
            MeshMaterial3d(palette.starlight.clone()),
            Transform::from_translation(direction * radius).with_scale(Vec3::splat(scale)),
        ));
    }

    // A tinted rim light as if from the horizon glow.
    commands.spawn((
        Name::new("Horizon Glow"),
        PointLight {
            intensity: 1_400.0,
            range: 200.0,
            radius: 12.0,
            color: tailwind::VIOLET_400.into(),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(60.0, 6.0, -60.0),
    ));
}

/// Terrain height sampler: gentle hills with a central bowl for the pool.
fn terrain_height(noise: &Perlin, x: f32, z: f32) -> f32 {
    let radius = Vec2::new(x, z).length();

    let rolling = noise.get([f64::from(x) * 0.028, f64::from(z) * 0.028]) as f32 * 2.1
        + noise.get([f64::from(x) * 0.09, f64::from(z) * 0.09]) as f32 * 0.55;

    let basin_factor = (1.0 - (radius / 18.0).clamp(0.0, 1.0)).powf(1.8);
    let basin = -3.4 * basin_factor;

    let rim = gaussian(radius, 20.5, 3.2) * 1.4;

    rolling + basin + rim
}

fn gaussian(value: f32, mean: f32, width: f32) -> f32 {
    let delta = value - mean;
    (-delta * delta / (2.0 * width * width)).exp()
}
