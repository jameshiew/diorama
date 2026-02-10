use bevy::prelude::*;

/// Rotates the attached mesh around a local axis over time.
#[derive(Component)]
pub struct RingSpin {
    pub axis: Vec3,
    pub speed: f32,
}

/// Moves a body on a circular orbit around a shared center with vertical oscillation.
#[derive(Component)]
pub struct OrbitingBody {
    pub center: Vec3,
    pub radius: f32,
    pub angular_speed: f32,
    pub vertical_amplitude: f32,
    pub vertical_speed: f32,
    pub phase: f32,
}

/// Pulses point light intensity with a configurable sine wave profile.
#[derive(Component)]
pub struct PulsingLight {
    pub base_intensity: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub phase: f32,
}

/// Adds a soft hovering motion for floating lantern meshes.
#[derive(Component)]
pub struct BobbingLantern {
    pub base_position: Vec3,
    pub vertical_amplitude: f32,
    pub sway_amplitude: f32,
    pub speed: f32,
    pub phase: f32,
}

pub fn animate_ring_spins(time: Res<Time>, mut query: Query<(&mut Transform, &RingSpin)>) {
    let delta = time.delta_secs();

    for (mut transform, ring_spin) in &mut query {
        let axis = ring_spin.axis.normalize_or_zero();
        if axis.length_squared() <= f32::EPSILON {
            continue;
        }

        transform.rotate(Quat::from_axis_angle(axis, ring_spin.speed * delta));
    }
}

pub fn animate_orbits(time: Res<Time>, mut query: Query<(&mut Transform, &OrbitingBody)>) {
    let elapsed = time.elapsed_secs();
    let delta = time.delta_secs();

    for (mut transform, orbit) in &mut query {
        let angle = elapsed * orbit.angular_speed + orbit.phase;
        let orbit_x = angle.cos() * orbit.radius;
        let orbit_z = angle.sin() * orbit.radius;
        let orbit_y =
            (elapsed * orbit.vertical_speed + orbit.phase).sin() * orbit.vertical_amplitude;

        transform.translation = orbit.center + Vec3::new(orbit_x, orbit_y, orbit_z);
        transform.rotate_y(delta * (orbit.angular_speed * 0.75));
    }
}

pub fn animate_lights(time: Res<Time>, mut query: Query<(&mut PointLight, &PulsingLight)>) {
    let elapsed = time.elapsed_secs();

    for (mut light, pulse) in &mut query {
        let wave = (elapsed * pulse.speed + pulse.phase).sin() * 0.5 + 0.5;
        let intensity = pulse.base_intensity + pulse.amplitude * wave;
        light.intensity = intensity.max(0.0);
    }
}

pub fn animate_lanterns(time: Res<Time>, mut query: Query<(&mut Transform, &BobbingLantern)>) {
    let elapsed = time.elapsed_secs();

    for (mut transform, lantern) in &mut query {
        let vertical = (elapsed * lantern.speed + lantern.phase).sin() * lantern.vertical_amplitude;
        let sway_x =
            (elapsed * lantern.speed * 0.75 + lantern.phase).cos() * lantern.sway_amplitude;
        let sway_z = (elapsed * lantern.speed * 0.6 + lantern.phase).sin() * lantern.sway_amplitude;

        transform.translation = lantern.base_position + Vec3::new(sway_x, vertical, sway_z);
        transform.rotate_y(time.delta_secs() * 0.2);
    }
}
