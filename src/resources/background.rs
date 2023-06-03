use bevy::prelude::*;

#[derive(Resource)]
pub struct BackgroundImage {
    pub handle: Handle<Image>
}

pub fn load_background_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image: Handle<Image> = asset_server.load("background.png");
    commands.insert_resource(BackgroundImage{handle: image});
}

pub fn spawn_background_system(mut commands: Commands, image: Res<BackgroundImage>) {
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
    });
}
