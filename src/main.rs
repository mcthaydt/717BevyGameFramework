use bevy::prelude::*;

mod common;
mod debug;
mod managers;
mod screens;
mod systems;

fn main() {
    App::new()
        .add_plugins((
            managers::game_manager::GameManagerPlugin,
            debug::DebugPlugin,
        ))
        .run();
}
