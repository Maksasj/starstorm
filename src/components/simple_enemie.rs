use bevy::prelude::*;

use crate::components::{
    enemie::*,
    simple_bluster::*,
    friction::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct SimpleEnemie {
    pub moving_speed: f32
}

impl SimpleEnemie {
    pub fn new() -> Self {
        SimpleEnemie { 
            moving_speed: 50.0 
        }
    }
}

#[derive(Bundle)]
pub struct SimpleEnemieBundle {
    name: Name,
    rotation: EntityRotation,
    friction: Friction,
    velocity: Velocity, 
    enemie: SimpleEnemie,
    weapon: SimpleBluster,
}

impl SimpleEnemieBundle {
    pub fn new() -> Self {
        SimpleEnemieBundle { 
            name: Name::new("SimpleEnemie"),
            rotation: EntityRotation::new(_DOWN),
            friction: Friction::new(0.97),
            velocity: Velocity::new(),
            enemie: SimpleEnemie::new(),
            weapon: SimpleBluster::new(),
        }
    }
}

impl Enemie for SimpleEnemie {
    fn move_enemie(&mut self, _rotation: &mut EntityRotation, velocity: &mut Velocity, time: &Res<Time>) {
        velocity.velocity.x = time.delta_seconds() * self.moving_speed;
    }
}

pub fn spawn_simple_enemie_system(mut commands: Commands, asset_server: Res<SpriteSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(1.0, 1.0, 1.0);
    sprite.custom_size = Some(Vec2::splat(32.0));
    
    commands.spawn(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: asset_server.handle.clone(),
        transform: Transform { 
            translation: Vec3::new(0.0, 0.0, 900.0), 
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(SimpleEnemieBundle::new());
}