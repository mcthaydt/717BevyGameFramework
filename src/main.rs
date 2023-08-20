use bevy::prelude::*;

mod common;
mod debug;
mod game_manager;
mod screens;
mod systems;

fn main() {
    App::new()
        .add_plugins((game_manager::GameManagerPlugin, debug::DebugPlugin))
        .run();
}
