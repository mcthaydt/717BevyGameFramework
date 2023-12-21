use crate::common::common_states::GameState;
use crate::common::common_systems::despawn_screen;
use crate::common::common_tags::OnSplashScreen;
use bevy::prelude::*;

pub struct SplashScreenPlugin;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), splash_setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), OnSplashScreen));
    let icon = asset_server.load("icon.png");

    let screen_canvas = NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ..default()
    };

    let screen_canvas_entity = commands
        .spawn_empty()
        .insert(screen_canvas)
        .insert(OnSplashScreen)
        .id();

    let splash_icon = ImageBundle {
        style: Style {
            width: Val::Px(200.0),
            ..default()
        },
        image: UiImage::new(icon),
        ..default()
    };

    let splash_icon_entity = commands
        .spawn_empty()
        .insert(splash_icon)
        .insert(OnSplashScreen)
        .id();

    commands
        .entity(screen_canvas_entity)
        .add_child(splash_icon_entity)
        .insert(Name::new("Splash Screen Icon"));

    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::MainMenu);
    }
}
