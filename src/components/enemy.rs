use bevy::prelude::*;

use crate::components::{
    friction::*,
    health::*,
};

use crate::resources::{
    sounds::*,
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

pub fn enemy_death_system(
        mut commands: Commands, 
        enemies: Query<(Entity, &Health, &dyn Enemy)>,
        mut sound_event_writer: EventWriter<SoundEvent>,
        sounds: Res<Sounds>
    ) {

    for (entity, health, _enemy) in enemies.iter() {
        if health.is_dead() {
            commands.entity(entity).despawn_recursive();
            sound_event_writer.send(SoundEvent{handle: sounds.death_handle.clone()});
        }
    }
}