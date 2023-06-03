use bevy::prelude::*;

use crate::components::{
    weapon::*,
    bullet::*,
    velocity::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct MortarBluster {
    pub timer: f32,
    pub speed: f32,
}

impl MortarBluster {
    pub fn new() -> Self {
        MortarBluster {
            timer: 0.0,
            speed: 3.5
        }
    }
}

impl Weapon for MortarBluster {
    fn shoot(&mut self, mut commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>) {
        self.timer += time.delta_seconds();

        if self.timer > self.speed {
            let handle = asset_server.handle.clone();
            
            for i in -2..2 {
                spawn_bullet(&mut commands, &handle, 5, start_pos, angle + 0.2617 * i as f32, Velocity::with(0.1, 0.0));
            }
            
            self.timer = 0.0;
        }
    }
}
