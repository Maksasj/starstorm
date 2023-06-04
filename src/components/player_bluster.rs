use bevy::prelude::*;

use crate::components::{
    weapon::*,
    bullet::*,
    velocity::*,
    collision::*,
};

use crate::resources::{
    sprite_sheet::*,
    sounds::*,
};

#[derive(Component)]
pub struct PlayerBluster {
    pub timer: f32,
    pub speed: f32,
    pub damage: f32,
}

impl PlayerBluster {
    pub fn new() -> Self {
        PlayerBluster {
            timer: 0.0,
            speed: 0.8,
            damage: 15.0,
        }
    }
}

pub struct PlayerShootEvent;

impl Weapon for PlayerBluster {
    fn shoot(&mut self, commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>) {
        self.timer += time.delta_seconds();

        if self.timer > self.speed {
            let handle = asset_server.handle.clone();
            
            spawn_bullet_from_bundle(commands, BulletBundle::custom(                
                &handle, 
                7, 
                start_pos, 
                angle, 
                Velocity::with(400.0, 0.0),
                Collider::new(
                    NONE_COLLISION_LAYER, 
                    ENEMY_COLLISION_LAYER, 
                    Vec2::new(15.0, 15.0)
                ),
                self.damage
            ));
            
            commands.add(|w: &mut World| {
                w.send_event(PlayerShootEvent);
            });

            self.timer = 0.0;
        }
    }
    fn get_charge(&self) -> f32 {
        self.timer / self.speed
    }
}

pub fn player_shoot_system(
        mut player_shoot_event_reader: EventReader<PlayerShootEvent>,
        mut sound_event_writer: EventWriter<SoundEvent>,
        sounds: Res<Sounds>
    ) {

    for _event in player_shoot_event_reader.iter() {
        sound_event_writer.send(SoundEvent{handle: sounds.shoot_handle.clone()});
    }
}
