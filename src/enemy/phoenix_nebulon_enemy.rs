use bevy::prelude::*;

use crate::weapon::{
    shooter::*,
    ionized_pulse_blaster::*,
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
pub struct PhoenixNebulonEnemy {
    pub moving_speed: f32
}

impl PhoenixNebulonEnemy {
    pub fn new() -> Self {
        PhoenixNebulonEnemy { 
            moving_speed: 75.0 
        }
    }
}

#[derive(Bundle)]
pub struct PhoenixNebulonEnemyBundle {
    name: Name,
    sprite_bundle: SpriteSheetBundle,
    health: Health,
    rotation: EntityRotation,
    friction: Friction,
    velocity: Velocity, 
    
    enemy: PhoenixNebulonEnemy,
    weapon: IonizedPulseBlaster,
    shooter: Shooter,

    collider: Collider,
    damage_shake: DamageShake,
    game_entity: GameEntity,
}

impl PhoenixNebulonEnemyBundle {
    pub fn new(asset_server: &Res<SpriteSheet>, pos: Vec2) -> Self {
        let mut sprite = TextureAtlasSprite::new(241);
        sprite.color = Color::rgb(1.0, 1.0, 1.0);
        sprite.custom_size = Some(Vec2::splat(32.0));

        PhoenixNebulonEnemyBundle { 
            name: Name::new("PhoenixNebulonEnemy"),
            sprite_bundle: SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: asset_server.handle.clone(),
                transform: Transform { 
                    translation: Vec3::new(pos.x, pos.y, 100.0), 
                    ..Default::default()
                },
                ..Default::default()
            },
            health: Health::new(60.0),
            rotation: EntityRotation::new(_DOWN),
            friction: Friction::new(20.0),
            velocity: Velocity::new(),
            
            enemy: PhoenixNebulonEnemy::new(),
            weapon: IonizedPulseBlaster::default(),
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

impl Enemy for PhoenixNebulonEnemy {
    fn move_enemy(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, _time: &Res<Time>) {
        velocity.velocity.x = self.moving_speed;
    }
}