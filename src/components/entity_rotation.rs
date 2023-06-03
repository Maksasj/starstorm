use bevy::prelude::*;

#[derive(Component)]
pub struct EntityRotation {
    pub rotation_angle: f32,
    pub rotation: Quat,
}

pub const _UP: f32 = 1.5708;
pub const _DOWN: f32 = -1.5708;
pub const _LEFT: f32 = 3.1415;
pub const _RIGHT: f32 = 0.0;

pub fn entity_rotation_system(mut targets: Query<(&mut Transform, &mut EntityRotation)>) {
    for (mut transform, mut rotation) in targets.iter_mut() {
        let rotation_quat = Quat::from_rotation_z(rotation.rotation_angle);
        transform.rotation = rotation_quat;
        rotation.rotation = rotation_quat;
    }
}