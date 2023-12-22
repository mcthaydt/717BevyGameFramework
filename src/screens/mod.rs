use crate::common::common_states::GameState;
use crate::common::common_systems::despawn_screen;
use crate::common::common_tags::OnGameplayScreen;
use bevy::prelude::*;

mod game_end_screen;
mod gameplay_screen;
mod main_menu_screen;
mod splash_screen;

pub struct ScreenManagerPlugin;

impl Plugin for ScreenManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            splash_screen::SplashScreenPlugin,
            main_menu_screen::MainMenuScreenPlugin,
            gameplay_screen::GameplayScreenPlugin,
            game_end_screen::GameEndScreenPlugin,
        ))
        .add_systems(
            OnExit(GameState::Gameplay),
            despawn_screen::<OnGameplayScreen>,
        );
    }
}
