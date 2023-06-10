use bevy::prelude::*;

use crate::resources::{
    sounds::*,
};

pub struct PlayerShootEvent;

pub fn player_shoot_system(
    mut player_shoot_event_reader: EventReader<PlayerShootEvent>,
    mut sound_event_writer: EventWriter<SoundEvent>,
    sounds: Res<Sounds>
) {

for _event in player_shoot_event_reader.iter() {
    sound_event_writer.send(SoundEvent{handle: sounds.shoot_handle.clone()});
}
}