//! Marine life simulation
//!
//! Features:
//! - Fish schools using boids algorithm
//! - Bioluminescent jellyfish with pulsing animation
//! - Sea turtles patrolling the reef
//! - Interactive dialogue with creatures

use bevy::math::Vec4;
use bevy::picking::events::{Click, Pointer};
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use diorama::picking::Hint;

use crate::dialogue::{OceanDialogue, start_dialogue};
use crate::materials::{
    FishScalesData, FishScalesMaterial, JellyfishData, JellyfishMaterial, TurtleShellData,
    TurtleShellMaterial,
};

pub struct CreaturesPlugin;

impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_fish_schools, spawn_jellyfish, spawn_turtle))
            .add_systems(
                Update,
                (
                    fish_boids,
                    animate_jellyfish,
                    patrol_turtle,
                    spawn_creature_bubbles,
                    animate_creature_bubbles,
                ),
            );
    }
}

/// Bubble trail from creatures
#[derive(Component)]
pub struct CreatureBubble {
    pub velocity: Vec3,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

// ============================================================================
// Fish Schools
// ============================================================================

#[derive(Component)]
pub struct Fish {
    pub velocity: Vec3,
    pub school_id: u32,
}

#[derive(Clone, Copy)]
struct FishSchoolConfig {
    color: Color,
    size: f32,
    count: u32,
    center: Vec3,
}

fn spawn_fish_schools(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FishScalesMaterial>>,
) {
    let schools = [
        FishSchoolConfig {
            color: Color::srgb(1.0, 0.6, 0.2), // Orange clownfish
            size: 0.3,
            count: 25,
            center: Vec3::new(10.0, 3.0, 5.0),
        },
        FishSchoolConfig {
            color: Color::srgb(0.2, 0.5, 1.0), // Blue tang
            size: 0.4,
            count: 20,
            center: Vec3::new(-15.0, 5.0, -10.0),
        },
        FishSchoolConfig {
            color: Color::srgb(1.0, 1.0, 0.3), // Yellow butterfly fish
            size: 0.25,
            count: 30,
            center: Vec3::new(0.0, 2.0, -20.0),
        },
    ];

    for (school_id, config) in schools.iter().enumerate() {
        let mesh = meshes.add(Mesh::from(Triangle3d::new(
            Vec3::new(0.0, 0.0, config.size),
            Vec3::new(-config.size * 0.6, 0.0, -config.size * 0.5),
            Vec3::new(config.size * 0.6, 0.0, -config.size * 0.5),
        )));

        // Convert color to Vec4 for shader
        let color_rgba = config.color.to_linear();
        let base_color = Vec4::new(color_rgba.red, color_rgba.green, color_rgba.blue, 1.0);
        // Create complementary iridescence color
        let irid_color = Vec4::new(
            1.0 - color_rgba.red * 0.3,
            1.0 - color_rgba.green * 0.3,
            1.0 - color_rgba.blue * 0.3,
            1.0,
        );

        let material = materials.add(FishScalesMaterial {
            data: FishScalesData {
                base_color,
                iridescence_color: irid_color,
                scale_size: 12.0 + school_id as f32 * 3.0,
                shimmer_speed: 2.0 + rand::random::<f32>(),
                _padding: 0,
            },
        });

        for _ in 0..config.count {
            let offset = Vec3::new(
                (rand::random::<f32>() - 0.5) * 10.0,
                (rand::random::<f32>() - 0.5) * 5.0,
                (rand::random::<f32>() - 0.5) * 10.0,
            );

            let vel = Vec3::new(
                rand::random::<f32>() - 0.5,
                (rand::random::<f32>() - 0.5) * 0.3,
                rand::random::<f32>() - 0.5,
            )
            .normalize()
                * 3.0;

            let pos = config.center + offset;

            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(pos).looking_at(pos + vel, Vec3::Y),
                Fish {
                    velocity: vel,
                    school_id: school_id as u32,
                },
                Name::new("Fish"),
            ));
        }
    }
}

fn fish_boids(time: Res<Time>, mut query: Query<(&mut Transform, &mut Fish)>) {
    let dt = time.delta_secs();
    let perception_radius = 8.0;
    let avoidance_radius = 1.5;
    let max_speed = 5.0;
    let min_speed = 2.0;
    let turn_speed = 3.0;

    // Collect all fish data
    let fish_data: Vec<(Vec3, Vec3, u32)> = query
        .iter()
        .map(|(t, f)| (t.translation, f.velocity, f.school_id))
        .collect();

    for (mut transform, mut fish) in query.iter_mut() {
        let mut separation = Vec3::ZERO;
        let mut alignment = Vec3::ZERO;
        let mut cohesion = Vec3::ZERO;
        let mut count = 0;

        for (other_pos, other_vel, other_school) in &fish_data {
            // Fish prefer to school with their own species
            let same_school = *other_school == fish.school_id;
            let effective_perception = if same_school {
                perception_radius
            } else {
                perception_radius * 0.5
            };

            let dist = transform.translation.distance(*other_pos);
            if dist > 0.01 && dist < effective_perception {
                // Cohesion - stay with the school
                if same_school {
                    cohesion += *other_pos;
                    alignment += *other_vel;
                }

                // Separation - avoid collisions with all fish
                if dist < avoidance_radius {
                    let away = (transform.translation - *other_pos).normalize_or_zero();
                    separation += away / dist.max(0.1);
                }

                if same_school {
                    count += 1;
                }
            }
        }

        if count > 0 {
            cohesion = (cohesion / count as f32) - transform.translation;
            alignment /= count as f32;
        }

        // Keep fish in bounds (underwater area)
        let bounds_center = Vec3::new(0.0, 3.0, 0.0);
        let center_pull = (bounds_center - transform.translation) * 0.02;

        // Floor avoidance
        let floor_avoidance = if transform.translation.y < -2.0 {
            Vec3::Y * 2.0
        } else {
            Vec3::ZERO
        };

        // Ceiling avoidance
        let ceiling_avoidance = if transform.translation.y > 12.0 {
            Vec3::NEG_Y * 2.0
        } else {
            Vec3::ZERO
        };

        // Combine forces
        let target_velocity = fish.velocity
            + (separation * 2.0)
            + (alignment * 1.0)
            + (cohesion * 0.8)
            + center_pull
            + floor_avoidance
            + ceiling_avoidance;

        // Smooth velocity update
        fish.velocity = fish.velocity.lerp(
            target_velocity.normalize_or_zero() * max_speed,
            dt * turn_speed,
        );

        // Clamp speed
        let speed = fish.velocity.length();
        if speed < min_speed {
            fish.velocity = fish.velocity.normalize_or_zero() * min_speed;
        } else if speed > max_speed {
            fish.velocity = fish.velocity.normalize_or_zero() * max_speed;
        }

        // Update position and rotation
        transform.translation += fish.velocity * dt;
        if fish.velocity.length_squared() > 0.01 {
            let target_pos = transform.translation + fish.velocity;
            transform.look_at(target_pos, Vec3::Y);
        }
    }
}

// ============================================================================
// Jellyfish
// ============================================================================

#[derive(Component)]
pub struct Jellyfish {
    pub base_y: f32,
    pub phase: f32,
    pub pulse_speed: f32,
}

fn spawn_jellyfish(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<JellyfishMaterial>>,
) {
    let bell_mesh = meshes.add(Sphere::new(0.5));

    let jellyfish_configs = [
        (
            Vec3::new(5.0, 8.0, -5.0),
            Vec4::new(0.8, 0.2, 0.9, 0.7), // Purple
            Vec4::new(0.9, 0.5, 1.0, 1.0), // Pink glow
            true,                          // interactive
        ),
        (
            Vec3::new(-8.0, 10.0, 3.0),
            Vec4::new(0.2, 0.8, 0.9, 0.7), // Cyan
            Vec4::new(0.4, 0.9, 1.0, 1.0), // Bright cyan glow
            false,
        ),
        (
            Vec3::new(12.0, 6.0, 8.0),
            Vec4::new(0.9, 0.6, 0.2, 0.7), // Orange
            Vec4::new(1.0, 0.8, 0.4, 1.0), // Golden glow
            false,
        ),
        (
            Vec3::new(-3.0, 12.0, -12.0),
            Vec4::new(0.3, 0.9, 0.4, 0.7), // Green
            Vec4::new(0.5, 1.0, 0.6, 1.0), // Bright green glow
            false,
        ),
        (
            Vec3::new(20.0, 9.0, -8.0),
            Vec4::new(0.9, 0.3, 0.5, 0.7), // Pink/red
            Vec4::new(1.0, 0.5, 0.7, 1.0), // Magenta glow
            false,
        ),
    ];

    for (pos, base_color, glow_color, interactive) in jellyfish_configs {
        let material = materials.add(JellyfishMaterial {
            data: JellyfishData {
                base_color,
                glow_color,
                pulse_speed: 1.5 + rand::random::<f32>() * 0.5,
                translucency: 0.7,
                _padding: 0,
            },
        });

        if interactive {
            // Special interactive jellyfish
            let mut jelly = commands.spawn((
                Mesh3d(bell_mesh.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(pos).with_scale(Vec3::new(1.0, 0.6, 1.0)),
                Jellyfish {
                    base_y: pos.y,
                    phase: rand::random::<f32>() * std::f32::consts::TAU,
                    pulse_speed: 0.8 + rand::random::<f32>() * 0.4,
                },
                Name::new("Elder Jellyfish"),
                Hint::new("✨ An ethereal jellyfish... it seems to shimmer with ancient wisdom"),
                OceanDialogue {
                    node_name: "Jellyfish".to_string(),
                },
            ));
            jelly.observe(on_creature_click);
        } else {
            commands.spawn((
                Mesh3d(bell_mesh.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(pos).with_scale(Vec3::new(1.0, 0.6, 1.0)),
                Jellyfish {
                    base_y: pos.y,
                    phase: rand::random::<f32>() * std::f32::consts::TAU,
                    pulse_speed: 0.8 + rand::random::<f32>() * 0.4,
                },
                Name::new("Jellyfish"),
                Hint::new("A bioluminescent jellyfish drifting gracefully"),
            ));
        }
    }
}

fn animate_jellyfish(time: Res<Time>, mut query: Query<(&mut Transform, &Jellyfish)>) {
    let t = time.elapsed_secs();

    for (mut transform, jelly) in query.iter_mut() {
        // Bobbing motion
        let bob = (t * jelly.pulse_speed + jelly.phase).sin() * 0.5;
        transform.translation.y = jelly.base_y + bob;

        // Pulsing scale (bell contraction)
        let pulse = ((t * jelly.pulse_speed * 2.0 + jelly.phase).sin() * 0.5 + 0.5) * 0.2 + 0.9;
        transform.scale = Vec3::new(pulse, 0.6 / pulse, pulse);

        // Gentle drift
        transform.translation.x += (t * 0.1 + jelly.phase).sin() * 0.002;
        transform.translation.z += (t * 0.15 + jelly.phase).cos() * 0.002;
    }
}

// ============================================================================
// Click handler for interactive creatures
// ============================================================================

fn on_creature_click(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    project: Res<YarnProject>,
    dialogue_query: Query<&OceanDialogue>,
    existing_runners: Query<&DialogueRunner>,
) {
    if let Ok(creature_dialogue) = dialogue_query.get(click.event().entity) {
        start_dialogue(
            &mut commands,
            &project,
            &creature_dialogue.node_name,
            &existing_runners,
        );
    }
}

// ============================================================================
// Sea Turtle
// ============================================================================

#[derive(Component)]
pub struct Turtle {
    pub patrol_center: Vec3,
    pub patrol_radius: f32,
    pub angle: f32,
    pub speed: f32,
}

fn spawn_turtle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shell_materials: ResMut<Assets<TurtleShellMaterial>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
) {
    // Simplified turtle body
    let body_mesh = meshes.add(Sphere::new(1.5));
    let head_mesh = meshes.add(Sphere::new(0.4));
    let flipper_mesh = meshes.add(Capsule3d::new(0.15, 0.8));

    // Use custom shader material for the shell
    let shell_material = shell_materials.add(TurtleShellMaterial {
        data: TurtleShellData {
            base_color: Vec4::new(0.3, 0.5, 0.25, 1.0),
            accent_color: Vec4::new(0.4, 0.35, 0.2, 1.0),
            age: 0.85, // Ancient wise turtle
            roughness: 0.5,
            _padding: 0,
        },
    });

    let skin_material = std_materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.55, 0.35),
        perceptual_roughness: 0.8,
        ..default()
    });

    let start_pos = Vec3::new(0.0, 5.0, 15.0);

    let mut turtle = commands.spawn((
        Mesh3d(body_mesh),
        MeshMaterial3d(shell_material),
        Transform::from_translation(start_pos).with_scale(Vec3::new(1.2, 0.5, 1.0)),
        Turtle {
            patrol_center: Vec3::new(0.0, 5.0, 0.0),
            patrol_radius: 20.0,
            angle: 0.0,
            speed: 0.3,
        },
        Name::new("Sea Turtle"),
        Hint::new("🐢 An ancient sea turtle... click to speak with it"),
        OceanDialogue {
            node_name: "SeaTurtle".to_string(),
        },
    ));

    turtle.observe(on_creature_click);

    turtle.with_children(|parent| {
        // Head
        parent.spawn((
            Mesh3d(head_mesh),
            MeshMaterial3d(skin_material.clone()),
            Transform::from_xyz(0.0, 0.3, 1.3),
        ));

        // Front flippers
        parent.spawn((
            Mesh3d(flipper_mesh.clone()),
            MeshMaterial3d(skin_material.clone()),
            Transform::from_xyz(1.2, 0.0, 0.3).with_rotation(Quat::from_rotation_z(-0.5)),
        ));
        parent.spawn((
            Mesh3d(flipper_mesh.clone()),
            MeshMaterial3d(skin_material.clone()),
            Transform::from_xyz(-1.2, 0.0, 0.3).with_rotation(Quat::from_rotation_z(0.5)),
        ));

        // Back flippers
        parent.spawn((
            Mesh3d(flipper_mesh.clone()),
            MeshMaterial3d(skin_material.clone()),
            Transform::from_xyz(0.8, 0.0, -0.8)
                .with_rotation(Quat::from_rotation_z(-0.3))
                .with_scale(Vec3::splat(0.7)),
        ));
        parent.spawn((
            Mesh3d(flipper_mesh),
            MeshMaterial3d(skin_material),
            Transform::from_xyz(-0.8, 0.0, -0.8)
                .with_rotation(Quat::from_rotation_z(0.3))
                .with_scale(Vec3::splat(0.7)),
        ));
    });
}

fn patrol_turtle(time: Res<Time>, mut query: Query<(&mut Transform, &mut Turtle)>) {
    let dt = time.delta_secs();

    for (mut transform, mut turtle) in query.iter_mut() {
        turtle.angle += turtle.speed * dt;

        // Circular patrol with vertical undulation
        let target_x = turtle.patrol_center.x + turtle.angle.cos() * turtle.patrol_radius;
        let target_z = turtle.patrol_center.z + turtle.angle.sin() * turtle.patrol_radius;
        let target_y = turtle.patrol_center.y + (turtle.angle * 2.0).sin() * 2.0;

        let target = Vec3::new(target_x, target_y, target_z);
        let direction = (target - transform.translation).normalize_or_zero();

        transform.translation += direction * turtle.speed * 5.0 * dt;

        // Face movement direction
        if direction.length_squared() > 0.01 {
            let look_target = transform.translation + direction;
            transform.look_at(look_target, Vec3::Y);
        }
    }
}

// ============================================================================
// Creature bubble effects
// ============================================================================

/// Resource to track bubble spawning
#[derive(Resource)]
struct BubbleSpawnTimer {
    timer: Timer,
}

impl Default for BubbleSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.3, TimerMode::Repeating),
        }
    }
}

/// Spawn bubbles from turtle and fish
fn spawn_creature_bubbles(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Local<BubbleSpawnTimer>,
    mut meshes: Local<Option<Handle<Mesh>>>,
    mut materials: Local<Option<Handle<StandardMaterial>>>,
    mesh_assets: ResMut<Assets<Mesh>>,
    material_assets: ResMut<Assets<StandardMaterial>>,
    turtle_query: Query<&Transform, With<Turtle>>,
    fish_query: Query<&Transform, With<Fish>>,
) {
    timer.timer.tick(time.delta());

    if !timer.timer.just_finished() {
        return;
    }

    // Lazily initialize mesh and material handles
    let bubble_mesh = meshes.get_or_insert_with(|| {
        let mut mesh_assets = mesh_assets;
        mesh_assets.add(Sphere::new(0.05))
    });

    let bubble_material = materials.get_or_insert_with(|| {
        let mut material_assets = material_assets;
        material_assets.add(StandardMaterial {
            base_color: Color::srgba(0.9, 0.95, 1.0, 0.5),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })
    });

    // Spawn bubbles from turtle
    for transform in turtle_query.iter() {
        if rand::random::<f32>() < 0.4 {
            let offset = Vec3::new(
                (rand::random::<f32>() - 0.5) * 0.5,
                0.5,
                (rand::random::<f32>() - 0.5) * 0.5,
            );
            spawn_bubble(
                &mut commands,
                bubble_mesh.clone(),
                bubble_material.clone(),
                transform.translation + offset,
                0.4 + rand::random::<f32>() * 0.4,
            );
        }
    }

    // Spawn bubbles from some fish (not all, to avoid too many)
    let mut fish_count = 0;
    for transform in fish_query.iter() {
        if fish_count > 5 {
            break;
        }
        if rand::random::<f32>() < 0.1 {
            let offset = Vec3::new(
                (rand::random::<f32>() - 0.5) * 0.2,
                0.1,
                (rand::random::<f32>() - 0.5) * 0.2,
            );
            spawn_bubble(
                &mut commands,
                bubble_mesh.clone(),
                bubble_material.clone(),
                transform.translation + offset,
                0.2 + rand::random::<f32>() * 0.3,
            );
            fish_count += 1;
        }
    }
}

fn spawn_bubble(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
    scale: f32,
) {
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position).with_scale(Vec3::splat(scale)),
        CreatureBubble {
            velocity: Vec3::new(
                (rand::random::<f32>() - 0.5) * 0.5,
                1.5 + rand::random::<f32>() * 1.0,
                (rand::random::<f32>() - 0.5) * 0.5,
            ),
            lifetime: 0.0,
            max_lifetime: 2.0 + rand::random::<f32>() * 2.0,
        },
        Name::new("Creature Bubble"),
    ));
}

/// Animate and despawn creature bubbles
fn animate_creature_bubbles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut CreatureBubble)>,
) {
    let dt = time.delta_secs();
    let t = time.elapsed_secs();

    for (entity, mut transform, mut bubble) in query.iter_mut() {
        bubble.lifetime += dt;

        // Despawn if lifetime exceeded
        if bubble.lifetime >= bubble.max_lifetime {
            commands.entity(entity).despawn();
            continue;
        }

        // Move bubble upward with wobble
        transform.translation += bubble.velocity * dt;
        transform.translation.x += (t * 3.0 + bubble.lifetime).sin() * 0.01;
        transform.translation.z += (t * 2.5 + bubble.lifetime * 1.3).cos() * 0.01;

        // Slow down horizontal velocity over time
        bubble.velocity.x *= 0.98;
        bubble.velocity.z *= 0.98;

        // Fade out by shrinking
        let life_ratio = bubble.lifetime / bubble.max_lifetime;
        let fade = 1.0 - life_ratio.powi(2);
        transform.scale = Vec3::splat(transform.scale.x * (0.99 + fade * 0.01));
    }
}
