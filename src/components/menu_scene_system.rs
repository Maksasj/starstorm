use bevy::prelude::*;

use crate::states::{
    app_state::*,
};

#[derive(Component)]
pub struct MenuEntity;


pub fn menu_scene_system(
        input: Res<Input<KeyCode>>,
        app_state: Res<State<AppState>>,
        mut app_state_next_state: ResMut<NextState<AppState>>,
    ) {

    if input.just_pressed(KeyCode::Space) {
        if app_state.0 != AppState::InGame {
            app_state_next_state.set(AppState::InGame);
        }
    }
}

pub fn despawn_menu_entities(mut commands: Commands, targets: Query<Entity, With<MenuEntity>>) {
    for entity in targets.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
