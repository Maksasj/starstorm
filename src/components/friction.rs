use bevy::prelude::*;

pub use crate::components::{
    velocity::*,
};

#[derive(Component)]
pub struct Friction {
    pub rate: f32
}

impl Friction {
    pub fn new(rate: f32) -> Self {
        Friction { 
            rate: rate
        }
    }
}

pub fn friction_system(mut targets: Query<(&Friction, &mut Velocity)>) {
    for (friction, mut velocity) in targets.iter_mut() {
        velocity.velocity.x *= friction.rate;
        velocity.velocity.y *= friction.rate;
    }
}
