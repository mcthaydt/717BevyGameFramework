use bevy::prelude::*;

mod game_end_screen;
mod gameplay_screen;
mod main_menu_screen;

pub struct ScreenManagerPlugin;

impl Plugin for ScreenManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            main_menu_screen::MainMenuScreenPlugin,
            gameplay_screen::GameplayScreenPlugin,
            game_end_screen::GameEndScreenPlugin,
        ));
    }
}
