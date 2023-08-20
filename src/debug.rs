use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::common::common_components;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new())
            .register_type::<common_components::Player>()
            .register_type::<common_components::GameCamera>()
            .register_type::<common_components::Light>()
            .register_type::<common_components::Floor>();
    }
}
