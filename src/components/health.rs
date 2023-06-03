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

    pub fn with(value: f32, max_value: f32) -> Self {
        Health { 
            value: value,
            max_value: max_value,
        }
    }
}
