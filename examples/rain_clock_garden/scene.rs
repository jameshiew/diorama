use std::f32::consts::{PI, TAU};

use avian3d::prelude::*;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use diorama::player::Player;

use crate::animation::{ClockHand, Drift, FallingDrop, Pendulum, PulseLight, Ripple, Spin};

const CLEAR_COLOR: Color = Color::srgb(0.008, 0.016, 0.035);
const GARDEN_RADIUS: f32 = 35.0;
const POOL_RADIUS: f32 = 12.5;
const WATER_Y: f32 = 0.28;
const CLOCK_CENTER: Vec3 = Vec3::new(0.0, 9.3, 0.0);
const PLAYER_SPAWN: Vec3 = Vec3::new(0.0, 2.4, 28.0);
const PLAYER_LOOK_AT: Vec3 = Vec3::new(0.0, 8.5, 0.0);

struct Palette {
    stone: Handle<StandardMaterial>,
    dark_stone: Handle<StandardMaterial>,
    brass: Handle<StandardMaterial>,
    verdigris: Handle<StandardMaterial>,
    water: Handle<StandardMaterial>,
    rain: Handle<StandardMaterial>,
    glass: Handle<StandardMaterial>,
    cyan_glow: Handle<StandardMaterial>,
    amber_glow: Handle<StandardMaterial>,
    foliage: Handle<StandardMaterial>,
    moon: Handle<StandardMaterial>,
    moon_halo: Handle<StandardMaterial>,
    star: Handle<StandardMaterial>,
    mist: Handle<StandardMaterial>,
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(ClearColor(CLEAR_COLOR));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.16, 0.24, 0.38),
        brightness: 170.0,
        affects_lightmapped_meshes: true,
    });

    let palette = build_palette(&mut materials);

    spawn_lighting(&mut commands);
    spawn_foundation(&mut commands, &mut meshes, &palette);
    spawn_pool_and_walkways(&mut commands, &mut meshes, &palette);
    spawn_clock_pavilion(&mut commands, &mut meshes, &palette);
    spawn_rainfall(&mut commands, &mut meshes, &palette);
    spawn_ripples(&mut commands, &mut meshes, &palette);
    spawn_rain_beds(&mut commands, &mut meshes, &palette);
    spawn_lanterns(&mut commands, &mut meshes, &palette);
    spawn_colonnade(&mut commands, &mut meshes, &palette);
    spawn_edge_waterfalls(&mut commands, &mut meshes, &palette);
    spawn_moon_and_stars(&mut commands, &mut meshes, &palette);
}

pub fn spawn_player(mut player: Single<&mut Transform, With<Player>>) {
    let spawn = Transform::from_translation(PLAYER_SPAWN).looking_at(PLAYER_LOOK_AT, Vec3::Y);
    player.translation = spawn.translation;
    player.rotation = spawn.rotation;
}

fn build_palette(materials: &mut Assets<StandardMaterial>) -> Palette {
    Palette {
        stone: materials.add(StandardMaterial {
            base_color: Color::srgb(0.19, 0.22, 0.25),
            metallic: 0.08,
            perceptual_roughness: 0.82,
            ..default()
        }),
        dark_stone: materials.add(StandardMaterial {
            base_color: Color::srgb(0.045, 0.065, 0.09),
            metallic: 0.12,
            perceptual_roughness: 0.74,
            ..default()
        }),
        brass: materials.add(StandardMaterial {
            base_color: Color::srgb(0.76, 0.51, 0.17),
            metallic: 0.92,
            perceptual_roughness: 0.2,
            reflectance: 0.78,
            ..default()
        }),
        verdigris: materials.add(StandardMaterial {
            base_color: Color::srgb(0.12, 0.38, 0.36),
            emissive: Color::srgb(0.015, 0.08, 0.075).into(),
            metallic: 0.72,
            perceptual_roughness: 0.3,
            ..default()
        }),
        water: materials.add(StandardMaterial {
            base_color: Color::srgba(0.04, 0.22, 0.31, 0.72),
            emissive: Color::srgb(0.015, 0.12, 0.17).into(),
            metallic: 0.28,
            perceptual_roughness: 0.12,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        rain: materials.add(StandardMaterial {
            base_color: Color::srgba(0.42, 0.9, 1.0, 0.78),
            emissive: Color::srgb(0.18, 0.68, 0.82).into(),
            alpha_mode: AlphaMode::Add,
            unlit: true,
            ..default()
        }),
        glass: materials.add(StandardMaterial {
            base_color: Color::srgba(0.14, 0.48, 0.56, 0.42),
            emissive: Color::srgb(0.04, 0.22, 0.28).into(),
            metallic: 0.35,
            perceptual_roughness: 0.08,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        cyan_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(0.45, 0.96, 1.0),
            emissive: Color::srgb(0.35, 1.15, 1.35).into(),
            unlit: true,
            ..default()
        }),
        amber_glow: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.68, 0.24),
            emissive: Color::srgb(1.25, 0.52, 0.08).into(),
            unlit: true,
            ..default()
        }),
        foliage: materials.add(StandardMaterial {
            base_color: Color::srgb(0.055, 0.18, 0.14),
            emissive: Color::srgb(0.008, 0.035, 0.025).into(),
            metallic: 0.04,
            perceptual_roughness: 0.88,
            ..default()
        }),
        moon: materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.94, 1.0),
            emissive: Color::srgb(1.1, 1.18, 1.35).into(),
            unlit: true,
            ..default()
        }),
        moon_halo: materials.add(StandardMaterial {
            base_color: Color::srgba(0.32, 0.52, 0.82, 0.08),
            emissive: Color::srgb(0.12, 0.24, 0.5).into(),
            alpha_mode: AlphaMode::Add,
            unlit: true,
            ..default()
        }),
        star: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: Color::srgb(1.0, 1.08, 1.25).into(),
            unlit: true,
            ..default()
        }),
        mist: materials.add(StandardMaterial {
            base_color: Color::srgba(0.12, 0.2, 0.32, 0.12),
            emissive: Color::srgb(0.018, 0.03, 0.06).into(),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
    }
}

fn spawn_lighting(commands: &mut Commands) {
    commands.spawn((
        Name::new("Moonlight"),
        DirectionalLight {
            illuminance: 4_200.0,
            color: Color::srgb(0.58, 0.72, 1.0),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-35.0, 58.0, 34.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("Clock Glow"),
        PointLight {
            intensity: 8_000.0,
            range: 38.0,
            radius: 2.5,
            color: Color::srgb(0.18, 0.72, 0.9),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_translation(CLOCK_CENTER + Vec3::new(0.0, 0.0, 2.0)),
        PulseLight {
            base_intensity: 6_500.0,
            amplitude: 2_200.0,
            speed: 0.7,
            phase: 0.0,
        },
    ));

    commands.spawn((
        Name::new("Pool Fill"),
        PointLight {
            intensity: 3_600.0,
            range: 32.0,
            radius: 5.0,
            color: Color::srgb(0.08, 0.38, 0.48),
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 3.0, 0.0),
    ));
}

fn spawn_foundation(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    commands.spawn((
        Name::new("Garden Foundation"),
        Mesh3d(meshes.add(Cylinder::new(GARDEN_RADIUS, 1.4))),
        MeshMaterial3d(palette.dark_stone.clone()),
        Transform::from_xyz(0.0, -0.7, 0.0),
        RigidBody::Static,
        Collider::cylinder(GARDEN_RADIUS, 1.4),
    ));

    commands.spawn((
        Name::new("Garden Inlay"),
        Mesh3d(meshes.add(Cylinder::new(GARDEN_RADIUS - 1.3, 0.16))),
        MeshMaterial3d(palette.stone.clone()),
        Transform::from_xyz(0.0, 0.05, 0.0),
    ));

    commands.spawn((
        Name::new("Foundation Brass Rim"),
        Mesh3d(meshes.add(Torus::new(GARDEN_RADIUS - 0.8, GARDEN_RADIUS))),
        MeshMaterial3d(palette.brass.clone()),
        Transform::from_xyz(0.0, 0.18, 0.0),
    ));
}

fn spawn_pool_and_walkways(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    commands.spawn((
        Name::new("Clock Pool"),
        Mesh3d(meshes.add(Cylinder::new(POOL_RADIUS, 0.16))),
        MeshMaterial3d(palette.water.clone()),
        Transform::from_xyz(0.0, WATER_Y, 0.0),
        NotShadowCaster,
    ));

    commands.spawn((
        Name::new("Clock Pool Rim"),
        Mesh3d(meshes.add(Torus::new(POOL_RADIUS - 0.2, POOL_RADIUS + 0.7))),
        MeshMaterial3d(palette.verdigris.clone()),
        Transform::from_xyz(0.0, WATER_Y + 0.08, 0.0),
    ));

    let bridge_mesh = meshes.add(Cuboid::new(4.2, 0.42, 17.0));
    let rail_mesh = meshes.add(Cuboid::new(0.16, 0.22, 16.5));

    for index in 0..4_usize {
        let angle = index as f32 * PI * 0.5;
        let direction = Vec3::new(angle.sin(), 0.0, angle.cos());
        let rotation = Quat::from_rotation_y(angle);
        let center = direction * 21.2 + Vec3::Y * 0.34;

        commands.spawn((
            Name::new(format!("Garden Walkway {}", index.saturating_add(1))),
            Mesh3d(bridge_mesh.clone()),
            MeshMaterial3d(palette.dark_stone.clone()),
            Transform::from_translation(center).with_rotation(rotation),
        ));

        for side in [-1.0, 1.0] {
            let offset = rotation * Vec3::new(side * 1.85, 0.34, 0.0);
            commands.spawn((
                Name::new(format!("Walkway {} Brass Edge", index.saturating_add(1))),
                Mesh3d(rail_mesh.clone()),
                MeshMaterial3d(palette.brass.clone()),
                Transform::from_translation(center + offset).with_rotation(rotation),
            ));
        }
    }
}

fn spawn_clock_pavilion(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let column_mesh = meshes.add(Cylinder::new(0.72, 15.0));
    let column_ring_mesh = meshes.add(Torus::new(0.7, 0.92));
    let cap_mesh = meshes.add(Cuboid::new(1.8, 0.52, 1.8));

    for (index, x) in [-7.2, 7.2].into_iter().enumerate() {
        commands.spawn((
            Name::new(format!("Clock Pier {}", index.saturating_add(1))),
            Mesh3d(column_mesh.clone()),
            MeshMaterial3d(palette.stone.clone()),
            Transform::from_xyz(x, 7.75, 0.5),
            RigidBody::Static,
            Collider::cylinder(0.72, 15.0),
        ));

        for y in [1.1, 6.2, 11.2, 15.0] {
            commands.spawn((
                Name::new(format!("Clock Pier {} Collar", index.saturating_add(1))),
                Mesh3d(column_ring_mesh.clone()),
                MeshMaterial3d(palette.brass.clone()),
                Transform::from_xyz(x, y, 0.5),
            ));
        }

        commands.spawn((
            Name::new(format!("Clock Pier {} Cap", index.saturating_add(1))),
            Mesh3d(cap_mesh.clone()),
            MeshMaterial3d(palette.dark_stone.clone()),
            Transform::from_xyz(x, 15.4, 0.5),
        ));
    }

    commands.spawn((
        Name::new("Clock Crown Beam"),
        Mesh3d(meshes.add(Cuboid::new(16.4, 0.8, 1.5))),
        MeshMaterial3d(palette.dark_stone.clone()),
        Transform::from_xyz(0.0, 15.7, 0.5),
    ));

    commands.spawn((
        Name::new("Clock Crown Channel"),
        Mesh3d(meshes.add(Torus::new(8.0, 9.0))),
        MeshMaterial3d(palette.verdigris.clone()),
        Transform::from_xyz(0.0, 16.15, 0.0),
    ));

    let rib_mesh = meshes.add(Cuboid::new(0.18, 0.18, 8.0));
    for index in 0..12_usize {
        let angle = index as f32 * TAU / 12.0;
        commands.spawn((
            Name::new(format!("Crown Rib {}", index.saturating_add(1))),
            Mesh3d(rib_mesh.clone()),
            MeshMaterial3d(palette.brass.clone()),
            Transform::from_xyz(angle.sin() * 4.0, 16.2, angle.cos() * 4.0)
                .with_rotation(Quat::from_rotation_y(angle)),
        ));
    }

    commands.spawn((
        Name::new("Clock Outer Face"),
        Mesh3d(meshes.add(Torus::new(5.55, 6.05))),
        MeshMaterial3d(palette.verdigris.clone()),
        Transform::from_translation(CLOCK_CENTER).with_rotation(Quat::from_rotation_x(PI * 0.5)),
    ));

    spawn_clock_ticks(commands, meshes, palette);
    spawn_instrument_ring(
        commands,
        meshes,
        palette,
        "Minute Water Ring",
        4.65,
        0.16,
        -0.11,
        Quat::from_rotation_x(PI * 0.5) * Quat::from_rotation_y(0.22),
    );
    spawn_instrument_ring(
        commands,
        meshes,
        palette,
        "Tide Water Ring",
        3.55,
        0.13,
        0.17,
        Quat::from_rotation_x(PI * 0.5) * Quat::from_rotation_z(-0.18),
    );
    spawn_clock_hands(commands, meshes, palette);
    spawn_pendulum(commands, meshes, palette);

    commands.spawn((
        Name::new("Clock Glass Heart"),
        Mesh3d(meshes.add(Sphere::new(1.25))),
        MeshMaterial3d(palette.glass.clone()),
        Transform::from_translation(CLOCK_CENTER + Vec3::new(0.0, 0.0, 0.75)),
        NotShadowCaster,
    ));

    commands.spawn((
        Name::new("Clock Heart"),
        Mesh3d(meshes.add(Sphere::new(0.62))),
        MeshMaterial3d(palette.cyan_glow.clone()),
        Transform::from_translation(CLOCK_CENTER + Vec3::new(0.0, 0.0, 0.86)),
        NotShadowCaster,
    ));
}

fn spawn_clock_ticks(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let major_tick = meshes.add(Cuboid::new(0.22, 0.85, 0.22));
    let minor_tick = meshes.add(Cuboid::new(0.12, 0.48, 0.16));

    for index in 0..24_usize {
        let angle = index as f32 * TAU / 24.0;
        let radius = 5.25;
        let mesh = if index % 2 == 0 {
            major_tick.clone()
        } else {
            minor_tick.clone()
        };

        commands.spawn((
            Name::new(format!("Clock Tick {}", index.saturating_add(1))),
            Mesh3d(mesh),
            MeshMaterial3d(palette.brass.clone()),
            Transform::from_xyz(
                CLOCK_CENTER.x + angle.sin() * radius,
                CLOCK_CENTER.y + angle.cos() * radius,
                CLOCK_CENTER.z + 0.42,
            )
            .with_rotation(Quat::from_rotation_z(-angle)),
        ));
    }
}

fn spawn_instrument_ring(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    palette: &Palette,
    name: &'static str,
    radius: f32,
    half_thickness: f32,
    speed: f32,
    rotation: Quat,
) {
    let ring_mesh = meshes.add(Torus::new(radius - half_thickness, radius + half_thickness));
    let marker_mesh = meshes.add(Sphere::new(0.2));

    commands
        .spawn((
            Name::new(name),
            Transform::from_translation(CLOCK_CENTER).with_rotation(rotation),
            Visibility::default(),
            Spin {
                local_axis: Vec3::Y,
                radians_per_second: speed,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Ring Arc"),
                Mesh3d(ring_mesh),
                MeshMaterial3d(palette.brass.clone()),
            ));

            for index in 0..6_usize {
                let angle = index as f32 * TAU / 6.0;
                parent.spawn((
                    Name::new(format!("Water Pearl {}", index.saturating_add(1))),
                    Mesh3d(marker_mesh.clone()),
                    MeshMaterial3d(palette.cyan_glow.clone()),
                    Transform::from_xyz(angle.cos() * radius, 0.0, angle.sin() * radius),
                    NotShadowCaster,
                ));
            }
        });
}

fn spawn_clock_hands(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let minute_mesh = meshes.add(Cuboid::new(0.16, 4.4, 0.16));
    let hour_mesh = meshes.add(Cuboid::new(0.28, 3.1, 0.22));

    commands
        .spawn((
            Name::new("Minute Hand"),
            Transform::from_translation(CLOCK_CENTER + Vec3::new(0.0, 0.0, 0.66)),
            Visibility::default(),
            ClockHand {
                radians_per_second: 0.2,
                phase: 0.45,
            },
        ))
        .with_child((
            Mesh3d(minute_mesh),
            MeshMaterial3d(palette.cyan_glow.clone()),
            Transform::from_xyz(0.0, 2.2, 0.0),
            NotShadowCaster,
        ));

    commands
        .spawn((
            Name::new("Hour Hand"),
            Transform::from_translation(CLOCK_CENTER + Vec3::new(0.0, 0.0, 0.72)),
            Visibility::default(),
            ClockHand {
                radians_per_second: 0.045,
                phase: -1.05,
            },
        ))
        .with_child((
            Mesh3d(hour_mesh),
            MeshMaterial3d(palette.amber_glow.clone()),
            Transform::from_xyz(0.0, 1.55, 0.0),
            NotShadowCaster,
        ));
}

fn spawn_pendulum(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    commands
        .spawn((
            Name::new("Rain Pendulum"),
            Transform::from_translation(CLOCK_CENTER + Vec3::new(0.0, -0.4, -0.65)),
            Visibility::default(),
            Pendulum {
                amplitude: 0.34,
                speed: 0.72,
                phase: 0.0,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Pendulum Rod"),
                Mesh3d(meshes.add(Cuboid::new(0.13, 7.2, 0.13))),
                MeshMaterial3d(palette.brass.clone()),
                Transform::from_xyz(0.0, -3.6, 0.0),
            ));
            parent.spawn((
                Name::new("Pendulum Bob"),
                Mesh3d(meshes.add(Sphere::new(0.82))),
                MeshMaterial3d(palette.verdigris.clone()),
                Transform::from_xyz(0.0, -7.15, 0.0).with_scale(Vec3::new(1.0, 1.3, 0.55)),
            ));
            parent.spawn((
                Name::new("Pendulum Drop"),
                Mesh3d(meshes.add(Sphere::new(0.3))),
                MeshMaterial3d(palette.cyan_glow.clone()),
                Transform::from_xyz(0.0, -7.15, 0.52),
                NotShadowCaster,
            ));
        });
}

fn spawn_rainfall(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let drop_mesh = meshes.add(Capsule3d::new(0.045, 0.34));
    let stream_mesh = meshes.add(Cylinder::new(0.028, 13.7));
    let top_y = 15.7;
    let bottom_y = WATER_Y + 0.18;
    let travel = top_y - bottom_y;

    for index in 0..128_usize {
        let angle = index as f32 * 2.399_963_1;
        let radius = 1.2 + unit_hash(index, 43, 127) * 7.5;
        let phase_distance = unit_hash(index, 71, 131) * travel;

        commands.spawn((
            Name::new(format!("Rain Drop {}", index.saturating_add(1))),
            Mesh3d(drop_mesh.clone()),
            MeshMaterial3d(palette.rain.clone()),
            Transform::from_xyz(
                angle.cos() * radius,
                top_y - phase_distance,
                angle.sin() * radius,
            ),
            FallingDrop {
                top_y,
                bottom_y,
                units_per_second: 5.8 + unit_hash(index, 29, 113) * 3.4,
                phase_distance,
            },
            NotShadowCaster,
        ));
    }

    for index in 0..16_usize {
        let angle = index as f32 * TAU / 16.0;
        let radius = 8.35;
        commands.spawn((
            Name::new(format!("Rain Chain {}", index.saturating_add(1))),
            Mesh3d(stream_mesh.clone()),
            MeshMaterial3d(palette.rain.clone()),
            Transform::from_xyz(
                angle.cos() * radius,
                (top_y + bottom_y) * 0.5,
                angle.sin() * radius,
            ),
            NotShadowCaster,
        ));
    }
}

fn spawn_ripples(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let ripple_mesh = meshes.add(Torus::new(0.18, 0.24));

    for index in 0..24_usize {
        let angle = index as f32 * 2.399_963_1;
        let radius = 1.4 + unit_hash(index, 37, 97) * 9.2;
        let scale = 0.25 + unit_hash(index, 23, 89) * 0.45;
        commands.spawn((
            Name::new(format!("Pool Ripple {}", index.saturating_add(1))),
            Mesh3d(ripple_mesh.clone()),
            MeshMaterial3d(palette.rain.clone()),
            Transform::from_xyz(angle.cos() * radius, WATER_Y + 0.12, angle.sin() * radius)
                .with_scale(Vec3::splat(scale)),
            Ripple {
                minimum_scale: 0.2,
                maximum_scale: 1.5 + unit_hash(index, 19, 83),
                cycles_per_second: 0.28 + unit_hash(index, 31, 79) * 0.18,
                phase: unit_hash(index, 47, 101),
            },
            NotShadowCaster,
        ));
    }
}

fn spawn_rain_beds(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let bed_mesh = meshes.add(Cuboid::new(10.0, 0.48, 7.6));
    let bed_water_mesh = meshes.add(Cuboid::new(9.25, 0.08, 6.85));
    let reed_mesh = meshes.add(Cylinder::new(0.065, 1.0));
    let bud_mesh = meshes.add(Sphere::new(0.18));
    let leaf_mesh = meshes.add(Sphere::new(0.65));
    let centers = [
        Vec3::new(-19.0, 0.0, -18.0),
        Vec3::new(19.0, 0.0, -18.0),
        Vec3::new(-19.0, 0.0, 18.0),
        Vec3::new(19.0, 0.0, 18.0),
    ];

    for (bed_index, center) in centers.into_iter().enumerate() {
        commands.spawn((
            Name::new(format!("Rain Bed {}", bed_index.saturating_add(1))),
            Mesh3d(bed_mesh.clone()),
            MeshMaterial3d(palette.dark_stone.clone()),
            Transform::from_translation(center + Vec3::Y * 0.3),
        ));
        commands.spawn((
            Name::new(format!("Rain Bed {} Water", bed_index.saturating_add(1))),
            Mesh3d(bed_water_mesh.clone()),
            MeshMaterial3d(palette.water.clone()),
            Transform::from_translation(center + Vec3::Y * 0.56),
            NotShadowCaster,
        ));

        for reed_index in 0..11_usize {
            let seed = bed_index.saturating_mul(17).saturating_add(reed_index);
            let x = (unit_hash(seed, 31, 101) - 0.5) * 8.0;
            let z = (unit_hash(seed, 47, 103) - 0.5) * 5.6;
            let height = 1.3 + unit_hash(seed, 59, 107) * 2.7;
            let reed_position = center + Vec3::new(x, 0.58 + height * 0.5, z);

            commands.spawn((
                Name::new(format!(
                    "Rain Bed {} Reed {}",
                    bed_index.saturating_add(1),
                    reed_index.saturating_add(1)
                )),
                Mesh3d(reed_mesh.clone()),
                MeshMaterial3d(palette.verdigris.clone()),
                Transform::from_translation(reed_position).with_scale(Vec3::new(1.0, height, 1.0)),
            ));
            commands.spawn((
                Name::new("Rain Reed Light"),
                Mesh3d(bud_mesh.clone()),
                MeshMaterial3d(if reed_index % 3 == 0 {
                    palette.amber_glow.clone()
                } else {
                    palette.cyan_glow.clone()
                }),
                Transform::from_translation(center + Vec3::new(x, 0.64 + height, z)),
                NotShadowCaster,
            ));

            if reed_index % 2 == 0 {
                commands.spawn((
                    Name::new("Rain Reed Leaf"),
                    Mesh3d(leaf_mesh.clone()),
                    MeshMaterial3d(palette.foliage.clone()),
                    Transform::from_translation(center + Vec3::new(x + 0.35, 0.9, z))
                        .with_scale(Vec3::new(1.2, 0.12, 0.58))
                        .with_rotation(Quat::from_rotation_y(seed as f32)),
                ));
            }
        }
    }
}

fn spawn_lanterns(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let base_mesh = meshes.add(Cylinder::new(0.85, 0.5));
    let post_mesh = meshes.add(Cylinder::new(0.22, 3.6));
    let roof_mesh = meshes.add(Cone::new(1.15, 0.9));
    let lamp_mesh = meshes.add(Sphere::new(0.42));
    let positions = [
        Vec3::new(-8.2, 0.0, 17.0),
        Vec3::new(8.2, 0.0, 17.0),
        Vec3::new(-8.2, 0.0, -17.0),
        Vec3::new(8.2, 0.0, -17.0),
    ];

    for (index, position) in positions.into_iter().enumerate() {
        commands.spawn((
            Name::new(format!("Garden Lantern {} Base", index.saturating_add(1))),
            Mesh3d(base_mesh.clone()),
            MeshMaterial3d(palette.stone.clone()),
            Transform::from_translation(position + Vec3::Y * 0.25),
        ));
        commands.spawn((
            Name::new(format!("Garden Lantern {} Post", index.saturating_add(1))),
            Mesh3d(post_mesh.clone()),
            MeshMaterial3d(palette.verdigris.clone()),
            Transform::from_translation(position + Vec3::Y * 2.05),
        ));
        commands.spawn((
            Name::new(format!("Garden Lantern {} Lamp", index.saturating_add(1))),
            Mesh3d(lamp_mesh.clone()),
            MeshMaterial3d(palette.amber_glow.clone()),
            Transform::from_translation(position + Vec3::Y * 4.05),
            NotShadowCaster,
        ));
        commands.spawn((
            Name::new(format!("Garden Lantern {} Roof", index.saturating_add(1))),
            Mesh3d(roof_mesh.clone()),
            MeshMaterial3d(palette.brass.clone()),
            Transform::from_translation(position + Vec3::Y * 4.62),
        ));
        commands.spawn((
            Name::new(format!("Garden Lantern {} Light", index.saturating_add(1))),
            PointLight {
                intensity: 2_200.0,
                range: 14.0,
                radius: 0.7,
                color: Color::srgb(1.0, 0.48, 0.12),
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_translation(position + Vec3::Y * 4.05),
            PulseLight {
                base_intensity: 1_700.0,
                amplitude: 850.0,
                speed: 1.1 + index as f32 * 0.13,
                phase: index as f32 * 1.4,
            },
        ));
    }

    let mote_mesh = meshes.add(Sphere::new(0.11));
    for index in 0..36_usize {
        let angle = index as f32 * 2.399_963_1;
        let radius = 14.0 + unit_hash(index, 41, 109) * 14.0;
        let anchor = Vec3::new(
            angle.cos() * radius,
            2.0 + unit_hash(index, 67, 113) * 5.0,
            angle.sin() * radius,
        );
        commands.spawn((
            Name::new(format!("Lantern Mote {}", index.saturating_add(1))),
            Mesh3d(mote_mesh.clone()),
            MeshMaterial3d(if index % 5 == 0 {
                palette.amber_glow.clone()
            } else {
                palette.cyan_glow.clone()
            }),
            Transform::from_translation(anchor),
            Drift {
                anchor,
                horizontal_radius: Vec2::new(
                    0.6 + unit_hash(index, 17, 73),
                    0.6 + unit_hash(index, 29, 79),
                ),
                vertical_amplitude: 0.4 + unit_hash(index, 53, 83) * 0.8,
                speed: 0.24 + unit_hash(index, 61, 89) * 0.34,
                phase: angle,
            },
            NotShadowCaster,
        ));
    }
}

fn spawn_colonnade(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let column_mesh = meshes.add(Cylinder::new(0.52, 5.6));
    let capital_mesh = meshes.add(Cuboid::new(1.35, 0.38, 1.35));
    let orb_mesh = meshes.add(Sphere::new(0.22));

    commands.spawn((
        Name::new("Colonnade Ring"),
        Mesh3d(meshes.add(Torus::new(30.4, 31.1))),
        MeshMaterial3d(palette.dark_stone.clone()),
        Transform::from_xyz(0.0, 6.15, 0.0),
    ));
    commands.spawn((
        Name::new("Colonnade Brass Inlay"),
        Mesh3d(meshes.add(Torus::new(30.65, 30.82))),
        MeshMaterial3d(palette.brass.clone()),
        Transform::from_xyz(0.0, 6.53, 0.0),
    ));

    for index in 0..16_usize {
        let angle = index as f32 * TAU / 16.0;
        let position = Vec3::new(angle.cos() * 30.75, 0.0, angle.sin() * 30.75);
        commands.spawn((
            Name::new(format!("Colonnade Pillar {}", index.saturating_add(1))),
            Mesh3d(column_mesh.clone()),
            MeshMaterial3d(palette.stone.clone()),
            Transform::from_translation(position + Vec3::Y * 3.0),
            RigidBody::Static,
            Collider::cylinder(0.52, 5.6),
        ));
        commands.spawn((
            Name::new(format!("Colonnade Capital {}", index.saturating_add(1))),
            Mesh3d(capital_mesh.clone()),
            MeshMaterial3d(palette.verdigris.clone()),
            Transform::from_translation(position + Vec3::Y * 5.9)
                .with_rotation(Quat::from_rotation_y(-angle)),
        ));

        if index % 2 == 1 {
            commands.spawn((
                Name::new(format!("Colonnade Gem {}", index.saturating_add(1))),
                Mesh3d(orb_mesh.clone()),
                MeshMaterial3d(palette.cyan_glow.clone()),
                Transform::from_translation(position + Vec3::Y * 6.55),
                NotShadowCaster,
            ));
        }
    }
}

fn spawn_edge_waterfalls(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let fall_mesh = meshes.add(Cuboid::new(3.2, 13.0, 0.12));
    let mist_mesh = meshes.add(Sphere::new(1.0));

    for index in 0..10_usize {
        let angle = index as f32 * TAU / 10.0;
        let direction = Vec3::new(angle.sin(), 0.0, angle.cos());
        let position = direction * 34.6;
        commands.spawn((
            Name::new(format!("Edge Waterfall {}", index.saturating_add(1))),
            Mesh3d(fall_mesh.clone()),
            MeshMaterial3d(palette.water.clone()),
            Transform::from_translation(position - Vec3::Y * 6.2)
                .with_rotation(Quat::from_rotation_y(angle)),
            NotShadowCaster,
        ));

        let mist_anchor = position - Vec3::Y * 12.0;
        commands.spawn((
            Name::new(format!("Waterfall Mist {}", index.saturating_add(1))),
            Mesh3d(mist_mesh.clone()),
            MeshMaterial3d(palette.mist.clone()),
            Transform::from_translation(mist_anchor).with_scale(Vec3::new(5.5, 1.3, 3.2)),
            Drift {
                anchor: mist_anchor,
                horizontal_radius: Vec2::new(1.2, 0.8),
                vertical_amplitude: 0.35,
                speed: 0.08 + index as f32 * 0.007,
                phase: angle,
            },
            NotShadowCaster,
        ));
    }
}

fn spawn_moon_and_stars(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let moon_position = Vec3::new(-31.0, 34.0, -68.0);
    commands.spawn((
        Name::new("Garden Moon Halo"),
        Mesh3d(meshes.add(Sphere::new(9.0))),
        MeshMaterial3d(palette.moon_halo.clone()),
        Transform::from_translation(moon_position),
        NotShadowCaster,
    ));
    commands.spawn((
        Name::new("Garden Moon"),
        Mesh3d(meshes.add(Sphere::new(6.2))),
        MeshMaterial3d(palette.moon.clone()),
        Transform::from_translation(moon_position + Vec3::new(0.0, 0.0, 0.5)),
        NotShadowCaster,
    ));

    let star_mesh = meshes.add(Sphere::new(0.14));
    for index in 0..180_usize {
        let x = (unit_hash(index, 47, 181) - 0.5) * 150.0;
        let y = 17.0 + unit_hash(index, 83, 179) * 58.0;
        let z = -48.0 - unit_hash(index, 101, 173) * 72.0;
        let scale = 0.45 + unit_hash(index, 61, 167) * 1.3;
        commands.spawn((
            Name::new(format!("Garden Star {}", index.saturating_add(1))),
            Mesh3d(star_mesh.clone()),
            MeshMaterial3d(palette.star.clone()),
            Transform::from_xyz(x, y, z).with_scale(Vec3::splat(scale)),
            NotShadowCaster,
        ));
    }
}

fn unit_hash(index: usize, multiplier: usize, modulus: usize) -> f32 {
    let denominator = modulus.saturating_sub(1).max(1);
    (index.saturating_mul(multiplier) % modulus) as f32 / denominator as f32
}
