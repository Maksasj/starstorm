use bevy::{
    prelude::*, 
    sprite::collide_aabb::collide, 
};

use crate::components::{
    collision::*,
    bullet::*,
    enemy::*,
    health::*,
};

pub fn enemy_and_bullet_collision_event_system(
        mut enemies: Query<(Entity, &mut Health, &Collider, &Transform, &dyn Enemy)>, 
        bullets: Query<(Entity, &Collider, &Transform), With<Bullet>>
    ) {
    
    for (_enemy_entity, mut enemy_health, enemy_collider, enemy_transform, _enemy) in enemies.iter_mut() {
        for (_bullet_entity, bullet_collider, bullet_transform) in bullets.iter() {
            if 0 == ((enemy_collider.collision_layer) & (bullet_collider.target_layer)) {
                continue;
            }

            let first_collision_box = enemy_collider.collision_box;
            let second_collision_box = bullet_collider.collision_box;

            let first_pos = Vec3::new(
                enemy_transform.translation.x - (first_collision_box.x / 2.0), 
                enemy_transform.translation.y - (first_collision_box.y / 2.0), 
                enemy_transform.translation.z
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
                println!("Enemy coollided {:?}, {:?}", enemy_collider.collision_layer, bullet_collider.target_layer);
                enemy_health.take_damage(15.0);
            }
        }
    }
}