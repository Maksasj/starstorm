use bevy::prelude::*;

pub use crate::components::{
    entity_rotation::*,
};

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2
}

pub fn velocity_movement_system(mut targets: Query<(&mut Transform, &mut EntityRotation, &Velocity)>) {
    for (mut transform, rotation, velocity) in targets.iter_mut() {
        transform.translation.y += velocity.velocity.x * rotation.rotation_angle.sin();
        transform.translation.x += velocity.velocity.x * rotation.rotation_angle.cos();

        transform.translation.y += velocity.velocity.y * rotation.rotation_angle.cos();
        transform.translation.x -= velocity.velocity.y * rotation.rotation_angle.sin();
    }
}
