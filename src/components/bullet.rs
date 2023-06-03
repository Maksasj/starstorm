use bevy::prelude::*;

use crate::components::{
    friction::*,
    collision::*,
};

#[derive(Component)]
pub struct Bullet {
    pub life_time: f32,
    timer: f32,
    pub damage: f32,
}

impl Bullet {
    pub fn new(life_time: f32, damage: f32) -> Self {
        Bullet {
            life_time: life_time,
            timer: 0.0,
            damage: damage,
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
    pub collider: Collider,
}

impl BulletBundle {
    pub fn new(asset_handle: &Handle<TextureAtlas>, sprite_index: usize, start_pos: Vec2, angle: f32, velocity: Velocity, damage: f32) -> Self {
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
            bullet: Bullet::new(10.0, damage),
            velocity: velocity,
            collider: Collider::new(
                BULLET_COLLISION_LAYER, 
                PLAYER_COLLISION_LAYER, 
                Vec2::new(10.0, 10.0)
            ),
        }
    }

    pub fn custom(asset_handle: &Handle<TextureAtlas>, sprite_index: usize, start_pos: Vec2, angle: f32, velocity: Velocity, collider: Collider, damage: f32) -> Self {
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
            bullet: Bullet::new(10.0, damage),
            velocity: velocity,
            collider: collider,
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
        damage: f32
    ) {

    commands.spawn(BulletBundle::new(asset_handle, sprite_index, start_pos, angle, velocity, damage));
}

pub fn spawn_bullet_from_bundle(commands: &mut Commands, bullet: BulletBundle) {
    commands.spawn(bullet);
}

pub fn bullet_life_time_system(mut commands: Commands, mut targets: Query<(Entity, &mut Bullet, &Transform)>, time: Res<Time>) {
    for (entity, mut bullet, transform) in targets.iter_mut() {
        bullet.timer += time.delta_seconds();
        
        if bullet.timer > bullet.life_time {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        
        if transform.translation.x > 225.0 ||  transform.translation.x < -225.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}