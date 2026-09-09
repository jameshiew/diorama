use bevy::prelude::*;

#[derive(Component)]
pub struct OrbitRing {
    pub tilt: Quat,
    pub speed: f32,
}

#[derive(Component)]
pub struct FloatingLantern {
    pub anchor: Vec3,
    pub phase: f32,
}

#[derive(Component)]
pub struct Tide;

fn tide_height(elapsed: f32) -> f32 {
    (elapsed * 0.22).sin() * 0.12
}

pub fn animate_rings(time: Res<Time>, mut rings: Query<(&mut Transform, &OrbitRing)>) {
    for (mut transform, ring) in &mut rings {
        transform.rotation = ring.tilt * Quat::from_rotation_y(time.elapsed_secs() * ring.speed);
    }
}

pub fn animate_lanterns(time: Res<Time>, mut lanterns: Query<(&mut Transform, &FloatingLantern)>) {
    let elapsed = time.elapsed_secs();
    for (mut transform, lantern) in &mut lanterns {
        let phase = elapsed * 0.65 + lantern.phase;
        transform.translation = lantern.anchor
            + Vec3::new(
                phase.cos() * 0.12,
                tide_height(elapsed) + phase.sin() * 0.04,
                0.0,
            );
        transform.rotation = Quat::from_rotation_z(phase.sin() * 0.06);
    }
}

pub fn animate_tide(time: Res<Time>, mut water: Query<&mut Transform, With<Tide>>) {
    for mut transform in &mut water {
        transform.translation.y = 0.05 + tide_height(time.elapsed_secs());
    }
}
