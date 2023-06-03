use bevy::prelude::*;

pub use crate::components::{
    entity_rotation::*,
};

#[derive(Component)]
pub struct Health {
    pub value: f32,
    pub max_value: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Health { 
            value: value,
            max_value: value,
        }
    }

    pub fn _with(value: f32, max_value: f32) -> Self {
        Health { 
            value: value,
            max_value: max_value,
        }
    }

    pub fn _is_dead(&self) -> bool {
        self.value < self.max_value
    }
}
