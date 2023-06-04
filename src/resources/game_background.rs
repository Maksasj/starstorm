use bevy::prelude::*;

use crate::components::{
    game_scene_system::*,
};

#[derive(Resource)]
pub struct GameBackgroundImage {
    pub handle: Handle<Image>
}

pub fn load_game_background_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image: Handle<Image> = asset_server.load("game_background.png");
    commands.insert_resource(GameBackgroundImage{handle: image});
}

pub fn spawn_game_background_system(mut commands: Commands, image: Res<GameBackgroundImage>) {
    let sprite = Sprite{
        color: Color::rgb(1.0, 1.0, 1.0),
        custom_size: Some(Vec2::new(800.0, 600.0)),
        ..default()
    };

    commands.spawn(SpriteBundle {
        sprite: sprite,
        texture: image.handle.clone(),
        transform: Transform{
            ..default()
        },
        ..default()
    }).insert(GameEntity);
}
