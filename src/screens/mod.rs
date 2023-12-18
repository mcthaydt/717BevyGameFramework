use bevy::prelude::*;

mod gameplay_screen;
mod splash_screen;

pub struct ScreenManagerPlugin;

impl Plugin for ScreenManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            splash_screen::SplashScreenPlugin,
            gameplay_screen::GameplayScreenPlugin,
        ));
    }
}
