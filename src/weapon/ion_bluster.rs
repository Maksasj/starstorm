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
pub struct IonBluster {
    pub timer: f32,
    pub speed: f32,
}

impl IonBluster {
    pub fn default() -> Self {
        IonBluster {
            timer: 0.0,
            speed: 0.8,
        }
    }
}

impl Weapon for IonBluster {
    fn shoot(&mut self, commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>, shooter: &Shooter) {
        self.timer += time.delta_seconds();

        if self.timer > self.speed {
            let handle = asset_server.handle.clone();
            
            spawn_bullet(commands, &handle, ION_BULLET, start_pos, angle, shooter);

            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}

