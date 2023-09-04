use crate::systems::gameplay_systems::GameplaySystemsPlugin;
use bevy::prelude::*;

pub struct GameplayScreenPlugin;

impl Plugin for GameplayScreenPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(GameplaySystemsPlugin);
    }
}
