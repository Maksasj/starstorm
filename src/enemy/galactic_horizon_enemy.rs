use bevy::prelude::*;

use crate::weapon::{
    shooter::*,
    dual_rapid_space_blast_cannon::*,
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
pub struct GalacticHorizonEnemy {
    pub moving_speed: f32
}

impl GalacticHorizonEnemy {
    pub fn new() -> Self {
        GalacticHorizonEnemy { 
            moving_speed: 140.0 
        }
    }
}

#[derive(Bundle)]
pub struct GalacticHorizonEnemyBundle {
    name: Name,
    sprite_bundle: SpriteSheetBundle,
    health: Health,
    rotation: EntityRotation,
    friction: Friction,
    velocity: Velocity, 
    
    enemy: GalacticHorizonEnemy,
    weapon: DualRapidSpaceBlastCannon,
    shooter: Shooter,

    collider: Collider,
    damage_shake: DamageShake,
    game_entity: GameEntity,
}

impl GalacticHorizonEnemyBundle {
    pub fn new(asset_server: &Res<SpriteSheet>, pos: Vec2) -> Self {
        let mut sprite = TextureAtlasSprite::new(97);
        sprite.color = Color::rgb(1.0, 1.0, 1.0);
        sprite.custom_size = Some(Vec2::splat(32.0));

        GalacticHorizonEnemyBundle { 
            name: Name::new("GalacticHorizonEnemy"),
            sprite_bundle: SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: asset_server.handle.clone(),
                transform: Transform { 
                    translation: Vec3::new(pos.x, pos.y, 100.0), 
                    ..Default::default()
                },
                ..Default::default()
            },
            health: Health::new(45.0),
            rotation: EntityRotation::new(_DOWN),
            friction: Friction::new(20.0),
            velocity: Velocity::new(),
            
            enemy: GalacticHorizonEnemy::new(),
            weapon: DualRapidSpaceBlastCannon::default(),
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

impl Enemy for GalacticHorizonEnemy {
    fn move_enemy(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, _time: &Res<Time>) {
        velocity.velocity.x = self.moving_speed;
    }
}