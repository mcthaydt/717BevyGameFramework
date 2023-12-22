use crate::common::common_states::GameState;
use crate::common::common_systems::despawn_screen;
use crate::common::common_tags::OnGameplayScreen;
use crate::systems::gameplay_systems::GameplaySystemsPlugin;
use bevy::prelude::*;

pub struct GameplayScreenPlugin;

#[derive(Resource, Deref, DerefMut)]
struct SwitchToGameEndTimer(Timer);

impl Plugin for GameplayScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameplaySystemsPlugin)
            .add_systems(OnEnter(GameState::Gameplay), gameplay_setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::Gameplay)))
            .add_systems(
                OnExit(GameState::Gameplay),
                despawn_screen::<OnGameplayScreen>,
            );
    }
}

fn gameplay_setup(mut commands: Commands) {
    commands.insert_resource(SwitchToGameEndTimer(Timer::from_seconds(
        3.0,
        TimerMode::Once,
    )));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SwitchToGameEndTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::GameEnd);
    }
}
