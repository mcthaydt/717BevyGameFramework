use crate::common::common_components::*;
use bevy::prelude::*;

impl Default for Player {
    fn default() -> Self {
        Self {
            size: 1.0,
            color: Color::rgb(0.8, 0.7, 0.6),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
        }
    }
}

impl Default for Floor {
    fn default() -> Self {
        Self {
            size: 5.0,
            color: Color::rgb(0.3, 0.5, 0.3),
        }
    }
}

impl Default for GameCamera {
    fn default() -> Self {
        Self {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0),
            look_at_transform: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        Self {
            brightness: 1500.0,
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
        }
    }
}
