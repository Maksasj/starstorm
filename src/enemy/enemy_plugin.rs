use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::enemy::{
    enemy::*,

    simple_enemy::*,
    spike_enemy::*,
    bug_enemy::*,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_component_as::<dyn Enemy, SimpleEnemy>();
        app.register_component_as::<dyn Enemy, SpikeEnemy>();
        app.register_component_as::<dyn Enemy, BugEnemy>();
    }
}