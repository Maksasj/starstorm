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
pub struct VoidEnergySlicer {
    pub timer: f32,
    pub speed: f32,
}

impl VoidEnergySlicer {
    pub fn default() -> Self {
        VoidEnergySlicer {
            timer: 0.0,
            speed: 1.1,
        }
    }
}

impl Weapon for VoidEnergySlicer {
    fn shoot(&mut self, commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>, shooter: &Shooter) {
        self.timer += time.delta_seconds();

        if self.timer > self.speed {
            let handle = asset_server.handle.clone();
            
            
            spawn_bullet(commands, &handle, VOID_ENERGY_BULLET, Vec2::new(start_pos.x - 10.0, start_pos.y), angle + 0.261799, shooter);
            spawn_bullet(commands, &handle, VOID_ENERGY_BULLET, Vec2::new(start_pos.x - 10.0, start_pos.y), angle - 0.261799, shooter);
            spawn_bullet(commands, &handle, VOID_ENERGY_BULLET, Vec2::new(start_pos.x + 10.0, start_pos.y), angle + 0.261799, shooter);
            spawn_bullet(commands, &handle, VOID_ENERGY_BULLET, Vec2::new(start_pos.x + 10.0, start_pos.y), angle - 0.261799, shooter);
            
            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}
