use bevy::prelude::*;
use crate::component::{Acceleration, Velocity};

pub fn acceleration_system(
    time: Res<Time>,
    mut query: Query<(&Acceleration, &mut Velocity)>
) {
    let dt = time.delta_secs();

    for (a, mut v) in &mut query {
        v.0 += a.0 * dt;
    }
}

pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>
) {
    let dt = time.delta_secs();

    for (v, mut tf) in &mut query {
        tf.translation += v.0 * dt;  // v.0.extend(0.0)?
    }
}


