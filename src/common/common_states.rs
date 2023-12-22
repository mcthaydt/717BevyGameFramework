use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    Gameplay,
    GameEnd,
}
