use bevy::prelude::*;

use crate::states::{
    app_state::*,
};

use crate::components::{
    menu_scene_system::*,
    game_scene_system::*,
};

use crate::resources::{
    menu_background::*,
    press_space_text::*,
};

pub struct MenuScenePlugin;

impl Plugin for MenuScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            spawn_menu_background_system,
            spawn_press_space_text_system,
            despawn_game_entities,
            ).in_schedule(OnEnter(AppState::MainMenu)));
        app.add_systems((
            despawn_menu_entities,
            ).in_schedule(OnExit(AppState::MainMenu)));

        app.add_systems((
            menu_scene_system,
            ).in_set(OnUpdate(AppState::MainMenu)));
    }
}