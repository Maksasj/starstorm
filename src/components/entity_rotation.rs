use bevy::prelude::*;

#[derive(Component)]
pub struct EntityRotation {
    pub rotation_angle: f32,
    pub rotation: Quat,
}
