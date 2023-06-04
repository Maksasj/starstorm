use bevy::{
    prelude::*, 
};

use rand::Rng;

use crate::Enemy;

#[derive(Component)]
pub struct DamageShake {
    pub timer: f32,
    pub shake_time: f32, 
    pub amplitude: f32,
    pub enabled: bool,

    offset: Vec2,
}

impl DamageShake {
    pub fn new(timer: f32, shake_time: f32, amplitude: f32, enabled: bool) -> Self {
        DamageShake {
            timer: timer,
            shake_time: shake_time,
            amplitude: amplitude,
            enabled: enabled,
            offset: Vec2::splat(0.0),
        }
    }

    pub fn start(&mut self, shake_time: f32, amplitude: f32) {
        self.timer = 0.0;
        self.shake_time = shake_time;
        self.amplitude = amplitude;
        self.enabled = true;
        self.offset = Vec2::splat(0.0);
    }
}

pub fn damage_shake_system(mut targets: Query<(&mut DamageShake, &mut Transform, &dyn Enemy)>, time: Res<Time>) {
    for (mut damage_shake,mut transform, _enemy) in targets.iter_mut() {
        if damage_shake.enabled == true {
            damage_shake.timer += time.delta_seconds();
            
            if damage_shake.timer > damage_shake.shake_time {
                damage_shake.enabled = false;
                damage_shake.timer = 0.0;
            }
    
            let mut rng = rand::thread_rng();
            
            let x = rng.gen_range(-1.0..1.0) * damage_shake.amplitude;
            let y = rng.gen_range(-1.0..1.0) * damage_shake.amplitude;

            damage_shake.offset.x += x;
            damage_shake.offset.y += y;

            transform.translation.x += x;
            transform.translation.y += y;
        } else {
            transform.translation.x -= damage_shake.offset.x;
            transform.translation.y -= damage_shake.offset.y;

            damage_shake.offset = Vec2::splat(0.0);
        }
    }
}