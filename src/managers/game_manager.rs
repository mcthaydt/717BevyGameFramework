use super::window_manager;
use crate::screens;
use bevy::prelude::*;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            window_manager::WindowManagerPlugin, // Create Window
            screens::ScreenManagerPlugin,        // Create Screens
        ));
    }
}

//move input here
//move audio here
