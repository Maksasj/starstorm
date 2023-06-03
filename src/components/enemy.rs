use bevy::prelude::*;

use crate::components::{
    friction::*,
};

#[bevy_trait_query::queryable]
pub trait Enemy {
    fn move_enemy(&mut self, rotation: &mut EntityRotation, velocity: &mut Velocity, time: &Res<Time>);
}

pub fn enemy_moving_system(
        mut targets: Query<(&mut dyn Enemy, &mut EntityRotation, &mut Velocity)>,
        time: Res<Time>
    ) {

    for (enemys, mut rotation, mut velocity) in targets.iter_mut() {
        for mut enemy in enemys {
            enemy.move_enemy(&mut rotation, &mut velocity, &time);
        }
    }
}