use bevy::prelude::*;

pub use crate::weapon::{
    ion_bluster::*,
    bullet::*,
};

pub use crate::enemy::{
    enemy::*,
    enemy_collider::*,
};

pub use crate::components::{
    entity_rotation::*,
    velocity::*,
    player_controller::*,
    friction::*,
    player::*,
    collision::*,
    player_collider::*,
    player_health_text::*,
    weapon_charget_bar::*,
    damage_shake::*,
    menu_scene_system::*,
    game_scene_system::*,
    wave_system::*,
    wave_count_text::*,
    death_scene_system::*,
    player_death_system::*,
    camera_shake::*,
    player_shoot::*,
};

use crate::resources::{
    game_background::*,
};

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            spawn_game_background_system, 
            spawn_player_health_text_system,
            spawn_player_health_bar_system,
            spawn_weapon_charge_bar_system,
            spawn_player_system, 
            spawn_wave_spawner_system,
            spawn_wave_count_text_system,
        ).in_schedule(OnEnter(AppState::InGame)));
        app.add_systems((
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
        ).in_set(OnUpdate(AppState::InGame)));
        app.add_systems((
            weapon_system,
            friction_system,
            player_shoot_system,
            wave_counting_system,
            wave_clear_system,
            wave_spawn_system,
            wave_text_update_system,
            player_death_system,
            game_scene_system,
            ).chain().in_set(OnUpdate(AppState::InGame)));
    }
}