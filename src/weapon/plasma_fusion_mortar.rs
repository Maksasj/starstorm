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
pub struct PlasmaFusionMortar {
    pub timer: f32,
    pub speed: f32,
}

impl PlasmaFusionMortar {
    pub fn default() -> Self {
        PlasmaFusionMortar {
            timer: 0.0,
            speed: 1.6,
        }
    }
}

impl Weapon for PlasmaFusionMortar {
    fn shoot(&mut self, commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>, shooter: &Shooter) {
        self.timer += time.delta_seconds();

        if self.timer > self.speed {
            let handle = asset_server.handle.clone();
            
            for i in -3..4 {
                spawn_bullet(commands, &handle, PLASMA_FUSION_BURST_BULLET, start_pos, angle + 0.314159 * i as f32, shooter);
            }
            
            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}
