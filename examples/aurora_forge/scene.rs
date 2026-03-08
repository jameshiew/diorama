use std::f32::consts::{PI, TAU};

use avian3d::prelude::*;
use bevy::mesh::{Indices, VertexAttributeValues};
use bevy::prelude::*;
use diorama::player::Player;
use noise::{NoiseFn, Perlin};

use crate::animation::{HoverMotion, OrbitMotion, PulseLight, RingRotor, SweepSpotlight};
use crate::materials::{
    AuroraRibbonData, AuroraRibbonMaterial, ForgePlasmaData, ForgePlasmaMaterial,
};

const FORGE_CLEAR_COLOR: Color = Color::srgb(0.012, 0.024, 0.04);
const TERRAIN_SIZE: f32 = 280.0;
const TERRAIN_SUBDIVISIONS: u32 = 140;
const TERRAIN_SEED: u32 = 27;
const TERRAIN_BASE_Y: f32 = -6.0;
const MAIN_DECK_Y: f32 = 11.0;
const MAIN_DECK_RADIUS: f32 = 28.0;
const FORGE_CENTER: Vec3 = Vec3::new(0.0, 19.0, 0.0);
const PLAYER_SPAWN: Vec3 = Vec3::new(28.0, MAIN_DECK_Y + 3.0, -24.0);

struct MaterialPalette {
    basalt: Handle<StandardMaterial>,
    basalt_dark: Handle<StandardMaterial>,
    brass: Handle<StandardMaterial>,
    copper: Handle<StandardMaterial>,
    glass: Handle<StandardMaterial>,
    ember: Handle<StandardMaterial>,
    ember_particle: Handle<StandardMaterial>,
    starlight: Handle<StandardMaterial>,
}

struct ShaderPalette {
    plasma_core: Handle<ForgePlasmaMaterial>,
    plasma_stream: Handle<ForgePlasmaMaterial>,
    aurora_primary: Handle<AuroraRibbonMaterial>,
    aurora_secondary: Handle<AuroraRibbonMaterial>,
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut plasma_materials: ResMut<Assets<ForgePlasmaMaterial>>,
    mut aurora_materials: ResMut<Assets<AuroraRibbonMaterial>>,
) {
    commands.insert_resource(ClearColor(FORGE_CLEAR_COLOR));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.18, 0.28, 0.32),
        brightness: 420.0,
        affects_lightmapped_meshes: true,
    });

    let terrain_noise = Perlin::new(TERRAIN_SEED);
    let palette = create_material_palette(&mut materials);
    let shader_palette = create_shader_palette(&mut plasma_materials, &mut aurora_materials);

    spawn_lighting(&mut commands);
    spawn_caldera(
        &mut commands,
        &mut meshes,
        &palette,
        &shader_palette,
        &terrain_noise,
    );
    spawn_main_deck(&mut commands, &mut meshes, &palette, &shader_palette);
    spawn_outer_crown(
        &mut commands,
        &mut meshes,
        &palette,
        &shader_palette,
        &terrain_noise,
    );
    spawn_central_forge(&mut commands, &mut meshes, &palette, &shader_palette);
    spawn_shard_halo(&mut commands, &mut meshes, &palette);
    spawn_aurora_sails(&mut commands, &mut meshes, &shader_palette);
    spawn_ember_swarm(&mut commands, &mut meshes, &palette);
    spawn_floating_islets(&mut commands, &mut meshes, &palette);
    spawn_distant_spires(&mut commands, &mut meshes, &palette, &terrain_noise);
    spawn_star_canopy(&mut commands, &mut meshes, &palette);
}

pub fn spawn_player(mut player: Single<&mut Transform, With<Player>>) {
    let spawn = Transform::from_translation(PLAYER_SPAWN).looking_at(FORGE_CENTER, Vec3::Y);
    player.translation = spawn.translation;
    player.rotation = spawn.rotation;
}

fn create_material_palette(materials: &mut Assets<StandardMaterial>) -> MaterialPalette {
    MaterialPalette {
        basalt: materials.add(StandardMaterial {
            base_color: Color::srgb(0.13, 0.12, 0.14),
            metallic: 0.05,
            perceptual_roughness: 0.92,
            ..default()
        }),
        basalt_dark: materials.add(StandardMaterial {
            base_color: Color::srgb(0.06, 0.06, 0.08),
            metallic: 0.16,
            perceptual_roughness: 0.76,
            ..default()
        }),
        brass: materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.65, 0.3),
            metallic: 0.92,
            perceptual_roughness: 0.24,
            reflectance: 0.7,
            ..default()
        }),
        copper: materials.add(StandardMaterial {
            base_color: Color::srgb(0.82, 0.38, 0.18),
            emissive: Color::srgb(0.06, 0.015, 0.0).into(),
            metallic: 0.88,
            perceptual_roughness: 0.22,
            ..default()
        }),
        glass: materials.add(StandardMaterial {
            base_color: Color::srgba(0.24, 0.34, 0.42, 0.58),
            emissive: Color::srgb(0.04, 0.09, 0.12).into(),
            metallic: 0.72,
            perceptual_roughness: 0.08,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        ember: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.72, 0.36),
            emissive: Color::srgb(0.45, 0.18, 0.03).into(),
            unlit: true,
            ..default()
        }),
        ember_particle: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.78, 0.42),
            emissive: Color::srgb(0.65, 0.24, 0.05).into(),
            unlit: true,
            ..default()
        }),
        starlight: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: Color::srgb(0.9, 0.96, 1.0).into(),
            unlit: true,
            ..default()
        }),
    }
}

fn create_shader_palette(
    plasma_materials: &mut Assets<ForgePlasmaMaterial>,
    aurora_materials: &mut Assets<AuroraRibbonMaterial>,
) -> ShaderPalette {
    ShaderPalette {
        plasma_core: plasma_materials.add(ForgePlasmaMaterial {
            data: ForgePlasmaData {
                base_color: Vec4::new(0.95, 0.34, 0.08, 0.35),
                hot_color: Vec4::new(1.0, 0.92, 0.52, 1.0),
                swirl_scale: 0.38,
                pulse_speed: 1.1,
                fresnel_power: 3.5,
                _padding: 0,
            },
        }),
        plasma_stream: plasma_materials.add(ForgePlasmaMaterial {
            data: ForgePlasmaData {
                base_color: Vec4::new(0.08, 0.82, 0.95, 0.25),
                hot_color: Vec4::new(0.8, 1.0, 1.0, 1.0),
                swirl_scale: 0.62,
                pulse_speed: 0.75,
                fresnel_power: 2.1,
                _padding: 0,
            },
        }),
        aurora_primary: aurora_materials.add(AuroraRibbonMaterial {
            data: AuroraRibbonData {
                start_color: Vec4::new(0.08, 0.95, 0.55, 0.08),
                end_color: Vec4::new(0.08, 0.58, 1.0, 0.7),
                band_density: 8.0,
                flow_speed: 0.36,
                glow_strength: 1.8,
                alpha_bias: 0.55,
            },
        }),
        aurora_secondary: aurora_materials.add(AuroraRibbonMaterial {
            data: AuroraRibbonData {
                start_color: Vec4::new(0.72, 0.22, 1.0, 0.08),
                end_color: Vec4::new(0.14, 0.72, 0.98, 0.72),
                band_density: 5.4,
                flow_speed: 0.24,
                glow_strength: 1.35,
                alpha_bias: 0.48,
            },
        }),
    }
}

fn spawn_lighting(commands: &mut Commands) {
    commands.spawn((
        Name::new("Forge Moonlight"),
        DirectionalLight {
            illuminance: 8_000.0,
            color: Color::srgb(0.72, 0.82, 1.0),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-60.0, 90.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("Sky Fill"),
        PointLight {
            intensity: 4_500.0,
            range: 180.0,
            radius: 8.0,
            color: Color::srgb(0.18, 0.34, 0.45),
            ..default()
        },
        Transform::from_xyz(0.0, 72.0, 0.0),
    ));

    for index in 0..3 {
        let phase = index as f32 * TAU / 3.0;

        commands.spawn((
            Name::new(format!("Sweep Spotlight {}", index + 1)),
            SpotLight {
                intensity: 26_000.0,
                range: 110.0,
                radius: 0.45,
                color: Color::srgb(0.6, 0.9, 1.0),
                shadows_enabled: true,
                inner_angle: PI / 11.0,
                outer_angle: PI / 7.0,
                ..default()
            },
            Transform::from_xyz(0.0, 34.0, 0.0).looking_at(FORGE_CENTER, Vec3::Y),
            SweepSpotlight {
                center: Vec3::ZERO,
                radius: 74.0 + index as f32 * 5.0,
                height: 33.0 + index as f32 * 2.0,
                angular_speed: 0.05 + index as f32 * 0.02,
                phase,
                focus_height: FORGE_CENTER.y - 2.0,
                wobble: 12.0 + index as f32 * 2.5,
            },
        ));
    }
}

fn spawn_caldera(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &MaterialPalette,
    shader_palette: &ShaderPalette,
    terrain_noise: &Perlin,
) {
    let mut terrain_mesh = Plane3d::default()
        .mesh()
        .size(TERRAIN_SIZE, TERRAIN_SIZE)
        .subdivisions(TERRAIN_SUBDIVISIONS)
        .build();

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        terrain_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for position in positions.iter_mut() {
            position[1] = terrain_height(terrain_noise, position[0], position[2]) + TERRAIN_BASE_Y;
        }
    }

    terrain_mesh.compute_normals();

    let vertex_positions: Vec<Vec3> = terrain_mesh
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

    let indices: Vec<[u32; 3]> = terrain_mesh
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
        Name::new("Volcanic Caldera"),
        Mesh3d(meshes.add(terrain_mesh)),
        MeshMaterial3d(palette.basalt.clone()),
        RigidBody::Static,
        Collider::trimesh(vertex_positions, indices),
    ));

    for (index, (inner, outer, y_offset, speed)) in [
        (48.0, 51.5, -2.8, 0.12),
        (61.0, 64.2, -1.4, -0.08),
        (75.0, 78.0, 0.2, 0.05),
    ]
    .into_iter()
    .enumerate()
    {
        commands.spawn((
            Name::new(format!("Caldera Energy Ring {}", index.saturating_add(1))),
            Mesh3d(meshes.add(Torus::new(inner, outer))),
            MeshMaterial3d(shader_palette.plasma_stream.clone()),
            Transform::from_xyz(0.0, TERRAIN_BASE_Y + y_offset, 0.0),
            RingRotor {
                axis: Vec3::Y,
                speed,
            },
        ));
    }
}

fn spawn_main_deck(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &MaterialPalette,
    shader_palette: &ShaderPalette,
) {
    commands.spawn((
        Name::new("Forge Deck"),
        Mesh3d(meshes.add(Cylinder::new(MAIN_DECK_RADIUS, 2.2))),
        MeshMaterial3d(palette.basalt_dark.clone()),
        Transform::from_xyz(0.0, MAIN_DECK_Y, 0.0),
        RigidBody::Static,
        Collider::cylinder(MAIN_DECK_RADIUS, 2.2),
    ));

    commands.spawn((
        Name::new("Deck Outer Ring"),
        Mesh3d(meshes.add(Torus::new(MAIN_DECK_RADIUS - 0.9, MAIN_DECK_RADIUS + 0.1))),
        MeshMaterial3d(palette.brass.clone()),
        Transform::from_xyz(0.0, MAIN_DECK_Y + 1.12, 0.0),
    ));

    commands.spawn((
        Name::new("Deck Inner Ring"),
        Mesh3d(meshes.add(Torus::new(14.6, 15.8))),
        MeshMaterial3d(palette.copper.clone()),
        Transform::from_xyz(0.0, MAIN_DECK_Y + 1.08, 0.0),
    ));

    for index in 0..12 {
        let angle = index as f32 * TAU / 12.0;
        let radial_length = 10.5;
        let center = Vec3::new(angle.cos() * 20.0, MAIN_DECK_Y + 1.05, angle.sin() * 20.0);

        commands.spawn((
            Name::new(format!("Deck Spoke {}", index + 1)),
            Mesh3d(meshes.add(Cuboid::new(radial_length, 0.3, 1.1))),
            MeshMaterial3d(palette.brass.clone()),
            Transform::from_translation(center).with_rotation(Quat::from_rotation_y(angle)),
        ));
    }

    for index in 0..6 {
        let angle = index as f32 * TAU / 6.0;
        let forward = Vec3::new(angle.cos(), 0.0, angle.sin());
        let bridge_center = forward * 37.0 + Vec3::Y * (MAIN_DECK_Y + 0.9);
        let outcrop_center = forward * 47.0 + Vec3::Y * MAIN_DECK_Y;

        commands.spawn((
            Name::new(format!("Bridge {}", index + 1)),
            Mesh3d(meshes.add(Cuboid::new(14.0, 0.8, 4.2))),
            MeshMaterial3d(palette.basalt.clone()),
            Transform::from_translation(bridge_center).with_rotation(Quat::from_rotation_y(angle)),
            RigidBody::Static,
            Collider::cuboid(14.0, 0.8, 4.2),
        ));

        commands.spawn((
            Name::new(format!("Outcrop Platform {}", index + 1)),
            Mesh3d(meshes.add(Cylinder::new(8.4, 1.2))),
            MeshMaterial3d(palette.basalt_dark.clone()),
            Transform::from_translation(outcrop_center),
            RigidBody::Static,
            Collider::cylinder(8.4, 1.2),
        ));

        commands.spawn((
            Name::new(format!("Outcrop Rim {}", index + 1)),
            Mesh3d(meshes.add(Torus::new(7.6, 8.6))),
            MeshMaterial3d(palette.copper.clone()),
            Transform::from_translation(outcrop_center + Vec3::Y * 0.65),
        ));

        commands.spawn((
            Name::new(format!("Outcrop Beacon {}", index + 1)),
            Mesh3d(meshes.add(Sphere::new(1.1))),
            MeshMaterial3d(shader_palette.plasma_core.clone()),
            Transform::from_translation(outcrop_center + Vec3::new(0.0, 2.2, 0.0)),
        ));

        commands.spawn((
            Name::new(format!("Outcrop Light {}", index + 1)),
            PointLight {
                color: Color::srgb(1.0, 0.62, 0.28),
                intensity: 3_400.0,
                range: 24.0,
                radius: 0.65,
                shadows_enabled: index % 3 == 0,
                ..default()
            },
            Transform::from_translation(outcrop_center + Vec3::new(0.0, 3.4, 0.0)),
            PulseLight {
                base_intensity: 2_400.0,
                amplitude: 1_800.0,
                speed: 0.8 + index as f32 * 0.16,
                phase: index as f32 * 0.65,
            },
        ));

        for support_index in 0..3 {
            let support_angle = angle + (support_index as f32 - 1.0) * 0.2;
            let support_position = outcrop_center
                + Vec3::new(support_angle.cos() * 2.2, -8.0, support_angle.sin() * 2.2);

            commands.spawn((
                Name::new(format!(
                    "Outcrop Support {}-{}",
                    index + 1,
                    support_index + 1
                )),
                Mesh3d(meshes.add(Cylinder::new(0.8, 16.0))),
                MeshMaterial3d(palette.basalt.clone()),
                Transform::from_translation(support_position),
            ));
        }
    }
}

fn spawn_outer_crown(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &MaterialPalette,
    shader_palette: &ShaderPalette,
    terrain_noise: &Perlin,
) {
    for index in 0..8 {
        let angle = index as f32 * TAU / 8.0 + PI / 16.0;
        let radius = 88.0 + (index as f32 * 1.7).sin() * 6.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let base_y = terrain_height(terrain_noise, x, z) + TERRAIN_BASE_Y;
        let monolith_height = 18.0 + (index as f32 * 1.9).sin().abs() * 8.0;

        commands.spawn((
            Name::new(format!("Crown Monolith {}", index + 1)),
            Mesh3d(meshes.add(Cuboid::new(6.0, monolith_height, 6.0))),
            MeshMaterial3d(palette.basalt_dark.clone()),
            Transform::from_xyz(x, base_y + monolith_height * 0.5, z).with_rotation(
                Quat::from_euler(EulerRot::XYZ, 0.04 * angle.sin(), angle, 0.05 * angle.cos()),
            ),
        ));

        commands.spawn((
            Name::new(format!("Monolith Beacon {}", index + 1)),
            Mesh3d(meshes.add(Sphere::new(1.2))),
            MeshMaterial3d(shader_palette.plasma_stream.clone()),
            Transform::from_xyz(x, base_y + monolith_height + 2.4, z),
        ));

        commands.spawn((
            Name::new(format!("Monolith Light {}", index + 1)),
            PointLight {
                color: Color::srgb(0.32, 0.88, 1.0),
                intensity: 4_800.0,
                range: 34.0,
                radius: 0.9,
                shadows_enabled: false,
                ..default()
            },
            Transform::from_xyz(x, base_y + monolith_height + 2.8, z),
            PulseLight {
                base_intensity: 3_400.0,
                amplitude: 2_600.0,
                speed: 0.5 + index as f32 * 0.11,
                phase: index as f32 * 0.9,
            },
        ));
    }
}

fn spawn_central_forge(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &MaterialPalette,
    shader_palette: &ShaderPalette,
) {
    commands.spawn((
        Name::new("Forge Inner Platform"),
        Mesh3d(meshes.add(Cylinder::new(12.5, 1.8))),
        MeshMaterial3d(palette.basalt.clone()),
        Transform::from_xyz(0.0, MAIN_DECK_Y + 1.9, 0.0),
        RigidBody::Static,
        Collider::cylinder(12.5, 1.8),
    ));

    commands.spawn((
        Name::new("Forge Inner Ring"),
        Mesh3d(meshes.add(Torus::new(11.8, 12.8))),
        MeshMaterial3d(palette.brass.clone()),
        Transform::from_xyz(0.0, MAIN_DECK_Y + 2.85, 0.0),
    ));

    for index in 0..6 {
        let angle = index as f32 * TAU / 6.0;
        let support_position = Vec3::new(angle.cos() * 8.6, MAIN_DECK_Y + 8.0, angle.sin() * 8.6);

        commands.spawn((
            Name::new(format!("Forge Conduit {}", index + 1)),
            Mesh3d(meshes.add(Cylinder::new(0.48, 11.0))),
            MeshMaterial3d(shader_palette.plasma_stream.clone()),
            Transform::from_translation(support_position),
        ));

        commands.spawn((
            Name::new(format!("Forge Buttress {}", index + 1)),
            Mesh3d(meshes.add(Cuboid::new(2.4, 10.0, 4.8))),
            MeshMaterial3d(palette.basalt_dark.clone()),
            Transform::from_xyz(angle.cos() * 10.2, MAIN_DECK_Y + 5.0, angle.sin() * 10.2)
                .with_rotation(Quat::from_rotation_y(angle)),
        ));
    }

    for (index, (inner, outer, height, tilt, speed)) in [
        (5.4, 5.9, 15.0, 0.0, 0.4),
        (7.2, 7.8, 18.9, PI / 3.4, -0.32),
        (9.5, 10.1, 22.5, -PI / 5.0, 0.25),
    ]
    .into_iter()
    .enumerate()
    {
        commands.spawn((
            Name::new(format!("Forge Ring {}", index.saturating_add(1))),
            Mesh3d(meshes.add(Torus::new(inner, outer))),
            MeshMaterial3d(palette.brass.clone()),
            Transform::from_xyz(0.0, height, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                tilt,
                0.0,
                PI / 2.0,
            )),
            RingRotor {
                axis: Vec3::Y,
                speed,
            },
        ));
    }

    commands.spawn((
        Name::new("Forge Plasma Core"),
        Mesh3d(meshes.add(Sphere::new(3.4))),
        MeshMaterial3d(shader_palette.plasma_core.clone()),
        Transform::from_translation(FORGE_CENTER),
    ));

    commands.spawn((
        Name::new("Forge Plasma Halo"),
        Mesh3d(meshes.add(Sphere::new(4.7))),
        MeshMaterial3d(shader_palette.plasma_stream.clone()),
        Transform::from_translation(FORGE_CENTER),
        HoverMotion {
            anchor: FORGE_CENTER,
            vertical_amplitude: 0.45,
            lateral_amplitude: 0.22,
            speed: 0.3,
            phase: 0.0,
            yaw_offset: 0.0,
            pitch_tilt: 0.0,
        },
    ));

    commands.spawn((
        Name::new("Forge Heart Light"),
        PointLight {
            color: Color::srgb(1.0, 0.72, 0.44),
            intensity: 9_500.0,
            range: 42.0,
            radius: 1.5,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(FORGE_CENTER),
        PulseLight {
            base_intensity: 6_200.0,
            amplitude: 5_000.0,
            speed: 1.05,
            phase: 0.0,
        },
    ));

    commands.spawn((
        Name::new("Forge Crown"),
        Mesh3d(meshes.add(Cylinder::new(2.4, 2.0))),
        MeshMaterial3d(palette.glass.clone()),
        Transform::from_xyz(0.0, FORGE_CENTER.y + 6.4, 0.0),
    ));

    for index in 0..12 {
        let angle = index as f32 * TAU / 12.0;
        let position = Vec3::new(angle.cos() * 4.9, FORGE_CENTER.y, angle.sin() * 4.9);

        commands.spawn((
            Name::new(format!("Forge Cage Strut {}", index + 1)),
            Mesh3d(meshes.add(Cuboid::new(0.45, 12.0, 0.45))),
            MeshMaterial3d(palette.copper.clone()),
            Transform::from_translation(position + Vec3::Y * 0.8)
                .with_rotation(Quat::from_rotation_y(angle)),
        ));
    }
}

fn spawn_shard_halo(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &MaterialPalette) {
    let shard_mesh = meshes.add(Capsule3d::new(0.28, 3.4));

    for index in 0..18 {
        let phase = index as f32 * TAU / 18.0;
        let radius = 15.0 + (index as f32 * 0.8).sin().abs() * 5.0;
        let height = -1.0 + (index as f32 * 0.43).sin() * 6.0;

        commands.spawn((
            Name::new(format!("Forge Shard {}", index + 1)),
            Mesh3d(shard_mesh.clone()),
            MeshMaterial3d(if index % 3 == 0 {
                palette.copper.clone()
            } else {
                palette.glass.clone()
            }),
            Transform::from_xyz(0.0, FORGE_CENTER.y + height, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                PI / 4.0,
                phase,
                phase * 0.5,
            )),
            OrbitMotion {
                center: FORGE_CENTER,
                radius,
                height,
                angular_speed: 0.12 + (index % 5) as f32 * 0.03,
                vertical_amplitude: 0.9 + (index % 4) as f32 * 0.25,
                vertical_speed: 0.5 + (index % 6) as f32 * 0.08,
                phase,
                spin_speed: 0.7 + (index % 3) as f32 * 0.3,
            },
        ));
    }
}

fn spawn_aurora_sails(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    shader_palette: &ShaderPalette,
) {
    for index in 0..6 {
        let angle = index as f32 * TAU / 6.0 + PI / 9.0;
        let radius = 78.0 + (index as f32 * 1.5).sin() * 12.0;
        let anchor = Vec3::new(
            angle.cos() * radius,
            48.0 + (index as f32 * 0.9).sin() * 7.0,
            angle.sin() * radius,
        );
        let ribbon_mesh = if index % 2 == 0 {
            meshes.add(Cuboid::new(52.0, 15.0, 0.55))
        } else {
            meshes.add(Cuboid::new(40.0, 11.0, 0.45))
        };

        commands.spawn((
            Name::new(format!("Aurora Sail {}", index + 1)),
            Mesh3d(ribbon_mesh),
            MeshMaterial3d(if index % 2 == 0 {
                shader_palette.aurora_primary.clone()
            } else {
                shader_palette.aurora_secondary.clone()
            }),
            Transform::from_translation(anchor).with_rotation(Quat::from_euler(
                EulerRot::YXZ,
                angle + PI * 0.5,
                -0.22 + (index as f32 * 0.35).sin() * 0.18,
                0.05,
            )),
            HoverMotion {
                anchor,
                vertical_amplitude: 1.8 + (index % 3) as f32 * 0.45,
                lateral_amplitude: 2.8 + (index % 2) as f32 * 0.9,
                speed: 0.09 + index as f32 * 0.012,
                phase: index as f32 * 0.8,
                yaw_offset: angle + PI * 0.5,
                pitch_tilt: -0.22 + (index as f32 * 0.35).sin() * 0.18,
            },
        ));
    }
}

fn spawn_ember_swarm(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &MaterialPalette,
) {
    let ember_mesh = meshes.add(Sphere::new(0.12));
    let wisp_mesh = meshes.add(Sphere::new(0.4));

    for index in 0..84 {
        let normalized = index as f32 / 84.0;
        let radius = 4.0 + normalized * 24.0;
        let height = -6.0 + (index as f32 * 0.37).sin() * 10.0;
        let phase = index as f32 * 0.47;

        commands.spawn((
            Name::new(format!("Ember {}", index + 1)),
            Mesh3d(ember_mesh.clone()),
            MeshMaterial3d(palette.ember_particle.clone()),
            Transform::from_xyz(0.0, FORGE_CENTER.y + height, 0.0),
            OrbitMotion {
                center: FORGE_CENTER,
                radius,
                height,
                angular_speed: 0.25 + (index % 8) as f32 * 0.045,
                vertical_amplitude: 0.7 + (index % 5) as f32 * 0.18,
                vertical_speed: 0.8 + (index % 7) as f32 * 0.12,
                phase,
                spin_speed: 0.0,
            },
        ));
    }

    for index in 0..6 {
        let phase = index as f32 * TAU / 6.0;
        let height = -2.0 + index as f32 * 1.6;

        commands.spawn((
            Name::new(format!("Forge Wisp {}", index + 1)),
            Mesh3d(wisp_mesh.clone()),
            MeshMaterial3d(palette.ember.clone()),
            Transform::from_xyz(0.0, FORGE_CENTER.y + height, 0.0),
            OrbitMotion {
                center: FORGE_CENTER,
                radius: 7.0 + index as f32 * 2.0,
                height,
                angular_speed: 0.18 + index as f32 * 0.05,
                vertical_amplitude: 1.0,
                vertical_speed: 0.55 + index as f32 * 0.08,
                phase,
                spin_speed: 0.5,
            },
        ));

        commands.spawn((
            Name::new(format!("Forge Wisp Light {}", index + 1)),
            PointLight {
                color: Color::srgb(1.0, 0.76, 0.42),
                intensity: 1_600.0,
                range: 16.0,
                radius: 0.45,
                shadows_enabled: false,
                ..default()
            },
            Transform::from_xyz(0.0, FORGE_CENTER.y + height, 0.0),
            PulseLight {
                base_intensity: 1_000.0,
                amplitude: 900.0,
                speed: 1.2 + index as f32 * 0.15,
                phase,
            },
            OrbitMotion {
                center: FORGE_CENTER,
                radius: 7.0 + index as f32 * 2.0,
                height,
                angular_speed: 0.18 + index as f32 * 0.05,
                vertical_amplitude: 1.0,
                vertical_speed: 0.55 + index as f32 * 0.08,
                phase,
                spin_speed: 0.0,
            },
        ));
    }
}

fn spawn_floating_islets(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &MaterialPalette,
) {
    for index in 0..6 {
        let angle = index as f32 * TAU / 6.0 + PI / 12.0;
        let anchor = Vec3::new(
            angle.cos() * (58.0 + (index % 2) as f32 * 9.0),
            29.0 + (index as f32 * 0.7).sin() * 4.0,
            angle.sin() * (58.0 + (index % 3) as f32 * 8.0),
        );

        commands.spawn((
            Name::new(format!("Floating Islet {}", index + 1)),
            Mesh3d(meshes.add(Cylinder::new(4.8 + (index % 3) as f32, 2.8))),
            MeshMaterial3d(palette.basalt.clone()),
            Transform::from_translation(anchor),
            HoverMotion {
                anchor,
                vertical_amplitude: 0.9 + (index % 3) as f32 * 0.25,
                lateral_amplitude: 1.2 + (index % 2) as f32 * 0.4,
                speed: 0.18 + index as f32 * 0.03,
                phase: index as f32 * 0.8,
                yaw_offset: angle,
                pitch_tilt: 0.03,
            },
        ));

        commands.spawn((
            Name::new(format!("Islet Ring {}", index + 1)),
            Mesh3d(meshes.add(Torus::new(4.4, 5.2))),
            MeshMaterial3d(palette.brass.clone()),
            Transform::from_translation(anchor + Vec3::Y * 0.8),
            HoverMotion {
                anchor: anchor + Vec3::Y * 0.8,
                vertical_amplitude: 0.7,
                lateral_amplitude: 1.0,
                speed: 0.22 + index as f32 * 0.025,
                phase: index as f32 * 0.8 + PI / 4.0,
                yaw_offset: 0.0,
                pitch_tilt: PI / 2.0,
            },
        ));
    }
}

fn spawn_distant_spires(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &MaterialPalette,
    terrain_noise: &Perlin,
) {
    for index in 0..20 {
        let angle = index as f32 * TAU / 20.0 + PI / 20.0;
        let radius = 118.0 + (index as f32 * 1.13).sin() * 10.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let base_y = terrain_height(terrain_noise, x, z) + TERRAIN_BASE_Y;
        let height = 14.0 + (index as f32 * 0.91).sin().abs() * 26.0;
        let width = 3.6 + (index % 4) as f32 * 1.2;

        commands.spawn((
            Name::new(format!("Distant Spire {}", index + 1)),
            Mesh3d(meshes.add(Cuboid::new(width, height, width))),
            MeshMaterial3d(if index % 5 == 0 {
                palette.basalt_dark.clone()
            } else {
                palette.basalt.clone()
            }),
            Transform::from_xyz(x, base_y + height * 0.5, z).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.02 * (index as f32).sin(),
                angle,
                0.02 * (index as f32 * 1.4).cos(),
            )),
        ));
    }
}

fn spawn_star_canopy(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &MaterialPalette,
) {
    let star_mesh = meshes.add(Sphere::new(0.16));
    const STAR_COUNT: usize = 220;
    const GOLDEN_ANGLE: f32 = 2.3999631;

    for index in 0..STAR_COUNT {
        let t = index as f32 / STAR_COUNT as f32;
        let y = 1.0 - 2.0 * t;
        let radial = (1.0 - y * y).sqrt();
        let angle = GOLDEN_ANGLE * index as f32;
        let direction = Vec3::new(angle.cos() * radial, y, angle.sin() * radial);
        let radius = 165.0 + (index as f32 * 0.37).sin().abs() * 34.0;
        let scale = 0.08 + (index % 7) as f32 * 0.018;

        commands.spawn((
            Name::new(format!("Star {}", index.saturating_add(1))),
            Mesh3d(star_mesh.clone()),
            MeshMaterial3d(palette.starlight.clone()),
            Transform::from_translation(direction * radius).with_scale(Vec3::splat(scale)),
        ));
    }
}

fn terrain_height(perlin: &Perlin, x: f32, z: f32) -> f32 {
    let radius = Vec2::new(x, z).length();
    let base = perlin.get([x as f64 * 0.024, z as f64 * 0.024]) as f32 * 5.6
        + perlin.get([x as f64 * 0.072, z as f64 * 0.072]) as f32 * 1.9;
    let crater = -16.0 * (1.0 - (radius / 38.0).clamp(0.0, 1.0)).powf(1.8);
    let rim = gaussian(radius, 62.0, 11.5) * 13.8;
    let terraces = ((radius * 0.22).sin() * 0.5 + 0.5) * 1.6;
    let radial_wave = ((z.atan2(x) * 7.0) + radius * 0.14).sin();
    let fissure_band = smooth_band(radius, 56.0, 104.0);
    let fissures = -radial_wave.abs().powf(4.0) * 3.1 * fissure_band;

    base + crater + rim + terraces + fissures
}

fn gaussian(value: f32, mean: f32, width: f32) -> f32 {
    let delta = value - mean;
    (-delta * delta / (2.0 * width * width)).exp()
}

fn smooth_band(value: f32, min: f32, max: f32) -> f32 {
    smoothstep(min, min + 12.0, value) * (1.0 - smoothstep(max - 12.0, max, value))
}

fn smoothstep(edge0: f32, edge1: f32, value: f32) -> f32 {
    let t = ((value - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
