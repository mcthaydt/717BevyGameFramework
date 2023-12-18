use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Player {
    pub size: f32,
    pub color: Color,
    pub transform: Transform,
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Floor {
    pub size: f32,
    pub color: Color,
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct GameCamera {
    pub transform: Transform,
    pub look_at_transform: Vec3,
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Light {
    pub brightness: f32,
    pub transform: Transform,
}
