use avian2d::prelude::*;
use bevy::prelude::*;

pub fn accelerate_bodies(
    mut query: Query<(&mut LinearVelocity, &mut AngularVelocity)>,
    time: Res<Time>,
) {
    let delta_secs = time.delta_secs();

    for (mut linear_velocity, mut angular_velocity) in &mut query {
        linear_velocity.x += 2.0 + delta_secs;
        angular_velocity.0 = 0.5 + delta_secs;
    }
}
