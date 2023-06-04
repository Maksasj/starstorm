use bevy::prelude::*;

use crate::states::{
    app_state::*,
};

pub fn main_menu_system(
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