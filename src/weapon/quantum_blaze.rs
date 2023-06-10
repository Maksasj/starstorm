use bevy::prelude::*;

use crate::weapon::{
    weapon::*,
    bullet::*,
    bullets::*,
    shooter::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct QuantumBlaze {
    pub timer: f32,
    pub speed: f32,
}

impl QuantumBlaze {
    pub fn default() -> Self {
        QuantumBlaze {
            timer: 0.0,
            speed: 2.5,
        }
    }
}

impl Weapon for QuantumBlaze {
    fn shoot(&mut self, commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>, shooter: &Shooter) {
        self.timer += time.delta_seconds();

        if self.timer > self.speed {
            let handle = asset_server.handle.clone();

            for i in 0..25 {
                spawn_bullet(commands, &handle, QUANTUM_BLAZE_BULLET, start_pos, angle + (i as f32 / 25.0) * 6.28318, shooter);
            }
            
            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}
