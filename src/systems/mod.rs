use bevy::prelude::*;

mod camera_system;
mod kinematic_mesh_system;
mod lighting_system;
mod static_mesh_system;
mod window_system;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            window_system::WindowSystemPlugin,                // Create Window
            kinematic_mesh_system::KinematicMeshSystemPlugin, // Create Player
            static_mesh_system::StaticMeshSystemPlugin,       // Create Floor
            camera_system::CameraSystemPlugin,                // Create Camera
            lighting_system::LightingSystemPlugin,            // Configure World Enviroment
                                                              // Physics
                                                              // Input
                                                              // Collision
                                                              // GameState
        ));
    }
}
