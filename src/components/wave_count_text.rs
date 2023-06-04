use bevy::prelude::*;

pub use crate::components::{
    entity_rotation::*,
    player_controller::*,
    health::*,
    game_scene_system::*,
    wave_system::*,
};

pub use crate::resources::{
    sprite_sheet::*,
    big_numbers_font::*,
};

#[derive(Component)]
pub struct WaveCountText {
    id: usize
}

impl WaveCountText {
    pub fn new(id: usize) -> Self {
        WaveCountText { 
            id: id,
        }
    }
}

pub fn spawn_wave_count_text_system(mut commands: Commands, asset_server: Res<BigNumberFontSpriteSheet>) {
    let mut childrens = Vec::new();

    for i in 0..2 {
        let mut sprite = TextureAtlasSprite::new(0);
        sprite.color = Color::rgb(1.0, 1.0, 1.0);
        sprite.custom_size = Some(Vec2::splat(48.0));
        
        let character = commands.spawn(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: asset_server.handle.clone(),
            transform: Transform { 
                translation: Vec3::new(0.0 + (i as f32) * 22.0, 0.0, 100.0), 
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("WaveCountText"))
        .insert(Visibility::Visible)
        .insert(WaveCountText::new(i))
        .insert(GameEntity).id();

        childrens.push(character);
    }

    commands
        .spawn(VisibilityBundle::default())
        .insert(Name::new("WaveCountTextParent"))
        .insert(Transform { 
            translation: Vec3::new(35.0, 279.0, 0.0), 
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .push_children(&childrens);
}

pub fn wave_text_update_system(
        mut events: EventReader<WaveSwitchEvent>, 
        mut wave_texts: Query<(&WaveCountText, &mut TextureAtlasSprite)>
    ) {

    for event in events.iter() {
        let wave = event.to;

        let mut health_dig: [usize; 2] = [0, 0];
        health_dig[0] = ((wave / 10) % 10) as usize;
        health_dig[1] = (wave % 10) as usize;

        for (wave_text, mut text_sprite) in wave_texts.iter_mut() {
            text_sprite.index = health_dig[wave_text.id];
        }
    }
}