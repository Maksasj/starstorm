use bevy::{
    prelude::*, 
    sprite::collide_aabb::collide
};

pub const NONE_COLLISION_LAYER: usize =     0b0;
pub const PLAYER_COLLISION_LAYER: usize =  0b01;
pub const BULLET_COLLISION_LAYER: usize =  0b10;
pub const ENEMY_COLLISION_LAYER: usize =  0b100;

#[derive(Component)]
pub struct Collider {
    pub collision_layer: usize,
    pub target_layer: usize,
    pub collision_box: Vec2,
}

impl Collider {
    pub fn new(collision_layer: usize, target_layer: usize, collision_box: Vec2) -> Self {
        Collider {
            collision_layer: collision_layer,
            target_layer: target_layer,
            collision_box: collision_box,
        }
    }
}