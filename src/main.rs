use bevy::{
    prelude::*,
    core_pipeline::clear_color::ClearColorConfig,
    window::PresentMode
};

mod components;
mod resources;

use crate::components::damage_shake::damage_shake_system;
use crate::components::enemy_collider::enemy_and_bullet_collision_event_system;
pub use crate::resources::{
    sprite_sheet::*,
    mouse_position::*,
    background::*,
    small_numbers_font::*,
};

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
                    resolution: (800., 600.).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    resizable: false,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }))
        .add_startup_systems((
                load_spritesheet_system, 
                load_background_system,
                load_small_number_font_system,
                apply_system_buffers, 
                spawn_background_system,
                spawn_player_health_text_system,
                spawn_player_health_bar_system,
                spawn_weapon_charge_bar_system,
                spawn_player_system, 
                spawn_simple_enemy_system,
                spawn_spike_enemy_system,
                spawn_bug_enemy_system,
                setup_camera_shake_system,
            ).chain())
        .add_startup_system(spawn_camera)
        .add_system(player_controller_system)
        .add_system(damage_shake_system)
        .add_system(player_rotation_system)
        .add_system(entity_rotation_system)
        .add_system(velocity_movement_system)
        .add_system(player_velocity_movement_system)
        .add_system(bullet_life_time_system)
        .register_component_as::<dyn Weapon, SimpleBluster>()
        .register_component_as::<dyn Weapon, OnyxBluster>()
        .register_component_as::<dyn Weapon, MortarBluster>()
        .register_component_as::<dyn Weapon, PlayerBluster>()
        
        .register_component_as::<dyn Enemy, SimpleEnemy>()
        .register_component_as::<dyn Enemy, SpikeEnemy>()
        .register_component_as::<dyn Enemy, BugEnemy>()
        .add_system(enemy_moving_system)
        
        .add_event::<CameraShakeEvent>()

        .add_systems((
                enemy_and_bullet_collision_event_system,
                player_and_bullet_collision_event_system,
                enemy_death_system,
                camera_shake_system
            ).chain())
        .add_system(player_helth_text_update_system)
        .add_system(player_helth_bar_update_system)
        .add_system(player_weapon_charge_bar_update_system)
        .add_system(weapon_system)
        .add_system(friction_system)
        .insert_resource(MousePosition::new(Vec2::new(800.0, 600.0)))
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
