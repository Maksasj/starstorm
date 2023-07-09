use bevy::prelude::*;

use crate::enemy::{
    enemy::*,
};

use crate::weapon::{
    mortar_bluster::*,
};

use crate::components::{
    friction::*,
    collision::*,
    health::*,
    game_scene_system::*,
    damage_shake::*,
};

use crate::resources::{
    sprite_sheet::*,
};
use crate::weapon::shooter::Shooter;

#[derive(Component)]
pub struct BugEnemy {
    pub moving_speed: f32,
    pub amplitude: f32,
}

impl BugEnemy {
    pub fn new() -> Self {
        BugEnemy { 
            moving_speed: 75.0,
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
    shooter: Shooter,

    collider: Collider,
    damage_shake: DamageShake,
    game_entity: GameEntity,
}

impl BugEnemyBundle {
    pub fn new(asset_server: &Res<SpriteSheet>, pos: Vec2) -> Self {
        let mut sprite = TextureAtlasSprite::new(33);
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
            friction: Friction::new(20.0),
            velocity: Velocity::new(),

            enemy: BugEnemy::new(),
            weapon: MortarBluster::default(),
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

impl Enemy for BugEnemy {
    fn move_enemy(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, time: &Res<Time>) {
        self.amplitude += time.delta_seconds();
        
        velocity.velocity.x = self.amplitude.cos() * self.moving_speed + self.moving_speed;
        velocity.velocity.y = self.amplitude.sin() * self.moving_speed;
    }
}