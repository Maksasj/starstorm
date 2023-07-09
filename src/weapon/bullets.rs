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

pub const NOVA_PLASMA_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 60.0, 67),
    velocity: Velocity::with(120.0, 0.0),
    collision_box: CollisionBox::new(15.0, 15.0),
};

pub const GALACTIC_PLASMA_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 45.0, 83),
    velocity: Velocity::with(120.0, 0.0),
    collision_box: CollisionBox::new(15.0, 15.0),
};

pub const SMALL_ION_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 5.0, 99),
    velocity: Velocity::with(400.0, 0.0),
    collision_box: CollisionBox::new(15.0, 15.0),
};

pub const QUANTUM_FUSION_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 80.0, 115),
    velocity: Velocity::with(110.0, 0.0),
    collision_box: CollisionBox::new(35.0, 35.0),
};

pub const STARFIRE_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 20.0, 131),
    velocity: Velocity::with(200.0, 0.0),
    collision_box: CollisionBox::new(15.0, 15.0),
};

pub const HYPERION_PARTICLE_BEAM_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 60.0, 147),
    velocity: Velocity::with(260.0, 0.0),
    collision_box: CollisionBox::new(35.0, 35.0),
};

pub const PLASMA_FUSION_BURST_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 60.0, 163),
    velocity: Velocity::with(150.0, 0.0),
    collision_box: CollisionBox::new(15.0, 15.0),
};

pub const VOID_ENERGY_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 60.0, 179),
    velocity: Velocity::with(150.0, 0.0),
    collision_box: CollisionBox::new(15.0, 15.0),
};

pub const QUANTUM_REPULSOR_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 3.0, 195),
    velocity: Velocity::with(400.0, 0.0),
    collision_box: CollisionBox::new(10.0, 10.0),
};

pub const NOVA_SURGE_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 40.0, 211),
    velocity: Velocity::with(150.0, 0.0),
    collision_box: CollisionBox::new(15.0, 15.0),
};

pub const QUANTUM_BLAZE_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 60.0, 227),
    velocity: Velocity::with(100.0, 0.0),
    collision_box: CollisionBox::new(15.0, 15.0),
};

pub const IONIZED_PULSE_BULLET: BulletBundle = BulletBundle {
    bullet: Bullet::new(10.0, 25.0, 243),
    velocity: Velocity::with(180.0, 0.0),
    collision_box: CollisionBox::new(15.0, 15.0),
};