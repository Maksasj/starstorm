use bevy::prelude::*;

use crate::components::{
    onyx_bluster::*,
    friction::*,
    collision::*,
    health::*,
    game_scene_system::*,
};

use crate::resources::{
    sprite_sheet::*,
};

use super::damage_shake::DamageShake;

#[derive(Component)]
pub struct SpikeEnemy {
    pub moving_speed: f32,
    pub amplitude: f32,
}

impl SpikeEnemy {
    pub fn new() -> Self {
        SpikeEnemy { 
            moving_speed: 120.0,
            amplitude: 0.0
        }
    }
}

#[derive(Bundle)]
pub struct SpikeEnemyBundle {
    name: Name,
    sprite_bundle: SpriteSheetBundle,
    health: Health,
    rotation: EntityRotation,
    friction: Friction,
    velocity: Velocity, 
    enemy: SpikeEnemy,
    weapon: OnyxBluster,
    collider: Collider,
    damage_skake: DamageShake,
    game_entity: GameEntity,
}

impl SpikeEnemyBundle {
    pub fn new(asset_server: &Res<SpriteSheet>, pos: Vec2) -> Self {
        let mut sprite = TextureAtlasSprite::new(2);
        sprite.color = Color::rgb(1.0, 1.0, 1.0);
        sprite.custom_size = Some(Vec2::splat(32.0));

        SpikeEnemyBundle { 
            name: Name::new("SpikeEnemy"),
            sprite_bundle: SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: asset_server.handle.clone(),
                transform: Transform { 
                    translation: Vec3::new(pos.x, pos.y, 100.0), 
                    ..Default::default()
                },
                ..Default::default()
            },
            health: Health::new(50.0),
            rotation: EntityRotation::new(_DOWN),
            friction: Friction::new(20.0),
            velocity: Velocity::new(),
            enemy: SpikeEnemy::new(),
            weapon: OnyxBluster::new(),
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

impl Enemy for SpikeEnemy {
    fn move_enemy(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, time: &Res<Time>) {
        self.amplitude += time.delta_seconds();
        
        velocity.velocity.x = self.moving_speed;
        velocity.velocity.y = 0.1 * self.amplitude.sin() * self.moving_speed;
    }
}