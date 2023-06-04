use bevy::{
    prelude::*, 
    sprite::collide_aabb::collide, 
};

use crate::components::{
    collision::*,
    bullet::*,
    health::*,
    damage_shake::*,
};

use crate::resources::{
    sounds::*,
};

pub fn enemy_and_bullet_collision_event_system(
        mut commands: Commands,
        mut enemies: Query<(Entity, &mut DamageShake, &mut Health, &Collider, &Transform, &dyn Enemy)>, 
        bullets: Query<(Entity, &Bullet, &Collider, &Transform, )>,
        mut sound_event_writer: EventWriter<SoundEvent>,
        sounds: Res<Sounds>
    ) {
    
    for (_enemy_entity, mut damage_shake, mut enemy_health, enemy_collider, enemy_transform, _enemy) in enemies.iter_mut() {
        for (bullet_entity, bullet, bullet_collider, bullet_transform) in bullets.iter() {
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
                enemy_health.take_damage(bullet.damage);
                commands.entity(bullet_entity).despawn_recursive();
                damage_shake.start(0.2, bullet.damage / 20.0);
                sound_event_writer.send(SoundEvent{handle: sounds.hit_handle.clone()});
            }
        }
    }
}