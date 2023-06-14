use bevy::prelude::*;

use crate::config::ui::*;
use crate::setup::setup::*;
use crate::grid::grid::*;
use crate::grid::cell::*;

#[derive(Component)]
pub struct Button;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct ResetButton;

fn button_spawner<T: Component>(
    commands: &mut Commands,
    size: Size,
    color: Color,
    text: String,
    component: T,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let button = commands
        .spawn(ButtonBundle {
            style: Style {
                size: size,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(color),

            ..default()
        })
        .insert(Button)
        .insert(component)
        .id();

    let text = commands
        .spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
        ))
        .id();

    commands.entity(button).add_child(text);

    button
}

pub fn button_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let parent = commands
        .spawn(NodeBundle {
            style: Style {
                size: UI_SIZE,
                position_type: PositionType::Relative,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::SpaceEvenly,
                direction: Direction::LeftToRight,
                ..default()
            },
            background_color: BackgroundColor(Color::ANTIQUE_WHITE),
            ..default()
        })
        .id();

    let start_button = button_spawner(
        &mut commands,
        BUTTON_SIZE,
        START_BUTTON_COLOR,
        "START".to_string(),
        StartButton,
        &asset_server,
    );

    let reset_button = button_spawner(
        &mut commands,
        BUTTON_SIZE,
        RESET_BUTTON_COLOR,
        "RESET".to_string(),
        ResetButton,
        &asset_server,
    );

    commands
        .entity(parent)
        .add_child(start_button)
        .add_child(reset_button);
}


pub fn start_button_event_system(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >,
) {
    
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = BackgroundColor(START_BUTTON_COLOR_CLICKED);
                next_state.set(AppState::Running);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(START_BUTTON_COLOR_HOVER);
            }
            Interaction::None => {
                *color = BackgroundColor(START_BUTTON_COLOR);
            }
        }
    }
}

pub fn reset_button_event_system(
    mut logical_grid: ResMut<LogicalGrid>,
    mut query: Query<(&mut Sprite, &mut Cell), With<Cell>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResetButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = BackgroundColor(RESET_BUTTON_COLOR_CLICKED);
                next_state.set(AppState::Draw);
                reset_grid_system(&mut logical_grid,&mut query);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(RESET_BUTTON_COLOR_HOVER);
            }
            Interaction::None => {
                *color = BackgroundColor(RESET_BUTTON_COLOR);
            }
        }
    }
}
