use bevy::prelude::*;

mod camera_system;
mod kinematic_mesh_system;
mod lighting_system;
mod static_mesh_system;

pub struct GameplaySystemsPlugin;

impl Plugin for GameplaySystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            kinematic_mesh_system::KinematicMeshSystemPlugin, // Create Kinematic Bodies
            static_mesh_system::StaticMeshSystemPlugin,       // Create Static Bodies
            camera_system::CameraSystemPlugin,                // Create Camera
            lighting_system::LightingSystemPlugin,            // Configure Lighting and Enviroment
        ));
    }
}
