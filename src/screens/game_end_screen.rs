use crate::common::common_states::GameState;
use crate::common::common_systems::despawn_screen;
use crate::common::common_tags::OnGameEndScreen;
use bevy::prelude::*;

pub struct GameEndScreenPlugin;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct ReturnToMenuButton;

impl Plugin for GameEndScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameEnd), game_end_setup)
            .add_systems(
                Update,
                (
                    detect_restart_button_input.run_if(in_state(GameState::GameEnd)),
                    detect_return_to_menu_button_input.run_if(in_state(GameState::GameEnd)),
                ),
            )
            .add_systems(
                OnExit(GameState::GameEnd),
                despawn_screen::<OnGameEndScreen>,
            );
    }
}

fn game_end_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), OnGameEndScreen));

    let screen_canvas = NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            row_gap: Val::Px(10.0),
            ..default()
        },
        ..default()
    };

    let screen_canvas_entity = commands
        .spawn_empty()
        .insert(screen_canvas)
        .insert(Name::new("Game End Screen Canvas"))
        .insert(OnGameEndScreen)
        .id();

    let game_end_title_text = TextBundle::from_section(
        "Game End",
        TextStyle {
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    let game_end_title_text_entity = commands
        .spawn_empty()
        .insert(game_end_title_text)
        .insert(Name::new("Game End Title Text"))
        .insert(OnGameEndScreen)
        .id();

    commands
        .entity(screen_canvas_entity)
        .add_child(game_end_title_text_entity);

    let restart_button = ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: BackgroundColor(Color::WHITE),
        ..default()
    };

    let restart_button_entity = commands
        .spawn_empty()
        .insert(restart_button)
        .insert(Name::new("Restart Button"))
        .insert(RestartButton)
        .insert(OnGameEndScreen)
        .id();

    let restart_button_text = TextBundle::from_section(
        "Restart",
        TextStyle {
            font_size: 20.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    let restart_button_text_entity = commands
        .spawn_empty()
        .insert(restart_button_text)
        .insert(Name::new("Game End Restart Button Text"))
        .insert(OnGameEndScreen)
        .id();

    let restart_button_with_text_entity = commands
        .entity(restart_button_entity)
        .add_child(restart_button_text_entity)
        .id();

    commands
        .entity(screen_canvas_entity)
        .add_child(restart_button_with_text_entity);

    let return_to_menu_button = ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: BackgroundColor(Color::WHITE),
        ..default()
    };

    let return_to_menu_button_entity = commands
        .spawn_empty()
        .insert(return_to_menu_button)
        .insert(ReturnToMenuButton)
        .insert(Name::new("Game End Return to Menu Button"))
        .insert(OnGameEndScreen)
        .id();

    let return_to_menu_button_text = TextBundle::from_section(
        "Return to Menu",
        TextStyle {
            font_size: 20.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    )
    .with_text_alignment(TextAlignment::Center);

    let return_to_menu_button_text_entity = commands
        .spawn_empty()
        .insert(return_to_menu_button_text)
        .insert(Name::new("Return to Menu Button Text"))
        .insert(OnGameEndScreen)
        .id();

    let return_to_menu_button_with_text_entity = commands
        .entity(return_to_menu_button_entity)
        .add_child(return_to_menu_button_text_entity)
        .id();

    commands
        .entity(screen_canvas_entity)
        .add_child(return_to_menu_button_with_text_entity);
}

fn detect_restart_button_input(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::Gameplay);
        }
    }
}

fn detect_return_to_menu_button_input(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ReturnToMenuButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::MainMenu);
        }
    }
}
