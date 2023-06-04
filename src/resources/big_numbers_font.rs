use bevy::prelude::*;

#[derive(Resource)]
pub struct BigNumberFontSpriteSheet {
    pub handle: Handle<TextureAtlas>
}

pub fn load_big_number_font_system(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let image: Handle<Image> = asset_server.load("big-numbers-font.png");
    let atlas: TextureAtlas = TextureAtlas::from_grid(image, Vec2::splat(16.0), 10, 1, None, None);

    let atlas_handle: Handle<TextureAtlas> = texture_atlases.add(atlas);
    commands.insert_resource(BigNumberFontSpriteSheet{handle: atlas_handle});
}