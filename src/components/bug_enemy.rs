use bevy::prelude::*;

use crate::components::{
    friction::*,
    mortar_bluster::*,
    collision::*,
    health::*,
    game_scene_system::*,
};

use crate::resources::{
    sprite_sheet::*,
};

use super::damage_shake::DamageShake;

#[derive(Component)]
pub struct BugEnemy {
    pub moving_speed: f32,
    pub amplitude: f32,
}

impl BugEnemy {
    pub fn new() -> Self {
        BugEnemy { 
            moving_speed: 25.0,
            amplitude: 0.0
        }
    }
}

#[derive(Bundle)]
pub struct BugEnemyBundle {
    name: Name,
    sprite_bundle: SpriteSheetBundle,
    health: Health,
    rotation: EntityRotation,
    friction: Friction,
    velocity: Velocity, 
    enemy: BugEnemy,
    weapon: MortarBluster,
    collider: Collider,
    damage_skake: DamageShake,
    game_entity: GameEntity,
}

impl BugEnemyBundle {
    pub fn new(asset_server: &Res<SpriteSheet>, pos: Vec2) -> Self {
        let mut sprite = TextureAtlasSprite::new(3);
        sprite.color = Color::rgb(1.0, 1.0, 1.0);
        sprite.custom_size = Some(Vec2::splat(32.0));

        BugEnemyBundle { 
            name: Name::new("BugEnemy"),
            sprite_bundle: SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: asset_server.handle.clone(),
                transform: Transform { 
                    translation: Vec3::new(pos.x, pos.y, 100.0), 
                    ..Default::default()
                },
                ..Default::default()
            },
            health: Health::new(90.0),
            rotation: EntityRotation::new(_DOWN),
            friction: Friction::new(0.97),
            velocity: Velocity::new(),
            enemy: BugEnemy::new(),
            weapon: MortarBluster::new(),
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

impl Enemy for BugEnemy {
    fn move_enemy(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, time: &Res<Time>) {
        self.amplitude += time.delta_seconds();
        
        velocity.velocity.x = 0.2 * (self.amplitude * 1.5).cos() + time.delta_seconds() * self.moving_speed;
        velocity.velocity.y = 0.2 * (self.amplitude * 1.5).sin();
    }
}