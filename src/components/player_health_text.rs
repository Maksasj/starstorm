use bevy::prelude::*;

pub use crate::components::{
    entity_rotation::*,
    player_controller::*,
};

pub use crate::resources::{
    small_numbers_font::*
};

use super::health::Health;

#[derive(Component)]
pub struct PlayerHealthText {
    id: usize
}

impl PlayerHealthText {
    pub fn new(id: usize) -> Self {
        PlayerHealthText { 
            id: id,
        }
    }
}

pub fn spawn_player_health_text_system(mut commands: Commands, asset_server: Res<SmallNumberFontSpriteSheet>) {
    let mut childrens = Vec::new();

    for i in 0..7 {
        let mut sprite;
        
        if i == 3 {
            sprite = TextureAtlasSprite::new(10);
        } else {
            sprite = TextureAtlasSprite::new(0);
        }

        sprite.color = Color::rgb(1.0, 1.0, 1.0);
        sprite.custom_size = Some(Vec2::splat(24.0));
        
        let character = commands.spawn(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: asset_server.handle.clone(),
            transform: Transform { 
                translation: Vec3::new(0.0 + (i as f32) * 15.0, 0.0, 900.0), 
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("PlayerHealthText"))
        .insert(Visibility::Visible)
        .insert(PlayerHealthText::new(i)).id();

        childrens.push(character);
    }

    commands
        .spawn(VisibilityBundle::default())
        .insert(Name::new("PlayerHealthTextParent"))
        .insert(Transform { 
            translation: Vec3::new(-210.0, 250.0, 0.0), 
            ..Default::default()
        })
        .insert(GlobalTransform::default())
        .push_children(&childrens);
}

pub fn player_helth_text_update_system(
        players: Query<&Health, With<PlayerController>>, 
        mut health_texts: Query<(&PlayerHealthText, &mut TextureAtlasSprite, &mut Visibility)>
    ) {

    for player_health in players.iter() {
        let health: i32 = player_health.value as i32;
        let health_max: i32 = player_health.max_value as i32;

        if health < 0 { continue; }

        let mut health_dig: [usize; 3] = [0, 0, 0];
        health_dig[0] = ((health / 100) % 10) as usize;
        health_dig[1] = ((health / 10) % 10) as usize;
        health_dig[2] = (health % 10) as usize;

        let mut health_max_dig: [usize; 3] = [0, 0, 0];
        health_max_dig[0] = ((health_max / 100) % 10) as usize;
        health_max_dig[1] = ((health_max / 10) % 10) as usize;
        health_max_dig[2] = (health_max % 10) as usize;

        for (health_text, mut text_sprite, visibility) in health_texts.iter_mut() {
            if health_text.id < 3 {
                text_sprite.index = health_dig[health_text.id];
            } else if health_text.id > 3 {
                text_sprite.index = health_max_dig[health_text.id - 4];
            }
        }
    }
}