use bevy::{
    prelude::*, 
    sprite::collide_aabb::collide, 
};

use crate::components::{
    collision::*,
    player_controller::*,
    bullet::*,
};

pub fn player_and_bullet_collision_event_system(
        players: Query<(Entity, &Collider, &Transform), With<PlayerController>>, 
        bullets: Query<(Entity, &Collider, &Transform), With<Bullet>>
    ) {
    
    for (_player_entity, player_collider, player_transform) in players.iter() {
        for (_bullet_entity, bullet_collider, bullet_transform) in bullets.iter() {
            if 0 == ((player_collider.collision_layer) & (bullet_collider.target_layer)) {
                continue;
            }
            

            let first_collision_box = player_collider.collision_box;
            let second_collision_box = bullet_collider.collision_box;

            let first_pos = Vec3::new(
                player_transform.translation.x - (first_collision_box.x / 2.0), 
                player_transform.translation.y - (first_collision_box.y / 2.0), 
                player_transform.translation.z
            ); 

            let second_pos = Vec3::new(
                bullet_transform.translation.x - (second_collision_box.x / 2.0), 
                bullet_transform.translation.y - (second_collision_box.y / 2.0), 
                bullet_transform.translation.z
            ); 

            let collision = collide(
                second_pos, 
                first_collision_box, 
                first_pos, 
                second_collision_box);
            
            if !collision.is_none() {
                // println!("Player coollided {:?}, {:?}", player_collider.collision_layer, bullet_collider.target_layer);
            }
        }
    }
}