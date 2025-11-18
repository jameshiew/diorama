use bevy::prelude::*;
use rand::prelude::*;

pub struct AtmospherePlugin;

impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
            .insert_resource(AmbientLight {
                color: Color::srgb(0.1, 0.1, 0.2),
                brightness: 200.0,
                affects_lightmapped_meshes: false,
            })
            .add_systems(Startup, (setup_lights, spawn_stars));
        // .add_systems(Update, add_fog_to_camera); // FogSettings not found
    }
}

fn setup_lights(mut commands: Commands) {
    // Sun
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            color: Color::srgb(1.0, 0.9, 0.8),
            ..default()
        },
        Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spawn_stars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::rng();
    let star_count = 2000;
    let radius = 400.0;

    let mesh = meshes.add(Sphere::new(0.5));
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::WHITE,
        unlit: true,
        ..default()
    });

    for _ in 0..star_count {
        let dir = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        )
        .normalize();

        let pos = dir * radius;

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(pos),
        ));
    }
}

/*
fn add_fog_to_camera(
    mut commands: Commands,
    query: Query<Entity, (With<Camera3d>, Without<FogSettings>)>,
) {
    for entity in &query {
        commands.entity(entity).insert(FogSettings {
            color: Color::srgb(0.1, 0.1, 0.2),
            falloff: FogFalloff::ExponentialSquared { density: 0.02 },
            ..default()
        });
    }
}
*/
