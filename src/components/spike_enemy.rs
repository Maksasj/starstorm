use bevy::prelude::*;

use crate::components::{
    enemy::*,
    onyx_bluster::*,
    friction::*,
    collision::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct SpikeEnemy {
    pub moving_speed: f32,
    pub amplitude: f32,
}

impl SpikeEnemy {
    pub fn new() -> Self {
        SpikeEnemy { 
            moving_speed: 25.0,
            amplitude: 0.0
        }
    }
}

#[derive(Bundle)]
pub struct SpikeEnemyBundle {
    name: Name,
    rotation: EntityRotation,
    friction: Friction,
    velocity: Velocity, 
    enemy: SpikeEnemy,
    weapon: OnyxBluster,
    collider: Collider,
}

impl SpikeEnemyBundle {
    pub fn new() -> Self {
        SpikeEnemyBundle { 
            name: Name::new("SpikeEnemy"),
            rotation: EntityRotation::new(_DOWN),
            friction: Friction::new(0.97),
            velocity: Velocity::new(),
            enemy: SpikeEnemy::new(),
            weapon: OnyxBluster::new(),
            collider: Collider::new(
                ENEMY_COLLISION_LAYER, 
                NONE_COLLISION_LAYER, 
                Vec2::new(25.0, 25.0)
            ),
        }
    }
}

impl Enemy for SpikeEnemy {
    fn move_enemy(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, time: &Res<Time>) {
        self.amplitude += time.delta_seconds();
        
        velocity.velocity.x = time.delta_seconds() * self.moving_speed;
        velocity.velocity.y = 0.2 * (self.amplitude * 1.5).sin();
    }
}

pub fn spawn_spike_enemy_system(mut commands: Commands, asset_server: Res<SpriteSheet>) {
    let mut sprite = TextureAtlasSprite::new(2);
    sprite.color = Color::rgb(1.0, 1.0, 1.0);
    sprite.custom_size = Some(Vec2::splat(32.0));
    
    commands.spawn(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: asset_server.handle.clone(),
        transform: Transform { 
            translation: Vec3::new(-100.0, 500.0, 900.0), 
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SpikeEnemyBundle::new());
}