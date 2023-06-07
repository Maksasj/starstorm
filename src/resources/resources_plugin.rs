use bevy::prelude::*;

use crate::resources:: {
    sprite_sheet::*,
    small_numbers_font::*,
    big_numbers_font::*,
    death_screen_background::*,
    game_background::*,
    menu_background::*,
    press_space_text::*,
    sounds::*,
};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((
            load_spritesheet_system, 
            load_game_background_system,
            load_death_screen_background_system,
            load_menu_background_system,
            load_small_number_font_system,
            load_big_number_font_system,
            load_press_space_text_system,
            load_sounds_system
        ).chain());
    }
}