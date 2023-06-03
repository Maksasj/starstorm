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
pub struct SimpleBluster {
    pub timer: f32,
    pub speed: f32,
    pub damage: f32,
}

impl SimpleBluster {
    pub fn new() -> Self {
        SimpleBluster {
            timer: 0.0,
            speed: 0.3,
            damage: 10.0,
        }
    }
}

impl Weapon for SimpleBluster {
    fn shoot(&mut self, mut commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>) {
        self.timer += time.delta_seconds();

        if self.timer > self.speed {
            let handle = asset_server.handle.clone();
            
            spawn_bullet(&mut commands, &handle, 4, start_pos, angle, Velocity::with(0.4, 0.0), self.damage);
            
            self.timer = 0.0;
        }
    }
}
