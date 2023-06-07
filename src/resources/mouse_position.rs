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

pub fn mouse_position_update_system(mut mouse: ResMut<MousePosition>, mut events: EventReader<CursorMoved>) {
    for e in events.iter() {
        mouse.pos = e.position;
        mouse.window_size = Vec2::new(800.0, 600.0);
    }
}