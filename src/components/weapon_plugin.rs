use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::components::{
    weapon::*,

    simple_bluster::*,
    onyx_bluster::*,
    mortar_bluster::*,
    player_bluster::*,
};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_component_as::<dyn Weapon, SimpleBluster>();
        app.register_component_as::<dyn Weapon, OnyxBluster>();
        app.register_component_as::<dyn Weapon, MortarBluster>();
        app.register_component_as::<dyn Weapon, PlayerBluster>();
    }
}