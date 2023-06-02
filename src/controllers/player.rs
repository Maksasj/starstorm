use bevy::prelude::*;

use crate::components::{
    entity_rotation::*,
    speed::*,
    player_controller::*,
    mouse_position::*,
};

use crate::resources::{
    sprite_sheet::*,
};

pub fn player_controller_system(input: Res<Input<KeyCode>>, mut targets: Query<(&mut Transform, &mut EntityRotation, &Speed), With<PlayerController>>) {
    if input.pressed(KeyCode::W) {
        for (mut transform, rotation, speed) in targets.iter_mut() {
            transform.translation.y += rotation.rotation_angle.sin();
            transform.translation.x += rotation.rotation_angle.cos();
        }
    }

    if input.pressed(KeyCode::S) {
        for (mut transform, rotation, speed) in targets.iter_mut() {
            transform.translation.y -= rotation.rotation_angle.sin();
            transform.translation.x -= rotation.rotation_angle.cos();
        }
    }
}

pub fn player_rotation_system(input: ResMut<MousePosition>, mut targets: Query<(&mut Transform, &mut EntityRotation), With<PlayerController>>) {
    for (mut transform, mut rotation) in targets.iter_mut() {
        let mut uv_pos_mouse_pos = input.pos;
        
        uv_pos_mouse_pos.x -= transform.translation.x;
        uv_pos_mouse_pos.y -= transform.translation.y;

        uv_pos_mouse_pos -= input.window_size / 2.0;
        uv_pos_mouse_pos /= input.window_size;

        let angle = uv_pos_mouse_pos.y.atan2(uv_pos_mouse_pos.x);

        let rotation_quat = Quat::from_rotation_z(angle);
        transform.rotation = rotation_quat;

        rotation.rotation_angle = angle;
        rotation.rotation = rotation_quat;
    }
}

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<SpriteSheet>) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.color = Color::rgb(1.0, 1.0, 1.0);
    sprite.custom_size = Some(Vec2::splat(64.0));
    
    commands.spawn(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: asset_server.handle.clone(),
        transform: Transform { 
            translation: Vec3::new(0.0, 0.0, 900.0), 
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Name::new("Player"))
    .insert(PlayerController{})
    .insert(EntityRotation{ rotation_angle: 0.0, rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0) })
    .insert(Speed{ speed: 0.0 });
}