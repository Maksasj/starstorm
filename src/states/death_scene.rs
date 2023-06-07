use bevy::prelude::*;

pub use crate::weapon::{
    bullet::*,
};

pub use crate::components::{
    entity_rotation::*,
    velocity::*,
    player_controller::*,
    friction::*,
    player::*,
    collision::*,
    player_collider::*,
    camera_shake::*,
    player_health_text::*,
    weapon_charget_bar::*,
    damage_shake::*,
    menu_scene_system::*,
    game_scene_system::*,
    wave_system::*,
    wave_count_text::*,
    death_scene_system::*,
    player_death_system::*,
};

use crate::resources::{
    death_screen_background::*,
    press_space_text::*,
};

pub struct DeathScenePlugin;

impl Plugin for DeathScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            spawn_death_screen_background_system,
            spawn_press_space_text_system
            ).in_schedule(OnEnter(AppState::DeathScreen)));
        app.add_systems((
            despawn_death_scene_entities,
            ).in_schedule(OnExit(AppState::DeathScreen)));
        app.add_systems((
            death_scene_system,
            ).in_set(OnUpdate(AppState::DeathScreen)));
    }
}