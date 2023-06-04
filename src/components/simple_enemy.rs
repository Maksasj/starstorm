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
            moving_speed: 50.0 
        }
    }
}

#[derive(Bundle)]
pub struct SimpleEnemyBundle {
    name: Name,
    health: Health,
    rotation: EntityRotation,
    friction: Friction,
    velocity: Velocity, 
    enemy: SimpleEnemy,
    weapon: SimpleBluster,
    collider: Collider,
    damage_skake: DamageShake,
}

impl SimpleEnemyBundle {
    pub fn new() -> Self {
        SimpleEnemyBundle { 
            name: Name::new("SimpleEnemy"),
            health: Health::new(80.0),
            rotation: EntityRotation::new(_DOWN),
            friction: Friction::new(0.97),
            velocity: Velocity::new(),
            enemy: SimpleEnemy::new(),
            weapon: SimpleBluster::new(),
            collider: Collider::new(
                ENEMY_COLLISION_LAYER, 
                NONE_COLLISION_LAYER, 
                Vec2::new(25.0, 25.0)
            ),
            damage_skake: DamageShake::new(0.0, 0.0, 0.0, false),
        }
    }
}

impl Enemy for SimpleEnemy {
    fn move_enemy(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, time: &Res<Time>) {
        velocity.velocity.x = time.delta_seconds() * self.moving_speed;
    }
}

pub fn spawn_simple_enemy_system(mut commands: Commands, asset_server: Res<SpriteSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(1.0, 1.0, 1.0);
    sprite.custom_size = Some(Vec2::splat(32.0));
    
    commands.spawn(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: asset_server.handle.clone(),
        transform: Transform { 
            translation: Vec3::new(0.0, 500.0, 900.0), 
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SimpleEnemyBundle::new())
    .insert(GameEntity{});
}