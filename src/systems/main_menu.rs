use bevy::prelude::*;
use bevy::ui::Node;

use crate::{components::main_menu::{MainMenuAction, MainMenuUI}, states::game_state::GameState};

pub fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        MainMenuUI,
        children![(
            Button,
            Node {
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            MainMenuAction::NewGame,
            children![(
                Text::new("New Game"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE)
            )]
        ),
        (
            Button,
            Node {
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            MainMenuAction::Quit,
            children![(
                Text::new("Quit"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE)
            )]
        )]
    ));
}

pub fn main_menu_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<(&Interaction, &MainMenuAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match action{
                MainMenuAction::NewGame => {
                    next_state.set(GameState::WorldGenSetup);
                },
                MainMenuAction::Quit => {
                    std::process::exit(0);
                }
            }
        }
    }
}

pub fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}