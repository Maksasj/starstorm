use bevy::{
    prelude::*, 
    sprite::collide_aabb::collide
};

pub const NONE_COLLISION_LAYER: usize = 0b0;
pub const PLAYER_COLLISION_LAYER: usize = 0b1;
pub const BULLET_COLLISION_LAYER: usize = 0b10;

#[bevy_trait_query::queryable]
pub trait Collider {
    fn get_collision_layer(&self) -> usize;
    fn get_target_layer(&self) -> usize;
    fn get_collision_box(&self) -> Vec2;
}

pub struct CollisionEvent(Entity, Entity);

pub fn collider_system(targets: Query<(Entity, &dyn Collider, &Transform)>, mut event_writer: EventWriter<CollisionEvent>) {
    for (first_entity, f_collider, first_transform) in targets.iter() {
        for (second_entity, s_collider, second_transform) in targets.iter() {
            
            let first_collider: &dyn Collider = f_collider.iter().last().unwrap();
            let second_collider: &dyn Collider = s_collider.iter().last().unwrap();

            if 0 == ((first_collider.get_target_layer()) & (second_collider.get_collision_layer())) {
                continue;
            }

            let first_collision_box = first_collider.get_collision_box();
            let second_collision_box = second_collider.get_collision_box();

            let first_pos = Vec3::new(
                first_transform.translation.x - (first_collision_box.x / 2.0), 
                first_transform.translation.y - (first_collision_box.y / 2.0), 
                first_transform.translation.z
            ); 

            let second_pos = Vec3::new(
                second_transform.translation.x - (second_collision_box.x / 2.0), 
                second_transform.translation.y - (second_collision_box.y / 2.0), 
                second_transform.translation.z
            ); 

            let collision = collide(
                second_pos, 
                first_collision_box, 
                first_pos, 
                second_collision_box);
            
            if !collision.is_none() {
                event_writer.send(CollisionEvent{ 0: first_entity, 1: second_entity});
            }
        }
    } 
}

pub fn collision_event_system(mut event_reader: EventReader<CollisionEvent>) {
    for event in event_reader.iter() {
        println!("{:?}, {:?}", event.0.index(), event.1.index());
    }

    event_reader.clear()
}