use bevy::prelude::*;

use crate::components::{
    friction::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[bevy_trait_query::queryable]
pub trait Weapon {
    fn shoot(&mut self, commands: &mut Commands, asset_server: &Res<SpriteSheet>, angle: f32, start_pos: Vec2, time: &Res<Time>);
    fn get_charge(&self) -> f32;
}

pub fn weapon_system(
        mut commands: Commands, 
        asset_server: Res<SpriteSheet>, 
        mut targets: Query<(&mut dyn Weapon, &EntityRotation, &Transform)>,
        time: Res<Time>
    ) {

    for (weapons, rotation, transform) in targets.iter_mut() {
        for mut weapon in weapons {
            weapon.shoot(&mut commands, &asset_server, rotation.rotation_angle, Vec2::new(transform.translation.x, transform.translation.y), &time);
        }
    }
}