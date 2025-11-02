use bevy::prelude::*;

use crate::Animated;

/// Rotation speed in radians per second
const ROTATION_SPEED: f32 = 0.5;

/// Animates entities marked with the Animated component by rotating them around the Y axis
pub fn animate(mut query: Query<&mut Transform, With<Animated>>, time: Res<Time>) {
    let delta_seconds = time.delta_secs();
    let rotation_amount = delta_seconds * ROTATION_SPEED;

    for mut transform in &mut query {
        transform.rotate_y(rotation_amount);
    }
}
