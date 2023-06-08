use bevy::prelude::*;

pub use crate::components::{
    health::*,
    player_controller::*,
};

pub use crate::states::{
    app_state::*,
};

pub fn player_death_system(
        players: Query<&Health, With<PlayerController>>, 
        app_state: Res<State<AppState>>,
        mut app_state_next_state: ResMut<NextState<AppState>>
    ) {

    for health in players.iter() {
        if health.is_dead() {
            if app_state.0 != AppState::DeathScreen {
                app_state_next_state.set(AppState::DeathScreen);
            }
        }
    } 
}