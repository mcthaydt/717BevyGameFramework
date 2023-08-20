use bevy::prelude::*;

use crate::systems::SystemsPlugin;

pub struct GameManagerPlugin;

impl Plugin for GameManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SystemsPlugin);
    }
}
