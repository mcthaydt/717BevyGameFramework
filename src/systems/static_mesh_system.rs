use crate::common::common_components::Floor;
use bevy::prelude::*;

pub struct StaticMeshSystemPlugin;

impl Plugin for StaticMeshSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_floor_entity)
            .add_systems(PostStartup, create_floor_mesh);
    }
}

fn create_floor_entity(mut commands: Commands) {
    commands.spawn(Floor::default());
}

fn create_floor_mesh(
    floor_query: Query<(Entity, &Floor), With<Floor>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (floor_entity, floor) in &floor_query {
        let floor_mesh = meshes.add(Mesh::from(shape::Plane::from_size(floor.size)));
        let floor_material = materials.add(floor.color.into());

        let floor_bundle = PbrBundle {
            mesh: floor_mesh,
            material: floor_material,
            ..Default::default()
        };

        commands
            .entity(floor_entity)
            .insert(floor_bundle)
            .insert(Name::new("Floor"));
    }
}
