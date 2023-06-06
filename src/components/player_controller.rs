use bevy::prelude::*;

use crate::components::{
    friction::*,
};

#[derive(Component)]
pub struct PlayerController;

pub fn player_controller_system(
        input: Res<Input<KeyCode>>, 
        mut targets: Query<&mut Velocity, With<PlayerController>>,
    ) {
    
    if input.pressed(KeyCode::W) {
        for mut velocity in targets.iter_mut() {
            velocity.velocity.y = 150.0;
        }
    }

    if input.pressed(KeyCode::S) {
        for mut velocity in targets.iter_mut() {
            velocity.velocity.y = -150.0;
        }
    }

    if input.pressed(KeyCode::A) {
        for mut velocity in targets.iter_mut() {
            velocity.velocity.x = -150.0;
        }
    }

    if input.pressed(KeyCode::D) {
        for mut velocity in targets.iter_mut() {
            velocity.velocity.x = 150.0;
        }
    }
}
