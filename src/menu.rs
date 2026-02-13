use bevy::app::AppExit;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;

use crate::{Difficulty, GameConfig, GameState};

pub struct MenuPlugin;

#[derive(Component)]
struct MenuTag;

#[derive(Component)]
struct NameText;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct QuitButton;

#[derive(Component)]
struct DifficultyButton(Difficulty);

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, menu_name_input.run_if(in_state(GameState::Menu)))
            .add_systems(Update, menu_buttons.run_if(in_state(GameState::Menu)))
            .add_systems(Update, menu_visuals.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

fn setup_menu(mut commands: Commands, config: Res<GameConfig>) {
    commands
        .spawn((
            MenuTag,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.06, 0.06, 0.07)),
        ))
        .with_children(|root| {
            // Card
            root.spawn((
                MenuTag,
                Node {
                    width: Val::Px(720.0),
                    padding: UiRect::all(Val::Px(28.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(16.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.10, 0.10, 0.12)),
                BorderColor::all(Color::srgb(0.20, 0.20, 0.24)),
            ))
            .with_children(|card| {
                card.spawn((
                    MenuTag,
                    Text::new("Fever Dream"),
                    TextFont {
                        font_size: 78.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                card.spawn((
                    MenuTag,
                    Text::new(
                        "Collect the memories before time runs out.\nWASD/Arrows to move.  1/2/3 to change mood.",
                    ),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.86, 0.86, 0.88)),
                ));

                // Name label
                card.spawn((
                    MenuTag,
                    Text::new("Player name"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.78, 0.78, 0.80)),
                ));

                // Name input box
                card.spawn((
                    MenuTag,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(52.0),
                        padding: UiRect::horizontal(Val::Px(14.0)),
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(12.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.14, 0.14, 0.16)),
                    BorderColor::all(Color::srgb(0.22, 0.22, 0.26)),
                ))
                .with_children(|row| {
                    let shown = if config.player_name.trim().is_empty() {
                        "Type your name…".to_string()
                    } else {
                        config.player_name.clone()
                    };

                    row.spawn((
                        MenuTag,
                        NameText,
                        Text::new(shown),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.92, 0.92, 0.94)),
                    ));
                });

                // Difficulty label
                card.spawn((
                    MenuTag,
                    Text::new("Difficulty"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.78, 0.78, 0.80)),
                ));

                // Difficulty buttons
                card.spawn((
                    MenuTag,
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                ))
                .with_children(|row| {
                    for d in [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard] {
                        row.spawn((
                            MenuTag,
                            Button,
                            DifficultyButton(d),
                            Node {
                                width: Val::Px(160.0),
                                height: Val::Px(48.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::all(Val::Px(12.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.16, 0.16, 0.18)),
                            BorderColor::all(Color::srgb(0.22, 0.22, 0.26)),
                        ))
                        .with_child((
                            MenuTag,
                            Text::new(d.label()),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    }
                });

                // Actions row
                card.spawn((
                    MenuTag,
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                ))
                .with_children(|row| {
                    row.spawn((
                        MenuTag,
                        Button,
                        StartButton,
                        Node {
                            width: Val::Px(220.0),
                            height: Val::Px(56.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.0)),
                            border_radius: BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.18, 0.18, 0.20)),
                        BorderColor::all(Color::srgb(0.22, 0.22, 0.26)),
                    ))
                    .with_child((
                        MenuTag,
                        Text::new("Start"),
                        TextFont {
                            font_size: 26.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    row.spawn((
                        MenuTag,
                        Button,
                        QuitButton,
                        Node {
                            width: Val::Px(160.0),
                            height: Val::Px(56.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.0)),
                            border_radius: BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.18, 0.18, 0.20)),
                        BorderColor::all(Color::srgb(0.22, 0.22, 0.26)),
                    ))
                    .with_child((
                        MenuTag,
                        Text::new("Quit"),
                        TextFont {
                            font_size: 26.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

                card.spawn((
                    MenuTag,
                    Text::new("Keyboard: type name • Enter = Start • Esc = Quit"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.70, 0.70, 0.73)),
                ));
            });
        });
}

fn menu_name_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<GameConfig>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
    mut q_name: Query<&mut Text, With<NameText>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
        return;
    }

    if keys.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Countdown);
        return;
    }

    let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    let mut changed = false;
    for key in keys.get_just_pressed() {
        match *key {
            KeyCode::Backspace | KeyCode::Delete => {
                changed = config.player_name.pop().is_some() || changed;
            }
            KeyCode::Space => {
                if config.player_name.len() < 18 {
                    config.player_name.push(' ');
                    changed = true;
                }
            }
            _ => {
                if let Some(ch) = keycode_to_char(*key, shift) {
                    if config.player_name.len() < 18 {
                        config.player_name.push(ch);
                        changed = true;
                    }
                }
            }
        }
    }

    if changed {
        let shown = if config.player_name.trim().is_empty() {
            "Type your name…".to_string()
        } else {
            config.player_name.clone()
        };

        for mut t in &mut q_name {
            *t = Text::new(shown.clone());
        }
    }
}

fn menu_buttons(
    mut q: Query<
        (
            &Interaction,
            Option<&StartButton>,
            Option<&QuitButton>,
            Option<&DifficultyButton>,
        ),
        (With<Button>, Changed<Interaction>),
    >,
    mut config: ResMut<GameConfig>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (i, start, quit, diff) in &mut q {
        if *i != Interaction::Pressed {
            continue;
        }

        if let Some(d) = diff {
            config.difficulty = d.0;
        } else if start.is_some() {
            next_state.set(GameState::Countdown);
        } else if quit.is_some() {
            exit.write(AppExit::Success);
        }
    }
}

fn menu_visuals(
    config: Res<GameConfig>,
    mut q: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&StartButton>,
            Option<&QuitButton>,
            Option<&DifficultyButton>,
        ),
        With<Button>,
    >,
) {
    let hovered = Color::srgb(0.24, 0.24, 0.27);
    let pressed = Color::srgb(0.28, 0.28, 0.32);

    for (i, mut bg, start, quit, diff) in &mut q {
        let mut base = Color::srgb(0.18, 0.18, 0.20);

        if start.is_some() {
            base = Color::srgb(0.20, 0.20, 0.22);
        } else if quit.is_some() {
            base = Color::srgb(0.18, 0.18, 0.20);
        } else if let Some(d) = diff {
            base = if d.0 == config.difficulty {
                Color::srgb(0.26, 0.26, 0.30)
            } else {
                Color::srgb(0.16, 0.16, 0.18)
            };
        }

        *bg = BackgroundColor(match *i {
            Interaction::Pressed => pressed,
            Interaction::Hovered => hovered,
            Interaction::None => base,
        });
    }
}

fn cleanup_menu(mut commands: Commands, q: Query<Entity, With<MenuTag>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn keycode_to_char(key: KeyCode, shift: bool) -> Option<char> {
    let c = match key {
        KeyCode::KeyA => 'a',
        KeyCode::KeyB => 'b',
        KeyCode::KeyC => 'c',
        KeyCode::KeyD => 'd',
        KeyCode::KeyE => 'e',
        KeyCode::KeyF => 'f',
        KeyCode::KeyG => 'g',
        KeyCode::KeyH => 'h',
        KeyCode::KeyI => 'i',
        KeyCode::KeyJ => 'j',
        KeyCode::KeyK => 'k',
        KeyCode::KeyL => 'l',
        KeyCode::KeyM => 'm',
        KeyCode::KeyN => 'n',
        KeyCode::KeyO => 'o',
        KeyCode::KeyP => 'p',
        KeyCode::KeyQ => 'q',
        KeyCode::KeyR => 'r',
        KeyCode::KeyS => 's',
        KeyCode::KeyT => 't',
        KeyCode::KeyU => 'u',
        KeyCode::KeyV => 'v',
        KeyCode::KeyW => 'w',
        KeyCode::KeyX => 'x',
        KeyCode::KeyY => 'y',
        KeyCode::KeyZ => 'z',
        KeyCode::Digit0 => '0',
        KeyCode::Digit1 => '1',
        KeyCode::Digit2 => '2',
        KeyCode::Digit3 => '3',
        KeyCode::Digit4 => '4',
        KeyCode::Digit5 => '5',
        KeyCode::Digit6 => '6',
        KeyCode::Digit7 => '7',
        KeyCode::Digit8 => '8',
        KeyCode::Digit9 => '9',
        _ => return None,
    };

    Some(if shift { c.to_ascii_uppercase() } else { c })
}
