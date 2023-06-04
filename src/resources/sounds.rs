use bevy::prelude::*;

use bevy_kira_audio::{
    prelude::*,
    Audio, 
    AudioSource
};

#[derive(Resource)]
pub struct Sounds {
    pub hurt_handle: Handle<AudioSource>,
}

pub struct SoundEvent {
    pub handle: Handle<AudioSource>,
}

pub fn load_sounds_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hurt_sound: Handle<AudioSource> = asset_server.load("sounds/hurt_sound.wav");

    let sounds = Sounds {
        hurt_handle: hurt_sound,
    };

    commands.insert_resource(sounds);
}

pub fn handle_sounds(audio: Res<Audio>, mut events: EventReader<SoundEvent>) {
    for event in events.iter() {
        audio.play(event.handle.clone());
    }

    events.clear();
}