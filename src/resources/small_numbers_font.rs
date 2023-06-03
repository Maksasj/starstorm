use bevy::prelude::*;

#[derive(Resource)]
pub struct SmallNumberFontSpriteSheet {
    pub handle: Handle<TextureAtlas>
}

pub fn load_small_number_font_system(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let image: Handle<Image> = asset_server.load("small-numbers-font.png");
    let atlas: TextureAtlas = TextureAtlas::from_grid(image, Vec2::splat(8.0), 11, 1, None, None);

    let atlas_handle: Handle<TextureAtlas> = texture_atlases.add(atlas);
    commands.insert_resource(SmallNumberFontSpriteSheet{handle: atlas_handle});
}