use bevy::prelude::*;

use crate::components::{
    simple_bluster::*,
    friction::*,
    collision::*,
    health::*,
    damage_shake::*,
    game_scene_system::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct SimpleEnemy {
    pub moving_speed: f32
}

impl SimpleEnemy {
    pub fn new() -> Self {
        SimpleEnemy { 
            moving_speed: 400.0 
        }
    }
}

#[derive(Bundle)]
pub struct SimpleEnemyBundle {
    name: Name,
    sprite_bundle: SpriteSheetBundle,
    health: Health,
    rotation: EntityRotation,
    friction: Friction,
    velocity: Velocity, 
    enemy: SimpleEnemy,
    weapon: SimpleBluster,
    collider: Collider,
    damage_skake: DamageShake,
    game_entity: GameEntity,
}

impl SimpleEnemyBundle {
    pub fn new(asset_server: &Res<SpriteSheet>, pos: Vec2) -> Self {
        let mut sprite = TextureAtlasSprite::new(1);
        sprite.color = Color::rgb(1.0, 1.0, 1.0);
        sprite.custom_size = Some(Vec2::splat(32.0));

        SimpleEnemyBundle { 
            name: Name::new("SimpleEnemy"),
            sprite_bundle: SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: asset_server.handle.clone(),
                transform: Transform { 
                    translation: Vec3::new(pos.x, pos.y, 100.0), 
                    ..Default::default()
                },
                ..Default::default()
            },
            health: Health::new(80.0),
            rotation: EntityRotation::new(_DOWN),
            friction: Friction::new(0.999),
            velocity: Velocity::new(),
            enemy: SimpleEnemy::new(),
            weapon: SimpleBluster::new(),
            collider: Collider::new(
                ENEMY_COLLISION_LAYER, 
                NONE_COLLISION_LAYER, 
                Vec2::new(25.0, 25.0)
            ),
            damage_skake: DamageShake::new(0.0, 0.0, 0.0, false),
            game_entity: GameEntity{},
        }
    }
}

impl Enemy for SimpleEnemy {
    fn move_enemy(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, _time: &Res<Time>) {
        velocity.velocity.x = self.moving_speed;
    }
}