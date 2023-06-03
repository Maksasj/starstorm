use bevy::prelude::*;

use crate::components::{
    friction::*,
};

#[bevy_trait_query::queryable]
pub trait Enemie {
    fn move_enemie(&mut self, rotation: &mut EntityRotation, velocity: &mut Velocity, time: &Res<Time>);
}

pub fn enemie_moving_system(
        mut targets: Query<(&mut dyn Enemie, &mut EntityRotation, &mut Velocity)>,
        time: Res<Time>
    ) {

    for (enemies, mut rotation, mut velocity) in targets.iter_mut() {
        for mut enemie in enemies {
            enemie.move_enemie(&mut rotation, &mut velocity, &time);
        }
    }
}