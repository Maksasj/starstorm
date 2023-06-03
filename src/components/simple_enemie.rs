use bevy::prelude::*;

use crate::components::{
    enemie::*,
    simple_bluster::*,
    friction::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct SimpleEnemie {
    pub moving_speed: f32
}

impl SimpleEnemie {
    pub fn new() -> Self {
        SimpleEnemie { 
            moving_speed: 50.0 
        }
    }
}

impl Enemie for SimpleEnemie {
    fn move_enemie(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, time: &Res<Time>) {
        velocity.velocity.x = time.delta_seconds() * self.moving_speed;
    }
}

pub fn spawn_simple_enemie_system(mut commands: Commands, asset_server: Res<SpriteSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(1.0, 1.0, 1.0);
    sprite.custom_size = Some(Vec2::splat(32.0));
    
    commands.spawn(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: asset_server.handle.clone(),
        transform: Transform { 
            translation: Vec3::new(0.0, 0.0, 900.0), 
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("Player"))
    .insert(SimpleEnemie::new())
    .insert(EntityRotation{ rotation_angle: _DOWN, rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0) })
    .insert(Friction{ rate: 0.97 })
    .insert(SimpleBluster::new())
    .insert(Velocity{ velocity: Vec2::new(0.0, 0.0) });
}