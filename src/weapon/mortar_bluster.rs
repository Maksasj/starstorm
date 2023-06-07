use bevy::prelude::*;

use crate::weapon::{
    weapon::*,
    bullet::*,
};

use crate::components::{
    velocity::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct MortarBluster {
    pub timer: f32,
    pub speed: f32,
    pub damage: f32,
}

impl MortarBluster {
    pub fn default() -> Self {
        MortarBluster {
            timer: 0.0,
            speed: 3.5,
            damage: 70.0,
        }
    }
}

impl Weapon for MortarBluster {
    fn shoot(&mut self, mut commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>) {
        self.timer += time.delta_seconds();

        if self.timer > self.speed {
            let handle = asset_server.handle.clone();
            
            for i in -2..2 {
                spawn_bullet(
                    &mut commands, 
                    &handle, 
                    35, 
                    start_pos, 
                    angle + 0.2617 * i as f32, 
                    Velocity::with(50.0, 0.0), 
                    self.damage
                );
            }
            
            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}
