use std::f32::consts::{FRAC_PI_2, PI, TAU};

use avian3d::prelude::*;
use bevy::light::NotShadowCaster;
use bevy::prelude::*;
use diorama::player::Player;

use crate::animation::{self, FloatingLantern, OrbitRing, Tide};
use crate::materials::TidalWaterMaterial;

const ORRERY_CENTER: Vec3 = Vec3::new(0.0, 7.6, -3.0);
const PLAYER_SPAWN: Vec3 = Vec3::new(0.0, 2.5, 27.0);

pub struct TidalOrreryPlugin;

impl Plugin for TidalOrreryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<TidalWaterMaterial>::default())
            .add_systems(Startup, (setup_scene, spawn_player).chain())
            .add_systems(
                Update,
                (
                    animation::animate_rings,
                    animation::animate_lanterns,
                    animation::animate_tide,
                ),
            );
    }
}

struct Palette {
    stone: Handle<StandardMaterial>,
    pale_stone: Handle<StandardMaterial>,
    brass: Handle<StandardMaterial>,
    patina: Handle<StandardMaterial>,
    amber: Handle<StandardMaterial>,
    pearl: Handle<StandardMaterial>,
    reed: Handle<StandardMaterial>,
    distant_rock: Handle<StandardMaterial>,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut water_materials: ResMut<Assets<TidalWaterMaterial>>,
) {
    commands.insert_resource(ClearColor(Color::srgb(0.025, 0.045, 0.085)));
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.48, 0.63, 0.78),
        brightness: 230.0,
        ..default()
    });

    let palette = Palette {
        stone: materials.add(StandardMaterial {
            base_color: Color::srgb(0.19, 0.28, 0.30),
            perceptual_roughness: 0.85,
            ..default()
        }),
        pale_stone: materials.add(StandardMaterial {
            base_color: Color::srgb(0.50, 0.57, 0.55),
            perceptual_roughness: 0.7,
            ..default()
        }),
        brass: materials.add(StandardMaterial {
            base_color: Color::srgb(0.72, 0.44, 0.16),
            metallic: 0.72,
            perceptual_roughness: 0.28,
            ..default()
        }),
        patina: materials.add(StandardMaterial {
            base_color: Color::srgb(0.07, 0.32, 0.29),
            metallic: 0.5,
            perceptual_roughness: 0.45,
            ..default()
        }),
        amber: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.65, 0.26),
            emissive: LinearRgba::new(3.5, 1.3, 0.25, 1.0),
            ..default()
        }),
        pearl: materials.add(StandardMaterial {
            base_color: Color::srgb(0.60, 0.91, 0.88),
            emissive: LinearRgba::new(0.32, 0.85, 0.78, 1.0),
            metallic: 0.25,
            perceptual_roughness: 0.24,
            ..default()
        }),
        reed: materials.add(Color::srgb(0.21, 0.34, 0.28)),
        distant_rock: materials.add(Color::srgb(0.09, 0.15, 0.21)),
    };

    commands.spawn((
        Name::new("Silver moonlight"),
        DirectionalLight {
            color: Color::srgb(0.67, 0.80, 1.0),
            illuminance: 5_500.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-25.0, 45.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Name::new("Orrery illumination"),
        PointLight {
            color: Color::srgb(0.33, 0.95, 0.83),
            intensity: 28_000.0,
            range: 24.0,
            radius: 1.5,
            ..default()
        },
        Transform::from_translation(ORRERY_CENTER),
    ));

    spawn_basin(&mut commands, &mut meshes, &mut water_materials, &palette);
    spawn_orrery(&mut commands, &mut meshes, &palette);
    spawn_colonnade(&mut commands, &mut meshes, &palette);
    spawn_lanterns(&mut commands, &mut meshes, &palette);
    spawn_shore(&mut commands, &mut meshes, &palette);
    spawn_sky(&mut commands, &mut meshes, &mut materials, &palette);
}

fn spawn_player(player: Single<&mut Transform, With<Player>>) {
    *player.into_inner() =
        Transform::from_translation(PLAYER_SPAWN).looking_at(Vec3::new(0.0, 6.2, -3.0), Vec3::Y);
}

fn spawn_basin(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    water_materials: &mut Assets<TidalWaterMaterial>,
    palette: &Palette,
) {
    commands.spawn((
        Name::new("Tidal basin floor"),
        Mesh3d(meshes.add(Cylinder::new(33.0, 2.0))),
        MeshMaterial3d(palette.stone.clone()),
        Transform::from_xyz(0.0, -1.4, 0.0),
        RigidBody::Static,
        Collider::cylinder(33.0, 2.0),
    ));
    commands.spawn((
        Name::new("Living tide"),
        Mesh3d(meshes.add(Cylinder::new(24.0, 0.08).mesh().resolution(128))),
        MeshMaterial3d(water_materials.add(TidalWaterMaterial {
            deep_color: LinearRgba::new(0.012, 0.075, 0.09, 1.0),
            crest_color: LinearRgba::new(0.13, 0.43, 0.40, 1.0),
        })),
        Transform::from_xyz(0.0, 0.05, 0.0),
        NotShadowCaster,
        Tide,
    ));
    commands.spawn((
        Name::new("Shoreline lip"),
        Mesh3d(meshes.add(Torus::new(24.0, 24.35))),
        MeshMaterial3d(palette.patina.clone()),
        Transform::from_xyz(0.0, -0.12, 0.0),
    ));
    commands.spawn((
        Name::new("Arrival terrace"),
        Mesh3d(meshes.add(Cylinder::new(5.0, 1.2))),
        MeshMaterial3d(palette.pale_stone.clone()),
        Transform::from_xyz(0.0, 0.05, 26.0),
        RigidBody::Static,
        Collider::cylinder(5.0, 1.2),
    ));

    let slab = meshes.add(Cuboid::new(3.2, 0.85, 2.8));
    let inlay = meshes.add(Cuboid::new(2.65, 0.025, 0.07));
    for index in 0..8 {
        let z = 22.0 - index as f32 * 3.1;
        commands.spawn((
            Name::new(format!("Causeway stone {index}")),
            Mesh3d(slab.clone()),
            MeshMaterial3d(palette.pale_stone.clone()),
            Transform::from_xyz(0.0, 0.2, z),
            RigidBody::Static,
            Collider::cuboid(3.2, 0.85, 2.8),
        ));
        commands.spawn((
            Mesh3d(inlay.clone()),
            MeshMaterial3d(palette.brass.clone()),
            Transform::from_xyz(0.0, 0.638, z + 1.0),
        ));
    }

    for (radius, height, y) in [(4.2, 1.2, 0.3), (3.6, 0.35, 1.075), (1.7, 1.3, 1.9)] {
        commands.spawn((
            Name::new("Orrery plinth"),
            Mesh3d(meshes.add(Cylinder::new(radius, height))),
            MeshMaterial3d(palette.pale_stone.clone()),
            Transform::from_xyz(0.0, y, -3.0),
            RigidBody::Static,
            Collider::cylinder(radius, height),
        ));
    }
    commands.spawn((
        Name::new("Patinated instrument cradle"),
        Mesh3d(meshes.add(Cone::new(1.9, 2.3))),
        MeshMaterial3d(palette.patina.clone()),
        Transform::from_xyz(0.0, 3.15, -3.0),
    ));
}

fn spawn_orrery(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let pearl = meshes.add(Sphere::new(1.3).mesh().uv(48, 32));
    let satellite = meshes.add(Sphere::new(0.28).mesh().uv(20, 12));
    let tick = meshes.add(Cuboid::new(0.055, 0.09, 0.3));
    commands.spawn((
        Name::new("Captive tidal moon"),
        Mesh3d(pearl),
        MeshMaterial3d(palette.pearl.clone()),
        Transform::from_translation(ORRERY_CENTER),
        NotShadowCaster,
    ));

    for (index, radius, tilt, speed) in [
        (0, 4.8, Quat::from_rotation_x(1.16), 0.12),
        (
            1,
            3.9,
            Quat::from_rotation_z(0.85) * Quat::from_rotation_x(0.5),
            -0.19,
        ),
        (
            2,
            2.9,
            Quat::from_rotation_z(-0.6) * Quat::from_rotation_x(1.5),
            0.27,
        ),
    ] {
        let ring_mesh = meshes.add(Torus::new(radius - 0.085, radius + 0.085));
        commands
            .spawn((
                Name::new(format!("Lunar orbit {index}")),
                Transform::from_translation(ORRERY_CENTER).with_rotation(tilt),
                Visibility::default(),
                OrbitRing { tilt, speed },
            ))
            .with_children(|parent| {
                parent.spawn((Mesh3d(ring_mesh), MeshMaterial3d(palette.brass.clone())));
                for marker in 0..32 {
                    let angle = marker as f32 * TAU / 32.0;
                    parent.spawn((
                        Mesh3d(tick.clone()),
                        MeshMaterial3d(palette.brass.clone()),
                        Transform::from_xyz(radius * angle.sin(), 0.0, radius * angle.cos())
                            .with_rotation(Quat::from_rotation_y(angle)),
                    ));
                }
                for (angle, scale) in [(0.0_f32, 1.5), (PI, 0.8)] {
                    parent.spawn((
                        Name::new("Orbiting amber satellite"),
                        Mesh3d(satellite.clone()),
                        MeshMaterial3d(palette.amber.clone()),
                        Transform::from_xyz(radius * angle.cos(), 0.0, radius * angle.sin())
                            .with_scale(Vec3::splat(scale)),
                        NotShadowCaster,
                    ));
                }
            });
    }

    commands.spawn((
        Name::new("Meridian support"),
        Mesh3d(meshes.add(Torus::new(5.4, 5.65))),
        MeshMaterial3d(palette.patina.clone()),
        Transform::from_translation(ORRERY_CENTER).with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
    ));
    for side in [-1.0, 1.0] {
        commands.spawn((
            Name::new("Meridian footing"),
            Mesh3d(meshes.add(Cylinder::new(0.36, 4.3))),
            MeshMaterial3d(palette.brass.clone()),
            Transform::from_xyz(side * 4.0, 2.5, -3.0),
        ));
    }
}

fn spawn_colonnade(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let base = meshes.add(Cylinder::new(1.25, 0.5));
    let crown = meshes.add(Cuboid::new(1.9, 0.38, 1.9));
    for index in 0..10 {
        let angle = PI + index as f32 * PI / 9.0;
        let position = Vec3::new(angle.cos() * 18.0, 0.0, angle.sin() * 18.0 - 1.0);
        let height = 4.8 + (index as f32 * 1.7).sin().abs() * 3.0;
        commands.spawn((
            Name::new(format!("Tidal column {index}")),
            Mesh3d(meshes.add(Cylinder::new(0.68, height))),
            MeshMaterial3d(palette.pale_stone.clone()),
            Transform::from_translation(position + Vec3::Y * (height * 0.5)),
            RigidBody::Static,
            Collider::cylinder(0.68, height),
        ));
        commands.spawn((
            Mesh3d(base.clone()),
            MeshMaterial3d(palette.stone.clone()),
            Transform::from_translation(position),
        ));
        commands.spawn((
            Mesh3d(crown.clone()),
            MeshMaterial3d(palette.pale_stone.clone()),
            Transform::from_translation(position + Vec3::Y * height)
                .with_rotation(Quat::from_rotation_y(-angle)),
        ));
        commands.spawn((
            Mesh3d(meshes.add(Torus::new(0.68, 0.76))),
            MeshMaterial3d(palette.brass.clone()),
            Transform::from_translation(position + Vec3::Y * (height - 0.55)),
        ));
    }
}

fn spawn_lanterns(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let float = meshes.add(Cylinder::new(0.62, 0.16));
    let glass = meshes.add(Cuboid::new(0.46, 0.62, 0.46));
    let roof = meshes.add(Cone::new(0.51, 0.38));
    let post = meshes.add(Cuboid::new(0.055, 0.8, 0.055));
    for index in 0..14 {
        let angle = index as f32 * TAU / 14.0 + 0.18;
        let radius = 10.0 + (index as f32 * 2.1).sin() * 2.0;
        let mut anchor = Vec3::new(angle.cos() * radius, 0.13, angle.sin() * radius);
        if anchor.x.abs() < 2.8 && anchor.z > 0.0 {
            anchor.x = anchor.x.signum() * 3.4;
        }
        commands
            .spawn((
                Name::new(format!("Floating lantern {index}")),
                Transform::from_translation(anchor),
                Visibility::default(),
                FloatingLantern {
                    anchor,
                    phase: angle * 3.0,
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Mesh3d(float.clone()),
                    MeshMaterial3d(palette.patina.clone()),
                ));
                parent.spawn((
                    Mesh3d(glass.clone()),
                    MeshMaterial3d(palette.amber.clone()),
                    Transform::from_xyz(0.0, 0.43, 0.0),
                    NotShadowCaster,
                ));
                parent.spawn((
                    Mesh3d(roof.clone()),
                    MeshMaterial3d(palette.brass.clone()),
                    Transform::from_xyz(0.0, 0.98, 0.0),
                ));
                for x in [-0.27, 0.27] {
                    for z in [-0.27, 0.27] {
                        parent.spawn((
                            Mesh3d(post.clone()),
                            MeshMaterial3d(palette.brass.clone()),
                            Transform::from_xyz(x, 0.48, z),
                        ));
                    }
                }
                parent.spawn((
                    PointLight {
                        color: Color::srgb(1.0, 0.54, 0.18),
                        intensity: 1_400.0,
                        range: 5.0,
                        radius: 0.4,
                        ..default()
                    },
                    Transform::from_xyz(0.0, 0.6, 0.0),
                ));
            });
    }
}

fn spawn_shore(commands: &mut Commands, meshes: &mut Assets<Mesh>, palette: &Palette) {
    let reed = meshes.add(Cone::new(0.10, 1.8));
    let rock = meshes.add(Sphere::new(1.0).mesh().ico(1).unwrap());
    for index in 0..36 {
        let angle = index as f32 * TAU / 36.0;
        let radius = 25.7 + (angle * 7.0).sin();
        let anchor = Vec3::new(radius * angle.cos(), -0.15, radius * angle.sin());
        if anchor.x.abs() < 5.5 && anchor.z > 0.0 {
            continue;
        }
        commands.spawn((
            Name::new("Shore boulder"),
            Mesh3d(rock.clone()),
            MeshMaterial3d(palette.stone.clone()),
            Transform::from_translation(anchor)
                .with_scale(Vec3::new(1.5, 0.55, 0.9))
                .with_rotation(Quat::from_rotation_y(angle)),
        ));
        for blade in 0..5 {
            let phase = blade as f32 * 2.4 + angle;
            commands.spawn((
                Name::new("Salt reed"),
                Mesh3d(reed.clone()),
                MeshMaterial3d(palette.reed.clone()),
                Transform::from_translation(
                    anchor + Vec3::new(phase.cos() * 0.5, 0.7, phase.sin() * 0.5),
                )
                .with_rotation(Quat::from_rotation_z(phase.sin() * 0.22))
                .with_scale(Vec3::new(1.0, 0.8 + phase.cos().abs() * 0.5, 1.0)),
            ));
        }
    }
}

fn spawn_sky(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    palette: &Palette,
) {
    let moon_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.83, 0.89, 0.96),
        unlit: true,
        ..default()
    });
    commands.spawn((
        Name::new("Distant moon"),
        Mesh3d(meshes.add(Sphere::new(6.5).mesh().uv(48, 32))),
        MeshMaterial3d(moon_material.clone()),
        Transform::from_xyz(-34.0, 25.0, -65.0),
        NotShadowCaster,
    ));
    let star = meshes.add(Sphere::new(0.08).mesh().uv(8, 6));
    for index in 0..100 {
        let phase = index as f32 * 2.399_963;
        let height = 15.0 + (index as f32 * 0.71).sin().abs() * 65.0;
        commands.spawn((
            Name::new("Star"),
            Mesh3d(star.clone()),
            MeshMaterial3d(moon_material.clone()),
            Transform::from_xyz(phase.cos() * 110.0, height, phase.sin() * 110.0),
            NotShadowCaster,
        ));
    }
    let island = meshes.add(Sphere::new(1.0).mesh().ico(1).unwrap());
    for index in 0..16 {
        let angle = index as f32 * TAU / 16.0;
        let height = 4.0 + (angle * 3.0).sin().abs() * 8.0;
        commands.spawn((
            Name::new("Horizon island"),
            Mesh3d(island.clone()),
            MeshMaterial3d(palette.distant_rock.clone()),
            Transform::from_xyz(angle.cos() * 70.0, -3.0, angle.sin() * 70.0)
                .with_scale(Vec3::new(14.0, height, 9.0))
                .with_rotation(Quat::from_rotation_y(angle)),
        ));
    }
}
