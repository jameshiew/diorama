use avian3d::prelude::*;
use bevy::prelude::*;

use crate::flora::Scannable;

pub struct FaunaPlugin;

impl Plugin for FaunaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_fauna)
            .add_systems(Update, boid_simulation);
    }
}

#[derive(Component)]
pub struct Boid {
    pub velocity: Vec3,
}

fn spawn_fauna(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(Triangle3d::new(
        Vec3::new(0.0, 0.0, 0.5),
        Vec3::new(-0.5, 0.0, -0.5),
        Vec3::new(0.5, 0.0, -0.5),
    )));

    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.8, 0.2),
        double_sided: true,
        ..default()
    });

    for _ in 0..50 {
        let pos = Vec3::new(
            (rand::random::<f32>() - 0.5) * 50.0,
            10.0 + rand::random::<f32>() * 20.0,
            (rand::random::<f32>() - 0.5) * 50.0,
        );
        let vel = Vec3::new(
            rand::random::<f32>() - 0.5,
            rand::random::<f32>() - 0.5,
            rand::random::<f32>() - 0.5,
        )
        .normalize()
            * 5.0;

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(pos).looking_at(pos + vel, Vec3::Y),
            Collider::sphere(0.5),
            RigidBody::Kinematic, // Kinematic because we move them manually
            Boid { velocity: vel },
            Name::new("Sky Ray"),
            Scannable {
                name: "Sky Ray".to_string(),
                description: "A passive airborne creature that feeds on solar radiation."
                    .to_string(),
            },
        ));
    }
}

fn boid_simulation(time: Res<Time>, mut query: Query<(&mut Transform, &mut Boid)>) {
    let dt = time.delta_secs();
    let perception_radius = 10.0;
    let avoidance_radius = 2.0;
    let max_speed = 8.0;
    let min_speed = 3.0;
    let turn_speed = 2.0;

    // Collect all positions and velocities first to avoid borrowing issues
    // (Naive O(N^2) approach is fine for N=50)
    let boids: Vec<(Vec3, Vec3)> = query
        .iter()
        .map(|(t, b)| (t.translation, b.velocity))
        .collect();

    for (mut transform, mut boid) in query.iter_mut() {
        let mut separation = Vec3::ZERO;
        let mut alignment = Vec3::ZERO;
        let mut cohesion = Vec3::ZERO;
        let mut count = 0;

        for (other_pos, other_vel) in &boids {
            let dist = transform.translation.distance(*other_pos);
            if dist > 0.0 && dist < perception_radius {
                // Cohesion
                cohesion += *other_pos;

                // Alignment
                alignment += *other_vel;

                // Separation
                if dist < avoidance_radius {
                    separation -= (*other_pos - transform.translation).normalize() / dist;
                }

                count += 1;
            }
        }

        if count > 0 {
            cohesion = (cohesion / count as f32) - transform.translation;
            alignment /= count as f32;
        }

        // World center attraction (keep them in the arena)
        let center_pull = -transform.translation * 0.05;

        // Combine forces
        let target_velocity =
            boid.velocity + (separation * 1.5) + (alignment * 1.0) + (cohesion * 1.0) + center_pull;

        // Update velocity
        boid.velocity = boid
            .velocity
            .lerp(target_velocity.normalize() * max_speed, dt * turn_speed);

        // Clamp speed
        let speed = boid.velocity.length();
        if speed < min_speed {
            boid.velocity = boid.velocity.normalize() * min_speed;
        } else if speed > max_speed {
            boid.velocity = boid.velocity.normalize() * max_speed;
        }

        // Move
        transform.translation += boid.velocity * dt;
        let translation = transform.translation;
        transform.look_at(translation + boid.velocity, Vec3::Y);
    }
}
