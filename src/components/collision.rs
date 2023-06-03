use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {

}

impl Collider {
    pub fn new() -> Self {
        Collider {

        }
    }
}

#[derive(Bundle)]
pub struct ColliderBundle {
    pub collider: Collider,
}

impl ColliderBundle {
    pub fn new() -> Self {
        ColliderBundle { 
            collider: Collider{
                
            }
        }
    }
}

pub fn collision_system() {

}