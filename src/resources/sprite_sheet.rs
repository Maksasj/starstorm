use bevy::prelude::*;

#[derive(Resource)]
pub struct SpriteSheet {
    pub handle: Handle<TextureAtlas>
}

pub fn load_spritesheet_system(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let image: Handle<Image> = asset_server.load("spritesheet.png");
    let atlas: TextureAtlas = TextureAtlas::from_grid(image, Vec2 { x: 32.0, y: 32.0 }, 8, 8, None, None);

    let atlas_handle: Handle<TextureAtlas> = texture_atlases.add(atlas);
    commands.insert_resource(SpriteSheet{handle: atlas_handle});
}