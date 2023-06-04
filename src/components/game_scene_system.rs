use bevy::prelude::*;

use crate::states::{
    app_state::*,
};

#[derive(Component)]
pub struct GameEntity;

pub fn game_scene_system(
        input: Res<Input<KeyCode>>,
        app_state: Res<State<AppState>>,
        mut app_state_next_state: ResMut<NextState<AppState>>,
    ) {

    if input.just_pressed(KeyCode::Escape) {
        if app_state.0 != AppState::MainMenu {
            app_state_next_state.set(AppState::MainMenu);
        }
    }
}

pub fn despawn_game_entities(mut commands: Commands, targets: Query<Entity, With<GameEntity>>) {
    for entity in targets.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
