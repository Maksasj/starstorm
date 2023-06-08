use bevy::prelude::*;

use crate::weapon::{
    shooter::*,
    dual_starfire_bluster::*,
};

use crate::enemy::{
    enemy::*,
};

use crate::components::{
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
pub struct OrionsFuryEnemy {
    pub moving_speed: f32
}

impl OrionsFuryEnemy {
    pub fn new() -> Self {
        OrionsFuryEnemy { 
            moving_speed: 80.0 
        }
    }
}

#[derive(Bundle)]
pub struct OrionsFuryEnemyBundle {
    name: Name,
    sprite_bundle: SpriteSheetBundle,
    health: Health,
    rotation: EntityRotation,
    friction: Friction,
    velocity: Velocity, 
    
    enemy: OrionsFuryEnemy,
    weapon: DualStarfireBluster,
    shooter: Shooter,

    collider: Collider,
    damage_shake: DamageShake,
    game_entity: GameEntity,
}

impl OrionsFuryEnemyBundle {
    pub fn new(asset_server: &Res<SpriteSheet>, pos: Vec2) -> Self {
        let mut sprite = TextureAtlasSprite::new(129);
        sprite.color = Color::rgb(1.0, 1.0, 1.0);
        sprite.custom_size = Some(Vec2::splat(32.0));

        OrionsFuryEnemyBundle { 
            name: Name::new("OrionsFuryEnemy"),
            sprite_bundle: SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: asset_server.handle.clone(),
                transform: Transform { 
                    translation: Vec3::new(pos.x, pos.y, 100.0), 
                    ..Default::default()
                },
                ..Default::default()
            },
            health: Health::new(65.0),
            rotation: EntityRotation::new(_DOWN),
            friction: Friction::new(20.0),
            velocity: Velocity::new(),
            
            enemy: OrionsFuryEnemy::new(),
            weapon: DualStarfireBluster::default(),
            shooter: Shooter::enemy(),

            collider: Collider::new(
                ENEMY_COLLISION_LAYER, 
                NONE_COLLISION_LAYER, 
                Vec2::new(25.0, 25.0)
            ),
            damage_shake: DamageShake::new(0.0, 0.0, 0.0, false),
            game_entity: GameEntity{},
        }
    }
}

impl Enemy for OrionsFuryEnemy {
    fn move_enemy(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, _time: &Res<Time>) {
        velocity.velocity.x = self.moving_speed;
    }
}