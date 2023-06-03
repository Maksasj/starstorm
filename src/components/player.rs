use bevy::prelude::*;

use crate::components::{
    friction::*,
    simple_bluster::*,
    player_controller::*,
};

use crate::resources::{
    sprite_sheet::*,
    mouse_position::*,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    name: Name,
    rotation: EntityRotation,
    controller: PlayerController, 
    friction: Friction,
    velocity: Velocity, 
    weapon: SimpleBluster,
}

impl PlayerBundle {
    pub fn new() -> Self {
        PlayerBundle { 
            name: Name::new("Player"),
            rotation: EntityRotation::new(_UP),
            controller: PlayerController{}, 
            friction: Friction::new(0.97),
            velocity: Velocity::new(),
            weapon: SimpleBluster::new(),
        }
    }
}

pub fn player_rotation_system(input: ResMut<MousePosition>, mut targets: Query<(&mut Transform, &mut EntityRotation), With<PlayerController>>) {
    for (transform, mut rotation) in targets.iter_mut() {
        let mut uv_pos_mouse_pos = input.pos;

        uv_pos_mouse_pos.x -= transform.translation.x;
        uv_pos_mouse_pos.y -= transform.translation.y;

        uv_pos_mouse_pos -= input.window_size / 2.0;
        uv_pos_mouse_pos /= input.window_size;

        let angle = uv_pos_mouse_pos.y.atan2(uv_pos_mouse_pos.x);
        
        rotation.rotation_angle = angle;
    }
}

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<SpriteSheet>) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.color = Color::rgb(1.0, 1.0, 1.0);
    sprite.custom_size = Some(Vec2::splat(32.0));
    
    commands.spawn(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: asset_server.handle.clone(),
        transform: Transform { 
            translation: Vec3::new(0.0, 0.0, 900.0), 
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(PlayerBundle::new());
}