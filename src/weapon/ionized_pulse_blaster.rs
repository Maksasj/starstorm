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
pub struct IonizedPulseBlaster {
    pub timer: f32,
    pub speed: f32,
}

impl IonizedPulseBlaster {
    pub fn default() -> Self {
        IonizedPulseBlaster {
            timer: 0.0,
            speed: 1.2,
        }
    }
}

impl Weapon for IonizedPulseBlaster {
    fn shoot(&mut self, commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>, shooter: &Shooter) {
        self.timer += time.delta_seconds();

        if self.timer > self.speed {
            let handle = asset_server.handle.clone();
            
            spawn_bullet(commands, &handle, STARFIRE_BULLET, Vec2::new(start_pos.x - 10.0, start_pos.y), angle + 3.14159, shooter);
            spawn_bullet(commands, &handle, STARFIRE_BULLET, Vec2::new(start_pos.x + 10.0, start_pos.y), angle + 3.14159, shooter);

            spawn_bullet(commands, &handle, IONIZED_PULSE_BULLET, start_pos, angle + 0.2, shooter);
            spawn_bullet(commands, &handle, IONIZED_PULSE_BULLET, start_pos, angle, shooter);
            spawn_bullet(commands, &handle, IONIZED_PULSE_BULLET, start_pos, angle - 0.2, shooter);
            
            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}
