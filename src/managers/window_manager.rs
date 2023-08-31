use crate::common::common_constants::DefaultWindowSettings;
use bevy::prelude::*;

pub struct WindowManagerPlugin;

impl Plugin for WindowManagerPlugin {
    fn build(&self, app: &mut App) {
        let default_window = Window {
            title: DefaultWindowSettings::WINDOW_TITLE.into(),
            resolution: (
                DefaultWindowSettings::WINDOW_WIDTH,
                DefaultWindowSettings::WINDOW_HEIGHT,
            )
                .into(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        };

        let window_plugin = WindowPlugin {
            primary_window: Some(default_window),
            ..default()
        };

        let default_plugins = DefaultPlugins.set(window_plugin);

        app.insert_resource(Msaa::Sample4)
            .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
            .add_plugins(default_plugins);
    }
}
