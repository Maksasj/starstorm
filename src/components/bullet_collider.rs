use bevy::{
    prelude::*, 
};

use crate::components::{
    collision::*
};

#[derive(Component)]
pub struct BulletCollider {
    pub collision_layer: usize,
    pub target_layer: usize,
    pub collision_box: Vec2,
}

impl BulletCollider {
    pub fn new(collision_layer: usize, target_layer: usize, collision_box: Vec2) -> Self {
        BulletCollider {
            collision_layer: collision_layer,
            target_layer: target_layer,
            collision_box: collision_box,
        }
    }
}

impl Collider for BulletCollider {
    fn get_collision_layer(&self) -> usize {
        return self.collision_layer;
    }

    fn get_target_layer(&self) -> usize {
        return self.target_layer;
    }

    fn get_collision_box(&self) -> Vec2 {
        return self.collision_box;
    }
}

pub struct BulletCollisionEvent(Entity, Entity);

pub fn bullet_collision_event_system(mut event_reader: EventReader<BulletCollisionEvent>) {
    for event in event_reader.iter() {
        println!("Bullet coollided with {:?}, {:?}", event.0.index(), event.1.index());
    }

    event_reader.clear()
}