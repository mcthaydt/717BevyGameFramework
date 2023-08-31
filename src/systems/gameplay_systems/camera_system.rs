use bevy::prelude::*;

use crate::common::common_components::GameCamera;

pub struct CameraSystemPlugin;

impl Plugin for CameraSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera_entity)
            .add_systems(PostStartup, create_camera);
    }
}

fn create_camera_entity(mut commands: Commands) {
    commands.spawn(GameCamera::default());
}

fn create_camera(
    camera_query: Query<(Entity, &GameCamera), With<GameCamera>>,
    mut commands: Commands,
) {
    for (camera_entity, camera) in &camera_query {
        let camera_transform = camera
            .transform
            .looking_at(camera.look_at_transform, Vec3::Y);
        let camera_bundle = Camera3dBundle {
            transform: camera_transform,
            ..default()
        };

        commands
            .entity(camera_entity)
            .insert(camera_bundle)
            .insert(Name::new("Camera"));
    }
}

// Track Player Position and Rotation