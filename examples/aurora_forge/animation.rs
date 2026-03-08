use std::f32::consts::PI;

use bevy::prelude::*;

/// Rotates an entity around a local axis.
#[derive(Component)]
pub struct RingRotor {
    pub axis: Vec3,
    pub speed: f32,
}

/// Moves an entity on a circular orbit with configurable vertical motion.
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

/// Adds a softer drifting motion suitable for banners, crystals, and sails.
#[derive(Component)]
pub struct HoverMotion {
    pub anchor: Vec3,
    pub vertical_amplitude: f32,
    pub lateral_amplitude: f32,
    pub speed: f32,
    pub phase: f32,
    pub yaw_offset: f32,
    pub pitch_tilt: f32,
}

/// Pulses point light intensity over time.
#[derive(Component)]
pub struct PulseLight {
    pub base_intensity: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub phase: f32,
}

/// Sweeps a spotlight around a shared center and continuously retargets it.
#[derive(Component)]
pub struct SweepSpotlight {
    pub center: Vec3,
    pub radius: f32,
    pub height: f32,
    pub angular_speed: f32,
    pub phase: f32,
    pub focus_height: f32,
    pub wobble: f32,
}

pub fn animate_rotors(time: Res<Time>, mut query: Query<(&mut Transform, &RingRotor)>) {
    let delta = time.delta_secs();

    for (mut transform, rotor) in &mut query {
        let axis = rotor.axis.normalize_or_zero();
        if axis.length_squared() <= f32::EPSILON {
            continue;
        }

        transform.rotate(Quat::from_axis_angle(axis, rotor.speed * delta));
    }
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
        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            -angle + PI * 0.5,
            bob * 0.08,
            elapsed * orbit.spin_speed + orbit.phase,
        );
    }
}

pub fn animate_hoverers(time: Res<Time>, mut query: Query<(&mut Transform, &HoverMotion)>) {
    let elapsed = time.elapsed_secs();

    for (mut transform, hover) in &mut query {
        let t = elapsed * hover.speed + hover.phase;
        let sway_x = t.cos() * hover.lateral_amplitude;
        let sway_z = (elapsed * hover.speed * 0.71 + hover.phase).sin() * hover.lateral_amplitude;
        let lift = t.sin() * hover.vertical_amplitude;

        transform.translation = hover.anchor + Vec3::new(sway_x, lift, sway_z);
        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            hover.yaw_offset + t * 0.15,
            hover.pitch_tilt + sway_z * 0.025,
            sway_x * 0.02,
        );
    }
}

pub fn animate_pulsing_lights(time: Res<Time>, mut query: Query<(&mut PointLight, &PulseLight)>) {
    let elapsed = time.elapsed_secs();

    for (mut light, pulse) in &mut query {
        let wave = (elapsed * pulse.speed + pulse.phase).sin() * 0.5 + 0.5;
        light.intensity = (pulse.base_intensity + pulse.amplitude * wave).max(0.0);
    }
}

pub fn animate_sweeping_spotlights(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &SweepSpotlight)>,
) {
    let elapsed = time.elapsed_secs();

    for (mut transform, sweep) in &mut query {
        let angle = elapsed * sweep.angular_speed + sweep.phase;
        let position = sweep.center
            + Vec3::new(
                angle.cos() * sweep.radius,
                sweep.height,
                angle.sin() * sweep.radius,
            );
        let target = sweep.center
            + Vec3::new(
                (elapsed * 0.37 + sweep.phase).cos() * sweep.wobble,
                sweep.focus_height + (elapsed * 0.63 + sweep.phase).sin() * sweep.wobble * 0.35,
                (elapsed * 0.37 + sweep.phase).sin() * sweep.wobble,
            );

        *transform = Transform::from_translation(position).looking_at(target, Vec3::Y);
    }
}
