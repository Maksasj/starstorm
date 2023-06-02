use bevy::prelude::*;

pub use crate::components::{
    velocity::*,
};

#[derive(Component)]
pub struct Friction {
    pub rate: f32
}

pub fn friction_system(mut targets: Query<(&Friction, &mut Velocity)>) {
    for (friction, mut velocity) in targets.iter_mut() {
        velocity.velocity.x *= friction.rate;
        velocity.velocity.y *= friction.rate;
    }
}
