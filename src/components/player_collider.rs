use bevy::{
    prelude::*, 
};

use crate::components::{
    collision::*
};

#[derive(Component)]
pub struct PlayerCollider {
    pub collision_layer: usize,
    pub target_layer: usize,
    pub collision_box: Vec2,
}

impl PlayerCollider {
    pub fn new(collision_layer: usize, target_layer: usize, collision_box: Vec2) -> Self {
        PlayerCollider {
            collision_layer: collision_layer,
            target_layer: target_layer,
            collision_box: collision_box,
        }
    }
}

pub struct PlayerCollisionEvent;

impl Collider for PlayerCollider {
    fn get_collision_layer(&self) -> usize {
        return self.collision_layer;
    }

    fn get_target_layer(&self) -> usize {
        return self.target_layer;
    }

    fn get_collision_box(&self) -> Vec2 {
        return self.collision_box;
    }

    fn notify(&self, commands: &mut Commands) {
        commands.add(|w: &mut World| {
            w.send_event(PlayerCollisionEvent{});
        });
    }
}

pub fn player_collision_event_system(mut event_reader: EventReader<PlayerCollisionEvent>) {
    for _event in event_reader.iter() {
        println!("Player coollided");
    }

    event_reader.clear()
}