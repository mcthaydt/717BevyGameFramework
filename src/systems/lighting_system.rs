use bevy::prelude::*;

use crate::common::common_components::Light;

pub struct LightingSystemPlugin;

impl Plugin for LightingSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_light_entity)
            .add_systems(PostStartup, create_light);
    }
}

fn create_light_entity(mut commands: Commands) {
    commands.spawn(Light::default());
}

fn create_light(light_query: Query<(Entity, &Light), With<Light>>, mut commands: Commands) {
    for (light_entity, light) in &light_query {
        let scene_point_light = PointLightBundle {
            point_light: PointLight {
                intensity: light.brightness,
                shadows_enabled: true,
                ..default()
            },
            transform: light.transform,
            ..default()
        };

        commands
            .entity(light_entity)
            .insert(scene_point_light)
            .insert(Name::new("Lamp"));
    }
}
