//! Animation components and systems for the Mycelial Reverie scene.

use bevy::prelude::*;

/// Orbits an entity around a center point with optional vertical oscillation and spin.
#[derive(Component)]
pub struct OrbitMotion {
    pub center: Vec3,
    pub radius: f32,
    pub height: f32,
    pub angular_speed: f32,
    pub vertical_amplitude: f32,
    pub vertical_speed: f32,
    pub phase: f32,
    pub spin_speed: f32,
}

/// Lightly hovers an entity around a fixed anchor with sway and lift.
#[derive(Component)]
pub struct HoverMotion {
    pub anchor: Vec3,
    pub vertical_amplitude: f32,
    pub lateral_amplitude: f32,
    pub speed: f32,
    pub phase: f32,
}

/// Pulses a point light's intensity over time.
#[derive(Component)]
pub struct PulseLight {
    pub base_intensity: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub phase: f32,
}

/// Continuously rotates an entity around a local axis.
#[derive(Component)]
pub struct SpinMotion {
    pub axis: Vec3,
    pub speed: f32,
}

pub fn animate_orbits(time: Res<Time>, mut query: Query<(&mut Transform, &OrbitMotion)>) {
    let elapsed = time.elapsed_secs();

    for (mut transform, orbit) in &mut query {
        let angle = elapsed * orbit.angular_speed + orbit.phase;
        let bob = (elapsed * orbit.vertical_speed + orbit.phase).sin() * orbit.vertical_amplitude;
        let position = Vec3::new(
            angle.cos() * orbit.radius,
            orbit.height + bob,
            angle.sin() * orbit.radius,
        );

        transform.translation = orbit.center + position;
        if orbit.spin_speed.abs() > f32::EPSILON {
            transform.rotation = Quat::from_rotation_y(elapsed * orbit.spin_speed + orbit.phase);
        }
    }
}

pub fn animate_hovers(time: Res<Time>, mut query: Query<(&mut Transform, &HoverMotion)>) {
    let elapsed = time.elapsed_secs();

    for (mut transform, hover) in &mut query {
        let t = elapsed * hover.speed + hover.phase;
        let sway_x = t.cos() * hover.lateral_amplitude;
        let sway_z = (elapsed * hover.speed * 0.71 + hover.phase).sin() * hover.lateral_amplitude;
        let lift = t.sin() * hover.vertical_amplitude;

        transform.translation = hover.anchor + Vec3::new(sway_x, lift, sway_z);
    }
}

pub fn animate_pulse_lights(time: Res<Time>, mut query: Query<(&mut PointLight, &PulseLight)>) {
    let elapsed = time.elapsed_secs();

    for (mut light, pulse) in &mut query {
        let wave = (elapsed * pulse.speed + pulse.phase).sin() * 0.5 + 0.5;
        light.intensity = (pulse.base_intensity + pulse.amplitude * wave).max(0.0);
    }
}

pub fn animate_spins(time: Res<Time>, mut query: Query<(&mut Transform, &SpinMotion)>) {
    let delta = time.delta_secs();

    for (mut transform, spin) in &mut query {
        let axis = spin.axis.normalize_or_zero();
        if axis.length_squared() <= f32::EPSILON {
            continue;
        }
        transform.rotate(Quat::from_axis_angle(axis, spin.speed * delta));
    }
}
