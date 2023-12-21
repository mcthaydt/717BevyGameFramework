use crate::common::common_states::GameState;
use crate::common::common_systems::despawn_screen;
use crate::common::common_tags::OnMainMenuScreen;
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct MainMenuScreenPlugin;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

impl Plugin for MainMenuScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), main_menu_setup)
            .add_systems(
                Update,
                (
                    detect_play_button_input.run_if(in_state(GameState::MainMenu)),
                    detect_quit_button_input.run_if(in_state(GameState::MainMenu)),
                ),
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                despawn_screen::<OnMainMenuScreen>,
            );
    }
}

fn main_menu_setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), OnMainMenuScreen));

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
        .insert(Name::new("Main Menu Screen Canvas"))
        .insert(OnMainMenuScreen)
        .id();

    let main_menu_title_text = TextBundle::from_section(
        "Main Menu",
        TextStyle {
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    let main_menu_title_text_entity = commands
        .spawn_empty()
        .insert(main_menu_title_text)
        .insert(Name::new("Main Menu Title Text"))
        .insert(OnMainMenuScreen)
        .id();

    commands
        .entity(screen_canvas_entity)
        .add_child(main_menu_title_text_entity);

    let play_button = ButtonBundle {
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

    let play_button_entity = commands
        .spawn_empty()
        .insert(play_button)
        .insert(Name::new("Main Menu Play Button"))
        .insert(PlayButton)
        .insert(OnMainMenuScreen)
        .id();

    let play_button_text = TextBundle::from_section(
        "Play",
        TextStyle {
            font_size: 20.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    let play_button_text_entity = commands
        .spawn_empty()
        .insert(play_button_text)
        .insert(Name::new("Main Menu Play Button Text"))
        .insert(OnMainMenuScreen)
        .id();

    let play_button_with_text_entity = commands
        .entity(play_button_entity)
        .add_child(play_button_text_entity)
        .id();

    commands
        .entity(screen_canvas_entity)
        .add_child(play_button_with_text_entity);

    let quit_button = ButtonBundle {
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

    let quit_button_entity = commands
        .spawn_empty()
        .insert(quit_button)
        .insert(QuitButton)
        .insert(Name::new("Main Menu Quit Button"))
        .insert(OnMainMenuScreen)
        .id();

    let quit_button_text = TextBundle::from_section(
        "Quit",
        TextStyle {
            font_size: 20.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    let quit_button_text_entity = commands
        .spawn_empty()
        .insert(quit_button_text)
        .insert(Name::new("Main Menu Quit Button Text"))
        .insert(OnMainMenuScreen)
        .id();

    let quit_button_with_text_entity = commands
        .entity(quit_button_entity)
        .add_child(quit_button_text_entity)
        .id();

    commands
        .entity(screen_canvas_entity)
        .add_child(quit_button_with_text_entity);
}

fn detect_play_button_input(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::Gameplay);
        }
    }
}

fn detect_quit_button_input(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            exit.send(AppExit);
        }
    }
}
