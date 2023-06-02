use bevy::prelude::*;

#[derive(Resource)]
pub struct MousePosition {
    pub pos: Vec2,
    pub window_size: Vec2,
}