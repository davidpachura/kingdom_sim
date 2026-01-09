use bevy::input::keyboard::Key;
use bevy::ui::Node;
use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::{components::game_config::*, states::game_state::GameState};

pub fn setup_game_config(mut commands: Commands) {
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
        GameConfigUI,
        children![
            seed_field(),
            terrain_scale_field(),
            continental_scale_field(),
            octave_field(),
            sea_threshold_field(),
            temperature_scale_field(),
            moisture_scale_field(),
            scaling_factor_field(),
            (
                Button,
                Node {
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                GameConfigAction::Generate,
                children![(
                    Text::new("Generate"),
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
                GameConfigAction::Back,
                children![(
                    Text::new("Back to Menu"),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(Color::WHITE)
                )]
            ),
        ],
    ));
}

fn seed_field() -> impl Bundle {
    return (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            ..default()
        },
        children![
            (
                Text::new("Seed:"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE)
            ),
            (
                Button,
                Node {
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                TextInput,
                InputValue {
                    text: String::new()
                },
                SeedField,
                children![(
                    Text::new(""),
                    SeedField,
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE)
                )]
            )
        ],
    );
}

fn terrain_scale_field() -> impl Bundle {
    return (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            ..default()
        },
        children![
            (
                Text::new("Terrain scale:"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE)
            ),
            (
                Button,
                Node {
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                TextInput,
                InputValue {
                    text: String::new()
                },
                TerrainScaleField,
                children![(
                    Text::new(""),
                    TerrainScaleField,
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE)
                )]
            )
        ],
    );
}

fn continental_scale_field() -> impl Bundle {
    return (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            ..default()
        },
        children![
            (
                Text::new("Continental scale:"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE)
            ),
            (
                Button,
                Node {
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                TextInput,
                InputValue {
                    text: String::new()
                },
                ContinentalScaleField,
                children![(
                    Text::new(""),
                    ContinentalScaleField,
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE)
                )]
            )
        ],
    );
}

fn octave_field() -> impl Bundle {
    return (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            ..default()
        },
        children![
            (
                Text::new("Number of octaves:"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE)
            ),
            (
                Button,
                Node {
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                TextInput,
                InputValue {
                    text: String::new()
                },
                OctaveField,
                children![(
                    Text::new(""),
                    OctaveField,
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE)
                )]
            )
        ],
    );
}

fn sea_threshold_field() -> impl Bundle {
    return (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            ..default()
        },
        children![
            (
                Text::new("Sea threshold:"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                Button,
                Node {
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                TextInput,
                InputValue {
                    text: String::new(),
                },
                SeaThresholdField,
                children![(
                    Text::new(""),
                    SeaThresholdField,
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE)
                )],
            )
        ],
    );
}

fn temperature_scale_field() -> impl Bundle {
    return (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            ..default()
        },
        children![
            (
                Text::new("Temperature scale:"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                Button,
                Node {
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                TextInput,
                InputValue {
                    text: String::new(),
                },
                TemperatureScaleField,
                children![(
                    Text::new(""),
                    TemperatureScaleField,
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE)
                )],
            )
        ],
    );
}

fn moisture_scale_field() -> impl Bundle {
    return (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            ..default()
        },
        children![
            (
                Text::new("Moisture scale:"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                Button,
                Node {
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                TextInput,
                InputValue {
                    text: String::new(),
                },
                MoistureScaleField,
                children![(
                    Text::new(""),
                    MoistureScaleField,
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE)
                )],
            )
        ],
    );
}

fn scaling_factor_field() -> impl Bundle {
    return (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            ..default()
        },
        children![
            (
                Text::new("Scaling factor:"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                Button,
                Node {
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                TextInput,
                InputValue {
                    text: String::new(),
                },
                ScalingFactorField,
                children![(
                    Text::new(""),
                    ScalingFactorField,
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE)
                )],
            )
        ],
    );
}

pub fn focus_text_inputs(
    mut commands: Commands,
    interactions: Query<(Entity, &Interaction), (With<TextInput>, Changed<Interaction>)>,
    focused: Query<Entity, With<Focused>>,
) {
    for (entity, interaction) in &interactions {
        if *interaction == Interaction::Pressed {
            for e in &focused {
                commands.entity(e).remove::<Focused>();
            }

            commands.entity(entity).insert(Focused);
        }
    }
}

pub fn game_config_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut button_query: Query<
        (&Interaction, &GameConfigAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action) in &mut button_query {
        if *interaction == Interaction::Pressed {
            match action {
                GameConfigAction::Generate => {
                    next_state.set(GameState::Playing);
                }
                GameConfigAction::Back => {
                    next_state.set(GameState::MainMenu);
                }
            }
        }
    }
}

pub fn game_config_text_input(
    mut keyboard_input_reader: MessageReader<KeyboardInput>,
    mut text_query: Query<&mut InputValue, With<Focused>>,
) {
    if let Ok(mut input) = text_query.single_mut() {
        for keyboard_input in keyboard_input_reader.read() {
            if !keyboard_input.state.is_pressed() {
                continue;
            }

            match (&keyboard_input.logical_key, &keyboard_input.text) {
                (Key::Backspace, _) => {
                    input.text.pop();
                }
                (_, Some(inserted_text)) => {
                    // Make sure the text doesn't have any control characters,
                    // which can happen when keys like Escape are pressed
                    if inserted_text.chars().all(is_printable_char) {
                        input.text.push_str(inserted_text);
                    }
                }
                _ => continue,
            }
        }
    }
}

fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
        || ('\u{f0000}'..='\u{ffffd}').contains(&chr)
        || ('\u{100000}'..='\u{10fffd}').contains(&chr);

    !is_in_private_use_area && !chr.is_ascii_control()
}

pub fn update_text_display(
    query: Query<(&InputValue, &Children), Changed<InputValue>>,
    mut text_query: Query<&mut Text>,
) {
    for (input, children) in &query {
        for &child in children {
            if let Ok(mut text) = text_query.get_mut(child) {
                text.clear();
                text.push_str(&input.text);
            }
        }
    }
}

pub fn cleanup_game_config(mut commands: Commands, query: Query<Entity, With<GameConfigUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
