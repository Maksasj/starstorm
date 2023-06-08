use bevy::prelude::*;

use crate::weapon::{
    weapon::*,
    bullet::*,
    shooter::*,
    bullets::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct NovaBurstCannon {
    pub timer: f32,
    pub speed: f32,
}

impl NovaBurstCannon {
    pub fn default() -> Self {
        NovaBurstCannon {
            timer: 0.0,
            speed: 1.2,
        }
    }
}

impl Weapon for NovaBurstCannon {
    fn shoot(&mut self, commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>, shooter: &Shooter) {
        self.timer += time.delta_seconds();

        
        if self.timer > self.speed {
            let handle = asset_server.handle.clone();

            spawn_bullet(commands, &handle, NOVA_PLASMA_BULLET, start_pos, angle, shooter);
            
            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}
