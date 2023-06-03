use bevy::{
    prelude::*, 
    sprite::collide_aabb::collide
};

pub const NONE_COLLISION_LAYER: usize = 0b0;
pub const PLAYER_COLLISION_LAYER: usize = 0b1;
pub const BULLET_COLLISION_LAYER: usize = 0b10;

#[derive(Component)]
pub struct Collider {
    collision_layer: usize,
    target_layer: usize,
}

impl Collider {
    pub fn new(collision_layer: usize, target_layer: usize) -> Self {
        Collider {
            collision_layer: collision_layer,
            target_layer: target_layer,
        }
    }
}

#[derive(Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
}

impl ColliderBundle {
    pub fn new(collision_layer: usize, target_layer: usize) -> Self {
        ColliderBundle { 
            collider: Collider::new(collision_layer, target_layer),
        }
    }
}

pub struct CollisionEvent(Entity, Entity);

pub fn collider_system(targets: Query<(Entity, &mut Collider, &Transform)>, mut event_writer: EventWriter<CollisionEvent>) {
    
    for (first_entity, first_collider, first_transform) in targets.iter() {
        for (second_entity, second_collider, second_transform) in targets.iter() {
            if 0 == ((first_collider.target_layer) & (second_collider.collision_layer)) {
                continue;
            }

            let collision = collide(
                first_transform.translation, 
                Vec2::splat(50.0), 
                second_transform.translation, 
                Vec2::splat(50.0));
            
            if !collision.is_none() {
                event_writer.send(CollisionEvent{ 0: first_entity, 1: second_entity});
            }
        }
    } 
}

pub fn collision_event_system(mut event_reader: EventReader<CollisionEvent>) {
    for event in event_reader.iter() {

    }

    event_reader.clear()
}