//! Animation components and systems for the Unwritten Fable scene.

use std::f32::consts::TAU;

use bevy::prelude::*;

/// The great quill: bobs and tilts as if writing, while slowly drifting
/// along the frontier line.
#[derive(Component)]
pub struct QuillMotion {
    pub anchor: Vec3,
    pub bob_amplitude: f32,
    pub sway_amplitude: f32,
    /// How far the quill wanders along z while it writes.
    pub drift_span: f32,
    /// Speed of the short writing strokes.
    pub write_speed: f32,
    pub phase: f32,
}

/// A glyph flying from the quill nib to its place on the page, along an
/// arched path with a slight corkscrew, shrinking as it settles.
#[derive(Component)]
pub struct GlyphFlight {
    pub start: Vec3,
    pub end: Vec3,
    pub arc_height: f32,
    /// Journeys per second.
    pub speed: f32,
    pub phase: f32,
    pub spin: f32,
}

/// Ambient mote that rises, sways, and shrinks away before looping.
#[derive(Component)]
pub struct DriftMote {
    pub anchor: Vec3,
    pub rise: f32,
    pub sway: f32,
    /// Loops per second.
    pub speed: f32,
    pub phase: f32,
    pub base_scale: f32,
}

/// A droplet of ink falling from the quill nib, accelerating as it goes.
#[derive(Component)]
pub struct InkDrip {
    pub origin: Vec3,
    pub fall: f32,
    /// Drops per second.
    pub speed: f32,
    pub phase: f32,
}

/// Breathing light intensity.
#[derive(Component)]
pub struct PulseLight {
    pub base_intensity: f32,
    pub amplitude: f32,
    pub speed: f32,
    pub phase: f32,
}

pub fn animate_quills(time: Res<Time>, mut query: Query<(&mut Transform, &QuillMotion)>) {
    let elapsed = time.elapsed_secs();
    for (mut transform, quill) in &mut query {
        let t = elapsed + quill.phase;
        let drift = (t * 0.11).sin() * quill.drift_span;
        let bob = (t * 0.9).sin() * quill.bob_amplitude + (t * quill.write_speed).sin() * 0.12;
        let sway = (t * 0.53).sin() * quill.sway_amplitude;

        // Short scratching strokes layered over a slow lean. The big lean is
        // around x so the feather's profile faces along the page.
        let pitch_jitter = (t * quill.write_speed * 1.7).sin() * 0.05;
        let roll_jitter = (t * quill.write_speed).cos() * 0.06;

        transform.translation = quill.anchor + Vec3::new(sway, bob, drift);
        transform.rotation =
            Quat::from_rotation_x(-0.55 + pitch_jitter) * Quat::from_rotation_z(0.15 + roll_jitter);
    }
}

pub fn animate_glyph_flights(time: Res<Time>, mut query: Query<(&mut Transform, &GlyphFlight)>) {
    let elapsed = time.elapsed_secs();
    for (mut transform, glyph) in &mut query {
        let t = (elapsed * glyph.speed + glyph.phase).fract();

        // Quadratic bezier through a raised midpoint
        let mid = glyph.start.midpoint(glyph.end) + Vec3::Y * glyph.arc_height;
        let a = glyph.start.lerp(mid, t);
        let b = mid.lerp(glyph.end, t);
        let mut position = a.lerp(b, t);

        // Slight corkscrew around the path
        let swirl = t * TAU * 2.0 + glyph.phase * TAU;
        position += Vec3::new(swirl.cos(), 0.0, swirl.sin()) * 0.5 * (1.0 - t);

        // Settle into the page: shrink near the end of the journey
        let settle = 1.0 - smoothstep(0.78, 1.0, t);
        let appear = smoothstep(0.0, 0.06, t);

        transform.translation = position;
        transform.scale = Vec3::splat((0.25 + 0.75 * settle) * appear.max(0.05));
        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            elapsed * glyph.spin,
            elapsed * glyph.spin * 0.63,
            glyph.phase * TAU,
        );
    }
}

pub fn animate_drift_motes(time: Res<Time>, mut query: Query<(&mut Transform, &DriftMote)>) {
    let elapsed = time.elapsed_secs();
    for (mut transform, mote) in &mut query {
        let t = (elapsed * mote.speed + mote.phase).fract();
        let sway_angle = elapsed * 0.4 + mote.phase * TAU;

        transform.translation = mote.anchor
            + Vec3::new(
                sway_angle.sin() * mote.sway,
                t * mote.rise,
                (sway_angle * 0.7).cos() * mote.sway,
            );

        let fade = (1.0 - smoothstep(0.7, 1.0, t)) * smoothstep(0.0, 0.1, t);
        transform.scale = Vec3::splat(mote.base_scale * (0.2 + 0.8 * fade));
    }
}

pub fn animate_ink_drips(time: Res<Time>, mut query: Query<(&mut Transform, &InkDrip)>) {
    let elapsed = time.elapsed_secs();
    for (mut transform, drip) in &mut query {
        let t = (elapsed * drip.speed + drip.phase).fract();

        // Accelerating fall, stretching slightly as it speeds up
        transform.translation = drip.origin - Vec3::Y * (drip.fall * t * t);
        transform.scale = Vec3::new(1.0 - t * 0.3, 1.0 + t * 0.9, 1.0 - t * 0.3);
    }
}

pub fn animate_pulse_lights(time: Res<Time>, mut query: Query<(&mut PointLight, &PulseLight)>) {
    let elapsed = time.elapsed_secs();
    for (mut light, pulse) in &mut query {
        let wave = (elapsed * pulse.speed + pulse.phase).sin() * 0.5 + 0.5;
        light.intensity = (pulse.base_intensity + pulse.amplitude * wave).max(0.0);
    }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
