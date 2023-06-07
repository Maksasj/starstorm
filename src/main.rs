#![windows_subsystem = "windows"]

// #[cfg(not(target = "wasm32-unknown-unknown"))]
// use winit::window::Icon;

// #[cfg(not(target = "wasm32-unknown-unknown"))]
// use bevy::winit::*;

use bevy::{
    prelude::*,
    core_pipeline::clear_color::ClearColorConfig,
    window::*,
};


use bevy_kira_audio::prelude::*;

mod resources;
use crate::components::weapon_plugin::WeaponPlugin;
pub use crate::resources::{
    sprite_sheet::*,
    mouse_position::*,
    small_numbers_font::*,
    big_numbers_font::*,
    game_background::*,
    menu_background::*,
    press_space_text::*,
    death_screen_background::*,
    sounds::*,
    resources_plugin::*,
    mouse_position::*,
};

mod enemy;
pub use crate::enemy::{
    enemy_plugin::*,
};

mod components;
pub use crate::components::{
    weapon_plugin::*,
};

mod states;
pub use crate::states::{
    app_state::*,

    menu_scene::*,
    game_scene::*,
    death_scene::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(
            ImagePlugin::default_nearest(),
            ).set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Starstorm".into(),
                    resolution: (800.0, 600.0).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: false,
                    canvas: Some("#game".to_string()),
                    resizable: false,
                    prevent_default_event_handling: false,
                    resize_constraints: WindowResizeConstraints{
                        min_width: 800.0,
                        min_height: 600.0,
                        max_width: 800.0,
                        max_height: 600.0,
                    },
                    ..default()
                }),
                ..default()
            }))
        .add_plugin(AudioPlugin)
        //.add_plugin(ResourcesPlugin)
        .add_startup_systems((
                // #[cfg(not(target = "wasm32-unknown-unknown"))]
                // setup_window_icon,
                load_spritesheet_system, 
                load_game_background_system,
                load_death_screen_background_system,
                load_menu_background_system,
                load_small_number_font_system,
                load_big_number_font_system,
                load_press_space_text_system,
                load_sounds_system,

                apply_system_buffers, 
                setup_camera_shake_system,
                spawn_camera,
                play_main_theme_looped_system,
            ).chain())
            
        .add_plugin(WeaponPlugin)
        .add_plugin(EnemyPlugin)
        
        .insert_resource(MousePosition::new(Vec2::new(800.0, 600.0)))
        .add_event::<CameraShakeEvent>()
        .add_event::<WaveSwitchEvent>()    
        .add_event::<PlayerShootEvent>()
        .add_event::<SoundEvent>()
        .add_state::<AppState>()     

        .add_plugin(MenuScenePlugin)
        .add_plugin(GameScenePlugin)
        .add_plugin(DeathScenePlugin)

        .add_systems((
            mouse_position_update_system, 
            wavy_update_system,
            handle_sounds,
        )) 
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgba(0.0, 0.0, 0.0, 1.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}

// #[cfg(not(target = "wasm32-unknown-unknown"))]
/*
pub fn setup_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_window_entity = primary_window_query.single();
    let primary_window = windows.get_window(primary_window_entity).unwrap();

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/starstorm_icon_clear.png")
            .expect("Failed to open icon")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary_window.set_window_icon(Some(icon));
}
*/