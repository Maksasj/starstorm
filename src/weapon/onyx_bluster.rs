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
pub struct OnyxBluster {
    pub timer: f32,
    pub speed: f32,
}

impl OnyxBluster {
    pub fn default() -> Self {
        OnyxBluster {
            timer: 0.0,
            speed: 0.8,
        }
    }
}

impl Weapon for OnyxBluster {
    fn shoot(&mut self, commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>, shooter: &Shooter) {
        self.timer += time.delta_seconds();

        
        if self.timer > self.speed {
            let handle = asset_server.handle.clone();

            spawn_bullet(commands, &handle, ONYX_BULLET, start_pos, angle + 0.5235, shooter);
            spawn_bullet(commands, &handle, ONYX_BULLET, start_pos, angle, shooter);
            spawn_bullet(commands, &handle, ONYX_BULLET, start_pos, angle - 0.5235, shooter);
            
            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}
