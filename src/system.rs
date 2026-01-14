use bevy::prelude::*;

use crate::component::{Acceleration, Velocity};


pub fn movement_system(
    time: Res<Time>,
    mut query: Query< (&Acceleration, &mut Velocity, &mut Transform) >
) {
    for (a, mut v, mut tf) in &mut query {
        v.0 = a.0 * time.delta_secs();
        tf.translation += v.0 * time.delta_secs();
    }
}
