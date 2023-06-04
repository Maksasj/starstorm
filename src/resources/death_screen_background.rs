use bevy::prelude::*;

use crate::components::{
    death_scene_system::*,
};

#[derive(Resource)]
pub struct DeathScreenImage {
    pub handle: Handle<Image>
}

pub fn load_death_screen_background_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image: Handle<Image> = asset_server.load("death_screen_background.png");
    commands.insert_resource(DeathScreenImage{handle: image});
}

pub fn spawn_death_screen_background_system(mut commands: Commands, image: Res<DeathScreenImage>) {
    let sprite = Sprite{
        color: Color::rgb(1.0, 1.0, 1.0),
        custom_size: Some(Vec2::new(800.0, 600.0)),
        ..default()
    };

    commands.spawn(SpriteBundle {
        sprite: sprite,
        texture: image.handle.clone(),
        transform: Transform{
            translation: Vec3::new(0.0, 0.0, 200.0),
            ..default()
        },
        ..default()
    }).insert(DeathScreenEntity{});
}
