use bevy::prelude::*;

use crate::components::{
    friction::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct Bullet {

}

pub fn spawn_bullet(mut commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2) {
    let mut sprite = TextureAtlasSprite::new(4);
    sprite.color = Color::rgb(1.0, 1.0, 1.0);
    sprite.custom_size = Some(Vec2::splat(32.0));
    
    commands.spawn(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: asset_server.handle.clone(),
        transform: Transform { 
            translation: Vec3::new(start_pos.x, start_pos.y, 900.0), 
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("Bullet"))
    .insert(EntityRotation{ rotation_angle: angle, rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0) })
    .insert(Bullet{})
    .insert(Velocity{ velocity: Vec2::new(1.0, 0.0) });
}