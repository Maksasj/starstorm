use bevy::prelude::*;

use crate::components::{
    enemie::*,
    friction::*,
    mortar_bluster::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct BugEnemie {
    pub moving_speed: f32,
    pub amplitude: f32,
}

impl BugEnemie {
    pub fn new() -> Self {
        BugEnemie { 
            moving_speed: 25.0,
            amplitude: 0.0
        }
    }
}

#[derive(Bundle)]
pub struct BugEnemieBundle {
    name: Name,
    rotation: EntityRotation,
    friction: Friction,
    velocity: Velocity, 
    enemie: BugEnemie,
    weapon: MortarBluster,
}

impl BugEnemieBundle {
    pub fn new() -> Self {
        BugEnemieBundle { 
            name: Name::new("BugEnemie"),
            rotation: EntityRotation::new(_DOWN),
            friction: Friction::new(0.97),
            velocity: Velocity::new(),
            enemie: BugEnemie::new(),
            weapon: MortarBluster::new(),
        }
    }
}

impl Enemie for BugEnemie {
    fn move_enemie(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, time: &Res<Time>) {
        self.amplitude += time.delta_seconds();
        
        velocity.velocity.x = 0.2 * (self.amplitude * 1.5).cos() + time.delta_seconds() * self.moving_speed;
        velocity.velocity.y = 0.2 * (self.amplitude * 1.5).sin();
    }
}

pub fn spawn_bug_enemie_system(mut commands: Commands, asset_server: Res<SpriteSheet>) {
    let mut sprite = TextureAtlasSprite::new(3);
    sprite.color = Color::rgb(1.0, 1.0, 1.0);
    sprite.custom_size = Some(Vec2::splat(32.0));
    
    commands.spawn(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: asset_server.handle.clone(),
        transform: Transform { 
            translation: Vec3::new(100.0, 500.0, 900.0), 
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(BugEnemieBundle::new());
}