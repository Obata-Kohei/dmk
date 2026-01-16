use bevy::prelude::*;
use crate::component::*;

pub fn acceleration_system(
    time: Res<Time>,
    mut query: Query<(&Acceleration, &mut Velocity)>
) {
    let dt = time.delta_secs();

    for (a, mut v) in &mut query {
        v.0 += a.0 * dt;
    }
}


const SHOOTER_Z: f32 = 0.0;

pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>
) {
    let dt = time.delta_secs();

    for (v, mut tf) in &mut query {
        tf.translation += v.0.extend(SHOOTER_Z) * dt;  // v.0.extend(0.0)?
    }
}


const PLAYER_SPEED: f32 = 300.0;

pub fn player_key_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut query {
        let mut v = Vec2::ZERO;

        if keyboard.pressed(KeyCode::ArrowLeft) {
            v.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            v.x += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowUp) {
            v.y += 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            v.y -= 1.0;
        }

        if v != Vec2::ZERO {
            velocity.0 = v.normalize() * PLAYER_SPEED;
        } else {
            // キーを離したら即ゼロ
            velocity.0 = Vec2::ZERO;
        }
    }
}

pub fn confine_player_system(
    windows: Query<&Window>,
    mut query: Query<(&Collider, &mut Transform), With<Player>>,
) {
    let window = windows.single().expect("One Window should exist.");

    let half_w = window.width() * 0.5;
    let half_h = window.height() * 0.5;

    for (collider, mut tf) in &mut query {
        let r = collider.radius;

        tf.translation.x = tf.translation.x.clamp(-half_w + r, half_w - r);
        tf.translation.y = tf.translation.y.clamp(-half_h + r, half_h - r);
    }
}
