use bevy::prelude::*;

use crate::components::{
    enemie::*,
    onyx_bluster::*,
    friction::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct SpikeEnemie {
    pub moving_speed: f32,
    pub amplitude: f32,
}

impl SpikeEnemie {
    pub fn new() -> Self {
        SpikeEnemie { 
            moving_speed: 25.0,
            amplitude: 0.0
        }
    }
}

#[derive(Bundle)]
pub struct SpikeEnemieBundle {
    name: Name,
    rotation: EntityRotation,
    friction: Friction,
    velocity: Velocity, 
    enemie: SpikeEnemie,
    weapon: OnyxBluster,
}

impl SpikeEnemieBundle {
    pub fn new() -> Self {
        SpikeEnemieBundle { 
            name: Name::new("SpikeEnemie"),
            rotation: EntityRotation::new(_DOWN),
            friction: Friction::new(0.97),
            velocity: Velocity::new(),
            enemie: SpikeEnemie::new(),
            weapon: OnyxBluster::new(),
        }
    }
}

impl Enemie for SpikeEnemie {
    fn move_enemie(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, time: &Res<Time>) {
        self.amplitude += time.delta_seconds();
        
        velocity.velocity.x = time.delta_seconds() * self.moving_speed;
        velocity.velocity.y = 0.2 * (self.amplitude * 1.5).sin();
    }
}

pub fn spawn_spike_enemie_system(mut commands: Commands, asset_server: Res<SpriteSheet>) {
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
    .insert(SpikeEnemieBundle::new());
}