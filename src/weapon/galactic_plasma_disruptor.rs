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
pub struct GalacticPlasmaDisruptor {
    pub timer: f32,
    pub speed: f32,
}

impl GalacticPlasmaDisruptor {
    pub fn default() -> Self {
        GalacticPlasmaDisruptor {
            timer: 0.0,
            speed: 1.2,
        }
    }
}

impl Weapon for GalacticPlasmaDisruptor {
    fn shoot(&mut self, commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>, shooter: &Shooter) {
        self.timer += time.delta_seconds();

        
        if self.timer > self.speed {
            let handle = asset_server.handle.clone();

            spawn_bullet(commands, &handle, GALACTIC_PLASMA_BULLET, start_pos, angle, shooter);
            spawn_bullet(commands, &handle, GALACTIC_PLASMA_BULLET, start_pos, angle + 1.5708, shooter);
            spawn_bullet(commands, &handle, GALACTIC_PLASMA_BULLET, start_pos, angle - 1.5708, shooter);
            spawn_bullet(commands, &handle, GALACTIC_PLASMA_BULLET, start_pos, angle + 3.1416, shooter);
            
            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}
