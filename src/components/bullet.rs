use bevy::prelude::*;

use crate::{components::{
    friction::*,
    collision::*,
}, Collider};

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
    pub sprite: SpriteSheetBundle,
    pub name: Name,
    pub rotation: EntityRotation,
    pub bullet: Bullet,
    pub velocity: Velocity, 
    pub collider: ColliderBundle,
}

impl BulletBundle {
    pub fn new(asset_handle: &Handle<TextureAtlas>, sprite_index: usize, start_pos: Vec2, angle: f32, velocity: Velocity) -> Self {
        let mut sprite: TextureAtlasSprite = TextureAtlasSprite::new(sprite_index);
        sprite.color = Color::rgb(1.0, 1.0, 1.0);
        sprite.custom_size = Some(Vec2::splat(32.0));

        BulletBundle { 
            sprite: SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: asset_handle.clone(),
                transform: Transform { 
                    translation: Vec3::new(start_pos.x, start_pos.y, 900.0), 
                    ..Default::default()
                },
                ..Default::default()
            },
            name: Name::new("Bullet"),
            rotation: EntityRotation::new(angle),
            bullet: Bullet::new(10.0),
            velocity: velocity,
            collider: ColliderBundle::new(BULLET_COLLISION_LAYER, PLAYER_COLLISION_LAYER),
        }
    }
}

pub fn spawn_bullet(
        commands: &mut Commands, 
        asset_handle: &Handle<TextureAtlas>, 
        sprite_index: usize,
        start_pos: Vec2,
        angle: f32, 
        velocity: Velocity,
    ) {

    commands.spawn(BulletBundle::new(asset_handle, sprite_index, start_pos, angle, velocity));
}

pub fn bullet_life_time_system(mut commands: Commands, mut targets: Query<(Entity, &mut Bullet)>, time: Res<Time>) {
    for (entity, mut bullet) in targets.iter_mut() {
        bullet.timer += time.delta_seconds();
        
        if bullet.timer > bullet.life_time {
            commands.entity(entity).despawn_recursive();
        }
    }
}