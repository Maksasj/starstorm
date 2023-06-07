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
};

mod components;
pub use crate::components::{
    entity_rotation::*,
    velocity::*,
    player_controller::*,
    friction::*,
    bullet::*,
    player::*,

    collision::*,
    player_collider::*,

    weapon::*,
    simple_bluster::*,
    onyx_bluster::*,
    mortar_bluster::*,
    player_bluster::*,

    enemy::*,
    simple_enemy::*,
    spike_enemy::*,
    bug_enemy::*,

    camera_shake::*,
    player_health_text::*,
    weapon_charget_bar::*,
    damage_shake::*,
    enemy_collider::*,

    menu_scene_system::*,
    game_scene_system::*,
    wave_system::*,
    wave_count_text::*,
    death_scene_system::*,
    player_death_system::*,
};

mod states;
pub use crate::states::{
    app_state::*,

    menu_sceen::*,
    game_sceen::*,
    death_sceen::*,
};

fn main() {
    use bevy_trait_query::RegisterExt;

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
            
        .register_component_as::<dyn Weapon, SimpleBluster>()
        .register_component_as::<dyn Weapon, OnyxBluster>()
        .register_component_as::<dyn Weapon, MortarBluster>()
        .register_component_as::<dyn Weapon, PlayerBluster>()
            
        .register_component_as::<dyn Enemy, SimpleEnemy>()
        .register_component_as::<dyn Enemy, SpikeEnemy>()
        .register_component_as::<dyn Enemy, BugEnemy>()
        
        .insert_resource(MousePosition::new(Vec2::new(800.0, 600.0)))
        .add_event::<CameraShakeEvent>()
        .add_event::<WaveSwitchEvent>()    
        .add_event::<PlayerShootEvent>()
        .add_event::<SoundEvent>()
        .add_state::<AppState>()     
        
        .add_systems((
            spawn_game_background_system, 
            spawn_player_health_text_system,
            spawn_player_health_bar_system,
            spawn_weapon_charge_bar_system,
            spawn_player_system, 
            spawn_wave_spawner_system,
            spawn_wave_count_text_system,
        ).in_schedule(OnEnter(AppState::InGame)))
        .add_systems((

        ).in_schedule(OnExit(AppState::InGame)))
        .add_systems((
            player_controller_system,
            damage_shake_system,
            player_rotation_system,
            entity_rotation_system,
            velocity_movement_system,
            player_velocity_movement_system,
            bullet_life_time_system,
            enemy_moving_system,
            enemy_and_bullet_collision_event_system,
            player_and_bullet_collision_event_system,
            enemy_death_system,
            camera_shake_system,
            player_helth_text_update_system,
            player_helth_bar_update_system,
            player_weapon_charge_bar_update_system,
        ).in_set(OnUpdate(AppState::InGame)))
            
        .add_systems((
            weapon_system,
            friction_system,
            player_shoot_system,
            wave_counting_system,
            wave_clear_system,
            wave_spawn_system,
            wave_text_update_system,
            player_death_system,
            game_scene_system,
            ).chain().in_set(OnUpdate(AppState::InGame)))
        
        .add_plugin(MenuScenePlugin)

        .add_systems((
            spawn_death_screen_background_system,
            spawn_press_space_text_system,
            ).in_schedule(OnEnter(AppState::DeathScreen)))
        .add_systems((
            despawn_death_scene_entities,
            ).in_schedule(OnExit(AppState::DeathScreen)))
        .add_systems((
            death_scene_system,
            ).in_set(OnUpdate(AppState::DeathScreen)))

        .add_systems((
            mouse_position_update_system, 
            wavy_update_system,
            handle_sounds,
        )) 
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