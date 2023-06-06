use bevy::prelude::*;

pub use crate::components::{
    entity_rotation::*,
    player_controller::*,
};

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2
}

impl Velocity {
    pub fn new() -> Self {
        Velocity { 
            velocity: Vec2::splat(0.0)
        }
    }

    pub fn with(velocity_x: f32, velocity_y: f32) -> Self {
        Velocity { 
            velocity: Vec2::new(velocity_x, velocity_y)
        }
    }
}

pub fn velocity_movement_system(mut targets: Query<(&mut Transform, &mut EntityRotation, &Velocity), Without<PlayerController>>, time: Res<Time>) {
    for (mut transform, rotation, velocity) in targets.iter_mut() {
        let mut new_translation = transform.translation;

        new_translation.y += velocity.velocity.x * rotation.rotation_angle.sin() * time.delta_seconds();
        new_translation.x += velocity.velocity.x * rotation.rotation_angle.cos() * time.delta_seconds();
        new_translation.y += velocity.velocity.y * rotation.rotation_angle.cos() * time.delta_seconds();
        new_translation.x -= velocity.velocity.y * rotation.rotation_angle.sin() * time.delta_seconds();

        if new_translation.x < 210.0 && new_translation.x > -210.0 {
            transform.translation.x = new_translation.x; 
        } 

        transform.translation.y = new_translation.y; 
    }
}

pub fn player_velocity_movement_system(mut targets: Query<(&mut Transform, &mut EntityRotation, &Velocity), With<PlayerController>>, time: Res<Time>) {
    for (mut transform, rotation, velocity) in targets.iter_mut() {
        let mut new_translation = transform.translation;

        new_translation.x += velocity.velocity.x * time.delta_seconds(); // *  rotation.rotation_angle.sin()
        new_translation.y += velocity.velocity.y * time.delta_seconds(); // *  rotation.rotation_angle.cos()

        if new_translation.x < 210.0 && new_translation.x > -210.0 {
            transform.translation.x = new_translation.x; 
        } 

        if new_translation.y < 285.0 && new_translation.y > -285.0 {
            transform.translation.y = new_translation.y; 
        } 
    }
}