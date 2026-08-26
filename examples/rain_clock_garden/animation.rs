use bevy::prelude::*;

#[derive(Component)]
pub struct ClockHand {
    pub radians_per_second: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct Drift {
    pub anchor: Vec3,
    pub horizontal_radius: Vec2,
    pub vertical_amplitude: f32,
    pub speed: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct FallingDrop {
    pub top_y: f32,
    pub bottom_y: f32,
    pub units_per_second: f32,
    pub phase_distance: f32,
}

#[derive(Component)]
pub struct Pendulum {
    pub amplitude: f32,
    pub speed: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct PulseLight {
    pub base_intensity: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct Ripple {
    pub minimum_scale: f32,
    pub maximum_scale: f32,
    pub cycles_per_second: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct Spin {
    pub local_axis: Vec3,
    pub radians_per_second: f32,
}

pub fn animate_clock_hands(time: Res<Time>, mut hands: Query<(&mut Transform, &ClockHand)>) {
    let elapsed = time.elapsed_secs();

    for (mut transform, hand) in &mut hands {
        transform.rotation = Quat::from_rotation_z(hand.phase - elapsed * hand.radians_per_second);
    }
}

pub fn animate_drifts(time: Res<Time>, mut drifts: Query<(&mut Transform, &Drift)>) {
    let elapsed = time.elapsed_secs();

    for (mut transform, drift) in &mut drifts {
        let angle = elapsed * drift.speed + drift.phase;
        transform.translation = drift.anchor
            + Vec3::new(
                angle.cos() * drift.horizontal_radius.x,
                (angle * 0.73).sin() * drift.vertical_amplitude,
                angle.sin() * drift.horizontal_radius.y,
            );
    }
}

pub fn animate_drops(time: Res<Time>, mut drops: Query<(&mut Transform, &FallingDrop)>) {
    let elapsed = time.elapsed_secs();

    for (mut transform, drop) in &mut drops {
        let travel = (drop.top_y - drop.bottom_y).max(f32::EPSILON);
        let distance = (elapsed * drop.units_per_second + drop.phase_distance).rem_euclid(travel);
        transform.translation.y = drop.top_y - distance;
    }
}

pub fn animate_pendulums(time: Res<Time>, mut pendulums: Query<(&mut Transform, &Pendulum)>) {
    let elapsed = time.elapsed_secs();

    for (mut transform, pendulum) in &mut pendulums {
        let angle = (elapsed * pendulum.speed + pendulum.phase).sin() * pendulum.amplitude;
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

pub fn animate_pulse_lights(time: Res<Time>, mut lights: Query<(&mut PointLight, &PulseLight)>) {
    let elapsed = time.elapsed_secs();

    for (mut light, pulse) in &mut lights {
        let wave = (elapsed * pulse.speed + pulse.phase).sin() * 0.5 + 0.5;
        light.intensity = (pulse.base_intensity + pulse.amplitude * wave).max(0.0);
    }
}

pub fn animate_ripples(time: Res<Time>, mut ripples: Query<(&mut Transform, &Ripple)>) {
    let elapsed = time.elapsed_secs();

    for (mut transform, ripple) in &mut ripples {
        let cycle = (elapsed * ripple.cycles_per_second + ripple.phase).rem_euclid(1.0);
        let scale =
            ripple.minimum_scale + (ripple.maximum_scale - ripple.minimum_scale).max(0.0) * cycle;
        transform.scale = Vec3::splat(scale);
    }
}

pub fn animate_spins(time: Res<Time>, mut spins: Query<(&mut Transform, &Spin)>) {
    let delta = time.delta_secs();

    for (mut transform, spin) in &mut spins {
        let axis = spin.local_axis.normalize_or_zero();
        if axis.length_squared() <= f32::EPSILON {
            continue;
        }

        transform.rotate_local(Quat::from_axis_angle(axis, spin.radians_per_second * delta));
    }
}
