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
pub struct DualRapidSpaceBlastCannon {
    pub timer: f32,
    pub speed: f32,
}

impl DualRapidSpaceBlastCannon {
    pub fn default() -> Self {
        DualRapidSpaceBlastCannon {
            timer: 0.0,
            speed: 0.2,
        }
    }
}

impl Weapon for DualRapidSpaceBlastCannon {
    fn shoot(&mut self, commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>, shooter: &Shooter) {
        self.timer += time.delta_seconds();

        
        if self.timer > self.speed {
            let handle = asset_server.handle.clone();

            spawn_bullet(commands, &handle, SMALL_ION_BULLET, Vec2::new(start_pos.x - 10.0, start_pos.y), angle, shooter);
            spawn_bullet(commands, &handle, SMALL_ION_BULLET, Vec2::new(start_pos.x + 10.0, start_pos.y), angle, shooter);
            
            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}
