use bevy::prelude::*;

use bevy_kira_audio::{
    prelude::*,
    Audio, 
    AudioSource
};

#[derive(Resource)]
pub struct Sounds {
    pub hurt_handle: Handle<AudioSource>,
    pub death_handle: Handle<AudioSource>,
    pub shoot_handle: Handle<AudioSource>,
    pub hit_handle: Handle<AudioSource>,
}

pub struct SoundEvent {
    pub handle: Handle<AudioSource>,
}

pub fn load_sounds_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hurt_sound: Handle<AudioSource> = asset_server.load("sounds/hurt_sound.wav");
    let death_sound: Handle<AudioSource> = asset_server.load("sounds/death.wav");
    let shoot_sound: Handle<AudioSource> = asset_server.load("sounds/shoot.wav");
    let hit_sound: Handle<AudioSource> = asset_server.load("sounds/hit_sound.wav");

    let sounds = Sounds {
        hurt_handle: hurt_sound,
        death_handle: death_sound,
        shoot_handle: shoot_sound,
        hit_handle: hit_sound,
    };

    commands.insert_resource(sounds);
}

pub fn handle_sounds(audio: Res<Audio>, mut events: EventReader<SoundEvent>) {
    for event in events.iter() {
        audio.play(event.handle.clone());
    }

    events.clear();
}