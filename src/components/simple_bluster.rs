use bevy::prelude::*;

use crate::components::{
    weapon::*,
    bullet::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct SimpleBluster {
    pub timer: f32,
    pub speed: f32,
}

impl SimpleBluster {
    pub fn new() -> Self {
        SimpleBluster {
            timer: 0.0,
            speed: 0.3
        }
    }
}

impl Weapon for SimpleBluster {
    fn shoot(&mut self, mut commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>) {
        self.timer += time.delta_seconds();

        if self.timer > self.speed {
            spawn_bullet(&mut commands, &asset_server, angle, start_pos);
            self.timer = 0.0;
        }
    }
}
