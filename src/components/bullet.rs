use bevy::prelude::*;

use crate::components::{
    friction::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct Bullet {
    pub life_time: f32,
    timer: f32
}

impl Bullet {
    pub fn new(life_time: f32) -> Self {
        Bullet {
            life_time: life_time,
            timer: 0.0
        }
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    name: Name,
    rotation: EntityRotation,
    bullet: Bullet,
    velocity: Velocity, 
}

impl BulletBundle {
    pub fn new(angle: f32) -> Self {
        BulletBundle { 
            name: Name::new("Bullet"),
            rotation: EntityRotation::new(angle),
            bullet: Bullet::new(10.0),
            velocity: Velocity::with(0.4, 0.0),
        }
    }
}

pub fn spawn_bullet(commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2) {
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
    .insert(BulletBundle::new(angle));
}

pub fn bullet_life_time_system(mut commands: Commands, mut targets: Query<(Entity, &mut Bullet)>, time: Res<Time>) {
    for (entity, mut bullet) in targets.iter_mut() {
        bullet.timer += time.delta_seconds();
        
        if bullet.timer > bullet.life_time {
            commands.entity(entity).despawn_recursive();
        }
    }
}