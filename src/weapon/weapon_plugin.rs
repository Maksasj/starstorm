use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

use crate::components::player_shoot::*;

use crate::weapon::{
    weapon::*,

    meteoric_bluster::*,
    onyx_bluster::*,
    mortar_bluster::*,
    ion_bluster::*,
};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerShootEvent>();

        app.register_component_as::<dyn Weapon, MeteoricBluster>();
        app.register_component_as::<dyn Weapon, OnyxBluster>();
        app.register_component_as::<dyn Weapon, MortarBluster>();
        app.register_component_as::<dyn Weapon, IonBluster>();
    }
}