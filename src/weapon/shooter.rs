use bevy::prelude::*;

pub enum ShooterType {
    Player,
    Enemy
}

#[derive(Component)]
pub struct Shooter(ShooterType);

impl Shooter {
    pub fn player() -> Self {
        Shooter {
            0: ShooterType::Player,
        }
    }

    pub fn enemy() -> Self {
        Shooter {
            0: ShooterType::Enemy,
        }
    }
}
