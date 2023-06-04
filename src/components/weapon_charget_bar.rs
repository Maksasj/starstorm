use bevy::prelude::*;

pub use crate::components::{
    entity_rotation::*,
    player_controller::*,
    health::*,
    weapon::*,
    game_scene_system::*,
};

pub use crate::resources::{
    sprite_sheet::*,
    small_numbers_font::*,
};

#[derive(Component)]
pub struct PlayerChargetBar;

pub fn spawn_weapon_charge_bar_system(mut commands: Commands, asset_server: Res<SpriteSheet>) {
    let mut sprite = TextureAtlasSprite::new(9);
    sprite.color = Color::rgb(1.0, 1.0, 1.0);
    sprite.custom_size = Some(Vec2::new(125.0, 12.0));

    commands.spawn(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: asset_server.handle.clone(),
        transform: Transform { 
            translation: Vec3::new(209.5, 282.0, 900.0), 
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("PlayerWeaponChargeBar"))
    .insert(Visibility::Visible)
    .insert(PlayerChargetBar{})
    .insert(GameEntity{});
}

pub fn player_weapon_charge_bar_update_system(
        players: Query<&dyn Weapon, With<PlayerController>>, 
        mut health_bars: Query<(&PlayerChargetBar, &mut TextureAtlasSprite, &mut Transform)>
    ) {
        
    for weapons in players.iter() {
        let mut rate = 1.0;

        for weapon in weapons {
            rate = weapon.get_charge();
        }

        for (_health_bar, mut bar_sprite, mut transform) in health_bars.iter_mut() {
            bar_sprite.custom_size = Some(Vec2::new(125.0 * rate, 12.0));
            transform.translation.x = 82.0 + (125.0 / 2.0) * rate;
        }
    }
}