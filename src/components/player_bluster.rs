use bevy::prelude::*;

use crate::components::{
    weapon::*,
    bullet::*,
    velocity::*,
    collision::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct PlayerBluster {
    pub timer: f32,
    pub speed: f32,
}

impl PlayerBluster {
    pub fn new() -> Self {
        PlayerBluster {
            timer: 0.0,
            speed: 0.8
        }
    }
}

impl Weapon for PlayerBluster {
    fn shoot(&mut self, mut commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>) {
        self.timer += time.delta_seconds();

        if self.timer > self.speed {
            let handle = asset_server.handle.clone();
            
            spawn_bullet_from_bundle(commands, BulletBundle::custom(                
                &handle, 
                7, 
                start_pos, 
                angle, 
                Velocity::with(0.4, 0.0),
                Collider::new(
                    NONE_COLLISION_LAYER, 
                    ENEMY_COLLISION_LAYER, 
                    Vec2::new(10.0, 10.0)
                )
            ));
            
            self.timer = 0.0;
        }
    }
}
