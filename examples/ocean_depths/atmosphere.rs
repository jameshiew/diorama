//! Underwater atmosphere and lighting effects
//!
//! Creates the underwater ambiance through:
//! - Shader-based caustics on the seafloor
//! - Underwater fog color
//! - Particle bubbles rising
//! - Floating plankton and organic matter
//! - Sand particles near the floor
//! - Animated god rays

use bevy::math::Vec4;
use bevy::prelude::*;

use crate::materials::{CausticsData, CausticsMaterial};

pub struct AtmospherePlugin;

impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (setup_atmosphere, spawn_caustics_planes, spawn_particles),
        )
        .add_systems(
            Update,
            (
                animate_caustics_light,
                animate_bubbles,
                animate_plankton,
                animate_sand_particles,
                animate_god_rays,
            ),
        );
    }
}

/// Main underwater light with caustics animation
#[derive(Component)]
pub struct CausticsLight {
    pub base_intensity: f32,
    pub phase: f32,
}

/// Rising bubble particle
#[derive(Component)]
pub struct Bubble {
    pub speed: f32,
    pub wobble_phase: f32,
    pub start_x: f32,
    pub start_z: f32,
}

/// Floating plankton/organic particle
#[derive(Component)]
pub struct Plankton {
    pub drift_phase: f32,
    pub drift_speed: f32,
    pub base_pos: Vec3,
}

/// Sand particle near seafloor
#[derive(Component)]
pub struct SandParticle {
    pub settle_speed: f32,
    pub drift_phase: f32,
    pub base_pos: Vec3,
}

/// God ray light shaft
#[derive(Component)]
pub struct GodRay {
    pub phase: f32,
    pub sway_speed: f32,
}

fn setup_atmosphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Deep blue underwater background
    commands.insert_resource(ClearColor(Color::srgb(0.02, 0.15, 0.3)));

    // Ambient underwater light - bright enough to see the ocean floor
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.4, 0.6, 0.8),
        brightness: 800.0,
        affects_lightmapped_meshes: true,
    });

    // Main directional "sun" light filtering through water
    commands.spawn((
        Name::new("Underwater Sun"),
        DirectionalLight {
            color: Color::srgb(0.6, 0.85, 1.0),
            illuminance: 25000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.3, 0.0)),
    ));

    // Animated caustics lights (multiple point lights simulating light refraction)
    let caustic_positions = [
        Vec3::new(0.0, 15.0, 0.0),
        Vec3::new(15.0, 12.0, 10.0),
        Vec3::new(-15.0, 14.0, -10.0),
        Vec3::new(10.0, 13.0, -15.0),
        Vec3::new(-10.0, 11.0, 15.0),
        Vec3::new(25.0, 10.0, 20.0),
        Vec3::new(-25.0, 12.0, -20.0),
    ];

    for (i, pos) in caustic_positions.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Caustic Light {i}")),
            PointLight {
                color: Color::srgb(0.5, 0.8, 1.0),
                intensity: 50000.0,
                radius: 30.0,
                shadows_enabled: false,
                ..default()
            },
            Transform::from_translation(*pos),
            CausticsLight {
                base_intensity: 50000.0,
                phase: i as f32 * 1.2,
            },
        ));
    }

    // Spawn bubble particles
    let bubble_mesh = meshes.add(Sphere::new(0.08));
    let bubble_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.8, 0.9, 1.0, 0.4),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    for _ in 0..80 {
        let x = (rand::random::<f32>() - 0.5) * 100.0;
        let z = (rand::random::<f32>() - 0.5) * 100.0;
        let y = rand::random::<f32>() * 20.0 - 5.0;

        commands.spawn((
            Mesh3d(bubble_mesh.clone()),
            MeshMaterial3d(bubble_material.clone()),
            Transform::from_xyz(x, y, z).with_scale(Vec3::splat(0.3 + rand::random::<f32>() * 1.2)),
            Bubble {
                speed: 0.8 + rand::random::<f32>() * 2.5,
                wobble_phase: rand::random::<f32>() * std::f32::consts::TAU,
                start_x: x,
                start_z: z,
            },
            Name::new("Bubble"),
        ));
    }

    // Spawn underwater "god rays" as semi-transparent animated shafts
    spawn_god_rays(&mut commands, &mut meshes, &mut materials);
}

/// Spawn caustics overlay planes using our custom shader
fn spawn_caustics_planes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut caustics_materials: ResMut<Assets<CausticsMaterial>>,
) {
    // Create a single large caustics plane that covers the entire seafloor (150x150)
    // Position it just above the seafloor terrain
    let plane_mesh = meshes.add(Plane3d::default().mesh().size(200.0, 200.0).build());

    let caustics_mat = caustics_materials.add(CausticsMaterial {
        data: CausticsData {
            color: Vec4::new(0.4, 0.65, 0.95, 1.0),
            speed: 0.8,
            _padding: 0,
        },
    });

    commands.spawn((
        Mesh3d(plane_mesh),
        MeshMaterial3d(caustics_mat),
        Transform::from_xyz(0.0, -3.5, 0.0),
        Name::new("Caustics Plane"),
    ));
}
/// Spawn various particle effects
fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plankton - tiny glowing organic particles
    let plankton_mesh = meshes.add(Sphere::new(0.03));
    let plankton_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.6, 0.9, 0.7, 0.6),
        emissive: Color::srgb(0.1, 0.2, 0.15).into(),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    for _ in 0..150 {
        let x = (rand::random::<f32>() - 0.5) * 100.0;
        let z = (rand::random::<f32>() - 0.5) * 100.0;
        let y = rand::random::<f32>() * 25.0 - 5.0;
        let base_pos = Vec3::new(x, y, z);

        commands.spawn((
            Mesh3d(plankton_mesh.clone()),
            MeshMaterial3d(plankton_material.clone()),
            Transform::from_translation(base_pos)
                .with_scale(Vec3::splat(0.5 + rand::random::<f32>() * 1.5)),
            Plankton {
                drift_phase: rand::random::<f32>() * std::f32::consts::TAU,
                drift_speed: 0.3 + rand::random::<f32>() * 0.5,
                base_pos,
            },
            Name::new("Plankton"),
        ));
    }

    // Bioluminescent plankton - brighter, rarer
    let biolum_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.3, 0.8, 1.0, 0.8),
        emissive: Color::srgb(0.2, 0.5, 0.6).into(),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    for _ in 0..30 {
        let x = (rand::random::<f32>() - 0.5) * 80.0;
        let z = (rand::random::<f32>() - 0.5) * 80.0;
        let y = rand::random::<f32>() * 20.0 - 3.0;
        let base_pos = Vec3::new(x, y, z);

        commands.spawn((
            Mesh3d(plankton_mesh.clone()),
            MeshMaterial3d(biolum_material.clone()),
            Transform::from_translation(base_pos)
                .with_scale(Vec3::splat(0.8 + rand::random::<f32>() * 1.0)),
            Plankton {
                drift_phase: rand::random::<f32>() * std::f32::consts::TAU,
                drift_speed: 0.2 + rand::random::<f32>() * 0.3,
                base_pos,
            },
            Name::new("Bioluminescent Plankton"),
        ));
    }

    // Sand particles near the seafloor
    let sand_mesh = meshes.add(Sphere::new(0.02));
    let sand_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.8, 0.75, 0.6, 0.5),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    for _ in 0..100 {
        let x = (rand::random::<f32>() - 0.5) * 120.0;
        let z = (rand::random::<f32>() - 0.5) * 120.0;
        let y = -4.0 + rand::random::<f32>() * 3.0; // Near seafloor
        let base_pos = Vec3::new(x, y, z);

        commands.spawn((
            Mesh3d(sand_mesh.clone()),
            MeshMaterial3d(sand_material.clone()),
            Transform::from_translation(base_pos)
                .with_scale(Vec3::splat(0.5 + rand::random::<f32>() * 1.0)),
            SandParticle {
                settle_speed: 0.1 + rand::random::<f32>() * 0.2,
                drift_phase: rand::random::<f32>() * std::f32::consts::TAU,
                base_pos,
            },
            Name::new("Sand Particle"),
        ));
    }
}

fn spawn_god_rays(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let ray_mesh = meshes.add(Cylinder::new(2.5, 35.0));
    let ray_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.6, 0.8, 1.0, 0.04),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        double_sided: true,
        ..default()
    });

    let ray_positions = [
        (Vec3::new(5.0, 12.0, 5.0), 0.0),
        (Vec3::new(-10.0, 12.0, -8.0), 1.2),
        (Vec3::new(15.0, 12.0, -5.0), 2.4),
        (Vec3::new(-5.0, 12.0, 12.0), 3.6),
        (Vec3::new(20.0, 12.0, 15.0), 4.8),
        (Vec3::new(-20.0, 12.0, -15.0), 0.6),
        (Vec3::new(0.0, 12.0, -20.0), 1.8),
        (Vec3::new(25.0, 12.0, 0.0), 3.0),
    ];

    for (pos, phase) in ray_positions {
        commands.spawn((
            Mesh3d(ray_mesh.clone()),
            MeshMaterial3d(ray_material.clone()),
            Transform::from_translation(pos).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                0.15 + rand::random::<f32>() * 0.2,
                rand::random::<f32>() * 0.3,
                0.0,
            )),
            GodRay {
                phase,
                sway_speed: 0.3 + rand::random::<f32>() * 0.2,
            },
            Name::new("God Ray"),
        ));
    }
}

/// Animate caustics lights to simulate water surface refraction
fn animate_caustics_light(time: Res<Time>, mut query: Query<(&mut PointLight, &CausticsLight)>) {
    let t = time.elapsed_secs();

    for (mut light, caustics) in query.iter_mut() {
        // Multiple sine waves for organic-feeling variation
        let wave1 = (t * 2.0 + caustics.phase).sin();
        let wave2 = (t * 3.7 + caustics.phase * 1.5).sin();
        let wave3 = (t * 1.3 + caustics.phase * 0.7).sin();

        let intensity_mod = 0.6 + (wave1 * 0.2 + wave2 * 0.15 + wave3 * 0.1);
        light.intensity = caustics.base_intensity * intensity_mod;
    }
}

/// Animate bubbles rising and wobbling
fn animate_bubbles(time: Res<Time>, mut query: Query<(&mut Transform, &mut Bubble)>) {
    let dt = time.delta_secs();
    let t = time.elapsed_secs();

    for (mut transform, mut bubble) in query.iter_mut() {
        // Rise upward
        transform.translation.y += bubble.speed * dt;

        // Wobble horizontally
        transform.translation.x = bubble.start_x + (t + bubble.wobble_phase).sin() * 0.5;
        transform.translation.z = bubble.start_z + (t * 1.3 + bubble.wobble_phase).cos() * 0.5;

        // Reset bubble when it reaches the surface
        if transform.translation.y > 20.0 {
            transform.translation.y = -5.0;
            bubble.start_x = (rand::random::<f32>() - 0.5) * 100.0;
            bubble.start_z = (rand::random::<f32>() - 0.5) * 100.0;
            transform.translation.x = bubble.start_x;
            transform.translation.z = bubble.start_z;
        }
    }
}

/// Animate plankton with gentle drifting motion
fn animate_plankton(time: Res<Time>, mut query: Query<(&mut Transform, &Plankton)>) {
    let t = time.elapsed_secs();

    for (mut transform, plankton) in query.iter_mut() {
        let phase = plankton.drift_phase;
        let speed = plankton.drift_speed;

        // 3D Lissajous-like drifting pattern
        let x_offset = (t * speed + phase).sin() * 1.5;
        let y_offset = (t * speed * 0.7 + phase * 1.3).sin() * 0.8;
        let z_offset = (t * speed * 0.9 + phase * 0.7).cos() * 1.5;

        transform.translation = plankton.base_pos + Vec3::new(x_offset, y_offset, z_offset);

        // Gentle pulsing scale for bioluminescence effect
        let pulse = 0.9 + (t * 2.0 + phase).sin() * 0.1;
        let base_scale = transform.scale.x; // Preserve original scale ratio
        transform.scale = Vec3::splat(base_scale * pulse / (0.9 + 0.1)); // Normalize
    }
}

/// Animate sand particles with settling and drifting
fn animate_sand_particles(time: Res<Time>, mut query: Query<(&mut Transform, &SandParticle)>) {
    let t = time.elapsed_secs();

    for (mut transform, sand) in query.iter_mut() {
        let phase = sand.drift_phase;

        // Horizontal drift from underwater currents
        let x_offset = (t * 0.5 + phase).sin() * 2.0;
        let z_offset = (t * 0.3 + phase * 1.5).cos() * 2.0;

        // Occasional upward stirring, then settling back down
        let stir_cycle = (t * 0.2 + phase).sin();
        let y_offset = if stir_cycle > 0.7 {
            // Being stirred up
            (stir_cycle - 0.7) * 5.0
        } else {
            // Settling back down at settle_speed rate
            let settle_amount = (0.7 - stir_cycle) * sand.settle_speed;
            -settle_amount.min(0.5)
        };

        transform.translation = sand.base_pos + Vec3::new(x_offset, y_offset.max(-0.5), z_offset);
    }
}

/// Animate god rays with gentle swaying
fn animate_god_rays(time: Res<Time>, mut query: Query<(&mut Transform, &GodRay)>) {
    let t = time.elapsed_secs();

    for (mut transform, ray) in query.iter_mut() {
        // Gentle swaying motion
        let sway_x = (t * ray.sway_speed + ray.phase).sin() * 0.05;
        let sway_z = (t * ray.sway_speed * 0.7 + ray.phase * 1.3).cos() * 0.03;

        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.15 + sway_x, sway_z, 0.0);

        // Subtle intensity variation (through scale)
        let intensity = 0.9 + (t * 0.5 + ray.phase).sin() * 0.1;
        transform.scale.x = intensity;
        transform.scale.z = intensity;
    }
}
