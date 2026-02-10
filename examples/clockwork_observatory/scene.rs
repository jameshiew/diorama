use std::f32::consts::{PI, TAU};

use avian3d::prelude::*;
use bevy::prelude::*;
use diorama::player::Player;

use crate::animation::{BobbingLantern, OrbitingBody, PulsingLight, RingSpin};

const OBSERVATORY_CLEAR_COLOR: Color = Color::srgb(0.015, 0.02, 0.05);
const OBSERVATORY_RADIUS: f32 = 22.0;
const OBSERVATORY_THICKNESS: f32 = 1.6;
const PLAYER_SPAWN: Vec3 = Vec3::new(-8.0, 4.0, -12.0);
const ORRERY_CENTER: Vec3 = Vec3::new(0.0, 10.5, 0.0);

struct MaterialPalette {
    obsidian: Handle<StandardMaterial>,
    brass: Handle<StandardMaterial>,
    walkway: Handle<StandardMaterial>,
    orb: Handle<StandardMaterial>,
    lantern: Handle<StandardMaterial>,
    stars: Handle<StandardMaterial>,
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(ClearColor(OBSERVATORY_CLEAR_COLOR));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.2, 0.26, 0.4),
        brightness: 450.0,
        affects_lightmapped_meshes: true,
    });

    let palette = create_material_palette(&mut materials);

    spawn_lighting(&mut commands);
    spawn_main_platform(&mut commands, &mut meshes, &palette);
    spawn_radial_bridges_and_pods(&mut commands, &mut meshes, &palette);
    spawn_central_tower(&mut commands, &mut meshes, &palette);
    spawn_orrery(&mut commands, &mut meshes, &palette);
    spawn_lantern_swarm(&mut commands, &mut meshes, &palette);
    spawn_star_canopy(&mut commands, &mut meshes, &palette);
}

pub fn spawn_player(mut player: Single<&mut Transform, With<Player>>) {
    let spawn = Transform::from_translation(PLAYER_SPAWN).looking_at(ORRERY_CENTER, Vec3::Y);
    player.translation = spawn.translation;
    player.rotation = spawn.rotation;
}

fn create_material_palette(materials: &mut ResMut<Assets<StandardMaterial>>) -> MaterialPalette {
    MaterialPalette {
        obsidian: materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.09, 0.12),
            metallic: 0.2,
            perceptual_roughness: 0.75,
            ..default()
        }),
        brass: materials.add(StandardMaterial {
            base_color: Color::srgb(0.77, 0.64, 0.3),
            metallic: 0.85,
            perceptual_roughness: 0.2,
            reflectance: 0.8,
            ..default()
        }),
        walkway: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.24, 0.32),
            metallic: 0.25,
            perceptual_roughness: 0.55,
            ..default()
        }),
        orb: materials.add(StandardMaterial {
            base_color: Color::srgb(0.7, 0.82, 0.96),
            emissive: Color::srgb(0.2, 0.3, 0.55).into(),
            metallic: 0.65,
            perceptual_roughness: 0.15,
            ..default()
        }),
        lantern: materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.75, 0.4),
            emissive: Color::srgb(0.45, 0.25, 0.1).into(),
            unlit: true,
            ..default()
        }),
        stars: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: Color::srgb(0.9, 0.95, 1.0).into(),
            unlit: true,
            ..default()
        }),
    }
}

fn spawn_lighting(commands: &mut Commands) {
    commands.spawn((
        Name::new("Celestial Directional Light"),
        DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            color: Color::srgb(0.8, 0.86, 1.0),
            ..default()
        },
        Transform::from_xyz(24.0, 50.0, -16.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let beacon_positions = [
        Vec3::new(0.0, 8.0, 0.0),
        Vec3::new(18.0, 6.0, 0.0),
        Vec3::new(-18.0, 6.0, 0.0),
        Vec3::new(0.0, 6.0, 18.0),
        Vec3::new(0.0, 6.0, -18.0),
    ];

    for (index, position) in beacon_positions.into_iter().enumerate() {
        commands.spawn((
            Name::new(format!("Beacon {}", index.saturating_add(1))),
            PointLight {
                color: Color::srgb(0.65, 0.8, 1.0),
                intensity: 3200.0,
                range: 35.0,
                radius: 0.8,
                shadows_enabled: index == 0,
                ..default()
            },
            Transform::from_translation(position),
            PulsingLight {
                base_intensity: 2200.0,
                amplitude: 1900.0,
                speed: 0.6 + index as f32 * 0.15,
                phase: index as f32 * 0.8,
            },
        ));
    }
}

fn spawn_main_platform(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    palette: &MaterialPalette,
) {
    commands.spawn((
        Name::new("Observatory Deck"),
        Mesh3d(meshes.add(Cylinder::new(OBSERVATORY_RADIUS, OBSERVATORY_THICKNESS))),
        MeshMaterial3d(palette.obsidian.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cylinder(OBSERVATORY_RADIUS, OBSERVATORY_THICKNESS),
    ));

    commands.spawn((
        Name::new("Deck Inset Ring"),
        Mesh3d(meshes.add(Torus::new(
            OBSERVATORY_RADIUS - 2.2,
            OBSERVATORY_RADIUS - 1.8,
        ))),
        MeshMaterial3d(palette.brass.clone()),
        Transform::from_xyz(0.0, OBSERVATORY_THICKNESS * 0.5 + 0.05, 0.0),
    ));
}

fn spawn_radial_bridges_and_pods(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    palette: &MaterialPalette,
) {
    const BRIDGE_LENGTH: f32 = 11.0;
    const BRIDGE_WIDTH: f32 = 3.0;
    const BRIDGE_HEIGHT: f32 = 0.6;
    const BRIDGE_OFFSET: f32 = 12.5;
    const POD_RADIUS: f32 = 4.4;
    const POD_HEIGHT: f32 = 0.8;
    const POD_OFFSET: f32 = 21.0;
    const POD_LEVEL_Y: f32 = 1.4;

    for (index, angle) in [0.0, PI * 0.5, PI, PI * 1.5].into_iter().enumerate() {
        let forward = Vec3::new(angle.cos(), 0.0, angle.sin());
        let bridge_center = forward * BRIDGE_OFFSET + Vec3::Y * POD_LEVEL_Y;
        let pod_center = forward * POD_OFFSET + Vec3::Y * POD_LEVEL_Y;

        commands.spawn((
            Name::new(format!("Bridge {}", index.saturating_add(1))),
            Mesh3d(meshes.add(Cuboid::new(BRIDGE_LENGTH, BRIDGE_HEIGHT, BRIDGE_WIDTH))),
            MeshMaterial3d(palette.walkway.clone()),
            Transform::from_translation(bridge_center).with_rotation(Quat::from_rotation_y(angle)),
            RigidBody::Static,
            Collider::cuboid(BRIDGE_LENGTH, BRIDGE_HEIGHT, BRIDGE_WIDTH),
        ));

        commands.spawn((
            Name::new(format!("Viewing Pod {}", index.saturating_add(1))),
            Mesh3d(meshes.add(Cylinder::new(POD_RADIUS, POD_HEIGHT))),
            MeshMaterial3d(palette.walkway.clone()),
            Transform::from_translation(pod_center),
            RigidBody::Static,
            Collider::cylinder(POD_RADIUS, POD_HEIGHT),
        ));

        commands.spawn((
            Name::new(format!("Pod Support {}", index.saturating_add(1))),
            Mesh3d(meshes.add(Cylinder::new(1.3, 6.0))),
            MeshMaterial3d(palette.obsidian.clone()),
            Transform::from_translation(pod_center - Vec3::Y * 3.8),
        ));

        commands.spawn((
            Name::new(format!("Pod Telescope {}", index.saturating_add(1))),
            Mesh3d(meshes.add(Capsule3d::new(0.35, 2.2))),
            MeshMaterial3d(palette.brass.clone()),
            Transform::from_translation(pod_center + Vec3::new(0.0, 1.4, 0.0))
                .with_rotation(Quat::from_rotation_x(PI * 0.33)),
        ));
    }
}

fn spawn_central_tower(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    palette: &MaterialPalette,
) {
    commands.spawn((
        Name::new("Central Tower"),
        Mesh3d(meshes.add(Cylinder::new(5.5, 12.0))),
        MeshMaterial3d(palette.obsidian.clone()),
        Transform::from_xyz(0.0, 6.0, 0.0),
        RigidBody::Static,
        Collider::cylinder(5.5, 12.0),
    ));

    commands.spawn((
        Name::new("Tower Crown Platform"),
        Mesh3d(meshes.add(Cylinder::new(7.0, 0.9))),
        MeshMaterial3d(palette.walkway.clone()),
        Transform::from_xyz(0.0, 12.45, 0.0),
        RigidBody::Static,
        Collider::cylinder(7.0, 0.9),
    ));
}

fn spawn_orrery(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    palette: &MaterialPalette,
) {
    commands.spawn((
        Name::new("Orrery Core"),
        Mesh3d(meshes.add(Sphere::new(1.1))),
        MeshMaterial3d(palette.orb.clone()),
        Transform::from_translation(ORRERY_CENTER),
        PointLight {
            color: Color::srgb(0.75, 0.9, 1.0),
            intensity: 4500.0,
            range: 20.0,
            radius: 1.0,
            shadows_enabled: true,
            ..default()
        },
        PulsingLight {
            base_intensity: 3000.0,
            amplitude: 2000.0,
            speed: 0.9,
            phase: 0.0,
        },
    ));

    let ring_specs = [
        (4.0, Vec3::Y, 0.18),
        (5.6, Vec3::new(0.8, 0.4, 0.3), -0.14),
        (7.2, Vec3::new(0.2, 0.7, 0.9), 0.1),
    ];

    for (index, (radius, axis, speed)) in ring_specs.into_iter().enumerate() {
        commands.spawn((
            Name::new(format!("Orrery Ring {}", index.saturating_add(1))),
            Mesh3d(meshes.add(Torus::new(radius - 0.14, radius + 0.14))),
            MeshMaterial3d(palette.brass.clone()),
            Transform::from_translation(ORRERY_CENTER),
            RingSpin { axis, speed },
        ));
    }

    let orbital_specs = [
        (3.4, 0.55, 0.25, 0.9, 0.1),
        (4.8, -0.4, 0.35, 0.75, 1.4),
        (6.2, 0.3, 0.45, 0.62, 2.8),
        (8.0, -0.22, 0.6, 0.52, 3.7),
    ];

    for (index, (radius, angular_speed, vertical_amplitude, scale, phase)) in
        orbital_specs.into_iter().enumerate()
    {
        commands.spawn((
            Name::new(format!("Orbital Body {}", index.saturating_add(1))),
            Mesh3d(meshes.add(Sphere::new(scale))),
            MeshMaterial3d(palette.orb.clone()),
            Transform::from_translation(ORRERY_CENTER + Vec3::new(radius, 0.0, 0.0)),
            OrbitingBody {
                center: ORRERY_CENTER,
                radius,
                angular_speed,
                vertical_amplitude,
                vertical_speed: 0.8 + index as f32 * 0.2,
                phase,
            },
            PointLight {
                color: Color::srgb(0.7, 0.88, 1.0),
                intensity: 1700.0,
                range: 10.0,
                radius: 0.4,
                shadows_enabled: false,
                ..default()
            },
            PulsingLight {
                base_intensity: 1200.0,
                amplitude: 900.0,
                speed: 1.0 + index as f32 * 0.25,
                phase: phase * 1.3,
            },
        ));
    }
}

fn spawn_lantern_swarm(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    palette: &MaterialPalette,
) {
    let lantern_mesh = meshes.add(Sphere::new(0.22));
    const LANTERN_COUNT: usize = 36;

    for index in 0..LANTERN_COUNT {
        let pct = index as f32 / LANTERN_COUNT as f32;
        let angle = pct * TAU * 2.0;
        let ring_radius = 27.0 + (index % 4) as f32 * 2.3;
        let height = 8.5 + (index % 6) as f32 * 1.3;
        let base_position = Vec3::new(angle.cos() * ring_radius, height, angle.sin() * ring_radius);

        commands.spawn((
            Name::new(format!("Lantern {}", index.saturating_add(1))),
            Mesh3d(lantern_mesh.clone()),
            MeshMaterial3d(palette.lantern.clone()),
            Transform::from_translation(base_position),
            BobbingLantern {
                base_position,
                vertical_amplitude: 0.35 + (index % 3) as f32 * 0.08,
                sway_amplitude: 0.15 + (index % 5) as f32 * 0.03,
                speed: 0.5 + (index % 7) as f32 * 0.06,
                phase: pct * TAU * 3.0,
            },
            PointLight {
                color: Color::srgb(1.0, 0.74, 0.46),
                intensity: 520.0,
                range: 7.5,
                radius: 0.25,
                shadows_enabled: false,
                ..default()
            },
            PulsingLight {
                base_intensity: 380.0,
                amplitude: 220.0,
                speed: 1.2 + (index % 5) as f32 * 0.1,
                phase: pct * TAU * 2.0,
            },
        ));
    }
}

fn spawn_star_canopy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    palette: &MaterialPalette,
) {
    let star_mesh = meshes.add(Sphere::new(0.08));
    const STAR_COUNT: usize = 900;

    for index in 0..STAR_COUNT {
        let fraction = index as f32 / STAR_COUNT as f32;
        let y = 0.35 + fraction * 0.65;
        let radial = (1.0 - y * y).sqrt();
        let golden_angle = 2.399_963_1_f32;
        let theta = index as f32 * golden_angle;
        let x = theta.cos() * radial;
        let z = theta.sin() * radial;
        let direction = Vec3::new(x, y, z).normalize_or_zero();
        let distance = 120.0 + (index % 9) as f32 * 6.0;

        commands.spawn((
            Name::new("Star"),
            Mesh3d(star_mesh.clone()),
            MeshMaterial3d(palette.stars.clone()),
            Transform::from_translation(direction * distance),
        ));
    }
}
