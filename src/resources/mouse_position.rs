use bevy::prelude::*;

#[derive(Resource)]
pub struct MousePosition {
    pub pos: Vec2,
    pub window_size: Vec2,
}

impl MousePosition {
    pub fn new(window_size: Vec2) -> Self {
        MousePosition {
            pos: Vec2::splat(0.0),
            window_size: window_size,
        }
    }
}