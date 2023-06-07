use bevy::prelude::*;

use crate::weapon::{
    shooter::*,
};

use crate::components::{
    friction::*,
    collision::*,
    game_scene_system::*,
};

#[derive(Component)]
pub struct Bullet {
    pub life_time: f32,
    pub damage: f32,
    
    pub sprite_index: usize,

    timer: f32,
}

impl Bullet {
    pub const fn new(life_time: f32, damage: f32, sprite_index: usize) -> Self {
        Bullet {
            life_time: life_time,
            damage: damage,
            sprite_index: sprite_index,

            timer: 0.0,
        }
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub velocity: Velocity, 
    pub collision_box: CollisionBox,
}

pub fn spawn_bullet(
        commands: &mut Commands, 
        asset_handle: &Handle<TextureAtlas>, 
        bullet: BulletBundle,
        start_pos: Vec2,
        angle: f32, 
        shooter: &Shooter
    ) {

    let mut sprite: TextureAtlasSprite = TextureAtlasSprite::new(bullet.bullet.sprite_index);
    sprite.color = Color::rgb(1.0, 1.0, 1.0);
    sprite.custom_size = Some(Vec2::splat(32.0));

    let entity = commands
        .spawn(bullet)
        .insert(SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: asset_handle.clone(),
                transform: Transform { 
                    translation: Vec3::new(start_pos.x, start_pos.y, 100.0), 
                    ..Default::default()
                },
                ..Default::default()
            })
        .insert(EntityRotation::new(angle))
        .insert(GameEntity{}).id();

    match shooter.0 {
        ShooterType::Player => {
            commands.entity(entity).insert(Collider::new(
                BULLET_COLLISION_LAYER, 
                ENEMY_COLLISION_LAYER, 
                Vec2::new(10.0, 10.0)
            ));
        },
        ShooterType::Enemy => {
            commands.entity(entity).insert(Collider::new(
                BULLET_COLLISION_LAYER, 
                PLAYER_COLLISION_LAYER, 
                Vec2::new(10.0, 10.0)
            ));
        },
    }
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