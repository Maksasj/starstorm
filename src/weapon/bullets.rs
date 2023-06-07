use crate::components::{
    velocity::*,
    collision::*,
};

use crate::weapon::{
    bullet::*,
};

pub const ION_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 15.0, 51),
    velocity: Velocity::with(400.0, 0.0),
    collision_box: CollisionBox::new(15.0, 15.0),
};

pub const METEORIC_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 10.0, 3),
    velocity: Velocity::with(200.0, 0.0),
    collision_box: CollisionBox::new(10.0, 10.0),
};

pub const MORTAR_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 70.0, 35),
    velocity: Velocity::with(50.0, 0.0),
    collision_box: CollisionBox::new(20.0, 20.0),
};

pub const ONYX_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 33.3, 19),
    velocity: Velocity::with(200.0, 0.0),
    collision_box: CollisionBox::new(10.0, 10.0),
};