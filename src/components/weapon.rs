use bevy::ecs::entity;
use bevy::prelude::*;

use crate::components::{
    player_controller::*,
    mouse_position::*,
    friction::*,
    bullet::*,
};

use crate::resources::{
    sprite_sheet::*,
};

#[derive(Component)]
pub struct Weapon {
    pub shooting_speed: f32,
    pub shooting_auto: bool,
    pub shooting_timer: f32,
}

pub fn weapon_system(
        mut commands: Commands, 
        asset_server: Res<SpriteSheet>, 
        mut targets: Query<(&mut Weapon, &EntityRotation, &Transform)>,
        time: Res<Time>
    ) {

    for (mut weapon, rotation, transform) in targets.iter_mut() {
        if weapon.shooting_auto == false { continue };

        weapon.shooting_timer += time.delta_seconds();

        if weapon.shooting_timer < weapon.shooting_speed { continue; }
        
        weapon.shooting_timer = 0.0;

        spawn_bullet(
            &mut commands, 
            &asset_server, 
            rotation.rotation_angle, 
            Vec2::new(transform.translation.x, transform.translation.y));
    }
}