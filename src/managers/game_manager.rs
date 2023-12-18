use super::window_manager;
use crate::common::common_states;
use crate::screens;
use bevy::prelude::*;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            window_manager::WindowManagerPlugin, // Create Window
            screens::ScreenManagerPlugin,        // Create Screens
        ))
        .add_state::<common_states::GameState>();
    }
}
