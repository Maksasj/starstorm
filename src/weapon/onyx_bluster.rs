use bevy::prelude::*;

use crate::weapon::{
    weapon::*,
    bullet::*,
};

use crate::components::{
    velocity::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct OnyxBluster {
    pub timer: f32,
    pub speed: f32,
    pub damage: f32,
}

impl OnyxBluster {
    pub fn default() -> Self {
        OnyxBluster {
            timer: 0.0,
            speed: 0.8,
            damage: 33.0,
        }
    }
}

impl Weapon for OnyxBluster {
    fn shoot(&mut self, mut commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>) {
        self.timer += time.delta_seconds();

        
        if self.timer > self.speed {
            let handle = asset_server.handle.clone();

            spawn_bullet(&mut commands, &handle, 19, start_pos, angle + 0.5235, Velocity::with(200.0, 0.0), self.damage);
            spawn_bullet(&mut commands, &handle, 19, start_pos, angle, Velocity::with(200.0, 0.0), self.damage);
            spawn_bullet(&mut commands, &handle, 19, start_pos, angle - 0.5235, Velocity::with(200.0, 0.0), self.damage);
            
            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}
