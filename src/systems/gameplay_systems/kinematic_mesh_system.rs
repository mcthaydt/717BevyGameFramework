use crate::common::common_components::*;
use crate::common::common_states::GameState;
use bevy::prelude::*;

pub struct KinematicMeshSystemPlugin;

impl Plugin for KinematicMeshSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), create_player_entity)
            .add_systems(
                Update,
                create_player_mesh.run_if(in_state(GameState::Gameplay)),
            );
    }
}

fn create_player_entity(mut commands: Commands) {
    commands.spawn(Player::default());
}

fn create_player_mesh(
    player_query: Query<(Entity, &Player), With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (player_entity, player) in &player_query {
        let player_mesh = meshes.add(Mesh::from(shape::Cube { size: player.size }));
        let player_material = materials.add(player.color.into());
        let player_transform = player.transform;

        let player_bundle = PbrBundle {
            mesh: player_mesh,
            material: player_material,
            transform: player_transform,
            ..Default::default()
        };

        commands
            .entity(player_entity)
            .insert(player_bundle)
            .insert(Name::new("Player"));
    }
}
