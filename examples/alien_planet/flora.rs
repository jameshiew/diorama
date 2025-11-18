use avian3d::prelude::*;
use bevy::prelude::*;
use diorama::picking::Hint;
use noise::{NoiseFn, Perlin};

use crate::materials::{CrystalMaterial, CrystalMaterialUniform};

pub struct FloraPlugin;

impl Plugin for FloraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_flora)
            .add_systems(Update, animate_bushes);
    }
}

#[derive(Component)]
pub struct Plant;

#[derive(Component)]
pub struct BushAnimation {
    pub target_scale: Vec3,
    pub speed: f32,
}

#[derive(Component)]
pub struct Scannable {
    pub name: String,
    pub description: String,
}

fn spawn_flora(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut crystal_materials: ResMut<Assets<CrystalMaterial>>,
) {
    let perlin = Perlin::new(1);
    let size = 100;
    let scale = 2.0;
    let height_scale = 10.0;
    let offset = Vec3::new(-50.0, -10.0, -50.0);

    let spire_mesh = meshes.add(Cylinder::new(0.2, 4.0));

    let bush_mesh = meshes.add(Sphere::new(0.8));
    let bush_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.2, 0.5),
        perceptual_roughness: 0.3,
        ..default()
    });

    // Spawn Spires
    for _ in 0..50 {
        let x = (rand::random::<f64>() * size as f64 * scale) as f32;
        let z = (rand::random::<f64>() * size as f64 * scale) as f32;

        let y_pos = perlin.get([x as f64 * 0.05, z as f64 * 0.05]) * height_scale
            + perlin.get([x as f64 * 0.1, z as f64 * 0.1]) * (height_scale * 0.5);

        let pos = Vec3::new(x, y_pos as f32, z) + offset;

        let spire_mat = crystal_materials.add(CrystalMaterial {
            uniform: CrystalMaterialUniform {
                base_color: LinearRgba::rgb(0.1, 0.8, 0.9),
                emissive: LinearRgba::rgb(0.0, 0.5, 0.8),
            },
        });

        commands
            .spawn((
                Mesh3d(spire_mesh.clone()),
                MeshMaterial3d(spire_mat),
                Transform::from_translation(pos + Vec3::Y * 2.0), // Half height
                Plant,
                Collider::cylinder(0.2, 4.0),
                Name::new("Crystal Spire"),
                Scannable {
                    name: "Crystal Spire".to_string(),
                    description: "A resonating crystal structure that hums in the wind."
                        .to_string(),
                },
                Hint::new("Click to resonate"),
            ))
            .observe(on_spire_click);
    }

    // Spawn Bubble Bushes
    for _ in 0..100 {
        let x = (rand::random::<f64>() * size as f64 * scale) as f32;
        let z = (rand::random::<f64>() * size as f64 * scale) as f32;

        let y_pos = perlin.get([x as f64 * 0.05, z as f64 * 0.05]) * height_scale
            + perlin.get([x as f64 * 0.1, z as f64 * 0.1]) * (height_scale * 0.5);

        let pos = Vec3::new(x, y_pos as f32, z) + offset;

        commands
            .spawn((
                Mesh3d(bush_mesh.clone()),
                MeshMaterial3d(bush_mat.clone()),
                Transform::from_translation(pos + Vec3::Y * 0.5),
                Plant,
                Collider::sphere(0.8),
                Name::new("Bubble Bush"),
                Scannable {
                    name: "Bubble Bush".to_string(),
                    description: "Contains pressurized gas. Do not puncture.".to_string(),
                },
                Hint::new("Click to poke"),
                BushAnimation {
                    target_scale: Vec3::ONE,
                    speed: 5.0,
                },
            ))
            .observe(on_bush_click);
    }
}

fn on_spire_click(
    click: On<Pointer<Click>>,
    mut materials: ResMut<Assets<CrystalMaterial>>,
    query: Query<&MeshMaterial3d<CrystalMaterial>>,
) {
    if let Some(material) = query
        .get(click.entity)
        .ok()
        .and_then(|h| materials.get_mut(h))
    {
        material.uniform.emissive = LinearRgba::rgb(
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
        );
        material.uniform.base_color = LinearRgba::rgb(
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
        );
    }
}

fn on_bush_click(click: On<Pointer<Click>>, mut query: Query<&mut BushAnimation>) {
    if let Ok(mut animation) = query.get_mut(click.entity) {
        if animation.target_scale.x > 1.5 {
            animation.target_scale = Vec3::ONE;
        } else {
            animation.target_scale = Vec3::splat(2.0);
        }
    }
}

fn animate_bushes(time: Res<Time>, mut query: Query<(&mut Transform, &BushAnimation)>) {
    for (mut transform, animation) in query.iter_mut() {
        transform.scale = transform
            .scale
            .lerp(animation.target_scale, time.delta_secs() * animation.speed);
    }
}
