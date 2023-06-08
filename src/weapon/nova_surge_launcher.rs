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
pub struct NovaSurgeLauncher {
    pub timer: f32,
    pub speed: f32,
}

impl NovaSurgeLauncher {
    pub fn default() -> Self {
        NovaSurgeLauncher {
            timer: 0.0,
            speed: 1.2,
        }
    }
}

impl Weapon for NovaSurgeLauncher {
    fn shoot(&mut self, commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>, shooter: &Shooter) {
        self.timer += time.delta_seconds();

        if self.timer > self.speed {
            let handle = asset_server.handle.clone();
            
            spawn_bullet(commands, &handle, NOVA_SURGE_BULLET, start_pos, angle + 2.0944, shooter);
            spawn_bullet(commands, &handle, NOVA_SURGE_BULLET, start_pos, angle - 2.0944, shooter);

            spawn_bullet(commands, &handle, NOVA_SURGE_BULLET, start_pos, angle + 0.261799, shooter);
            spawn_bullet(commands, &handle, NOVA_SURGE_BULLET, start_pos, angle, shooter);
            spawn_bullet(commands, &handle, NOVA_SURGE_BULLET, start_pos, angle - 0.261799, shooter);
            
            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}
