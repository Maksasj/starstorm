use bevy::{
    prelude::*,
    core_pipeline::clear_color::ClearColorConfig,
    window::PresentMode
};

mod components;
mod controllers;
mod resources;

pub use crate::resources::{
    sprite_sheet::*,
};

pub use crate::controllers::{
    player::*,
};

pub use crate::components::{
    entity_rotation::*,
    velocity::*,
    player_controller::*,
    mouse_position::*,
    friction::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(
            ImagePlugin::default_nearest(),
            ).set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Starstorm".into(),
                    resolution: (800., 600.).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    resizable: false,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }))
        .add_startup_systems((load_spritesheet_system, apply_system_buffers, spawn_player_system).chain())
        .add_startup_system(spawn_camera)
        .add_system(player_controller_system)
        .add_system(player_rotation_system)
        .add_system(player_shoot_system)
        .add_system(velocity_movement_system)
        .add_system(friction_system)
        .insert_resource(MousePosition { 
            pos: Vec2::new(0.0, 0.0),
            window_size: Vec2::new(800.0, 600.0),
        })
        .add_system(mouse_position_update_system) 
        .run();
}

fn mouse_position_update_system(mut mouse: ResMut<MousePosition>, mut events: EventReader<CursorMoved>) {
    for e in events.iter() {
        mouse.pos = e.position;
        mouse.window_size = Vec2::new(800.0, 600.0);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}
