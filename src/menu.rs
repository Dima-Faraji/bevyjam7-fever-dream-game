use bevy::app::AppExit;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::schedule::IntoScheduleConfigs; // <-- IMPORTANT for .run_if(...)
use bevy::prelude::*;

use crate::{Difficulty, GameConfig, GameState};

pub struct MenuPlugin;

#[derive(Component)]
struct MenuTag;

#[derive(Component)]
struct TitleText;

#[derive(Component)]
struct TitleGlow;

#[derive(Component)]
struct NameBox;

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
            BackgroundColor(Color::srgb(0.05, 0.05, 0.06)),
        ))
        .with_children(|root| {
            // Background “dream blobs” (purely decorative)
            root.spawn((
                MenuTag,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(-180.0),
                    top: Val::Px(-220.0),
                    width: Val::Px(620.0),
                    height: Val::Px(620.0),
                    border_radius: BorderRadius::all(Val::Px(999.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.85, 0.25, 0.95, 0.12)),
            ));
            root.spawn((
                MenuTag,
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(-220.0),
                    bottom: Val::Px(-200.0),
                    width: Val::Px(700.0),
                    height: Val::Px(700.0),
                    border_radius: BorderRadius::all(Val::Px(999.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.20, 0.90, 0.95, 0.10)),
            ));
            root.spawn((
                MenuTag,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(30.0),
                    top: Val::Percent(55.0),
                    width: Val::Px(360.0),
                    height: Val::Px(360.0),
                    border_radius: BorderRadius::all(Val::Px(999.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.95, 0.35, 0.55, 0.08)),
            ));

            // Card
            root.spawn((
                MenuTag,
                Node {
                    width: Val::Px(760.0),
                    padding: UiRect::all(Val::Px(28.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(14.0),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(18.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.09, 0.09, 0.11)),
                BorderColor::all(Color::srgb(0.22, 0.22, 0.28)),
            ))
            .with_children(|card| {
                // Title area (glow + main)
                card.spawn((
                    MenuTag,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(90.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|t| {
                    t.spawn((
                        MenuTag,
                        TitleGlow,
                        Text::new("✦ FEVER DREAM ✦"),
                        TextFont {
                            font_size: 74.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.85, 0.25, 0.95, 0.45)),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(4.0),
                            top: Val::Px(3.0),
                            ..default()
                        },
                    ));

                    t.spawn((
                        MenuTag,
                        TitleText,
                        Text::new("✦ FEVER DREAM ✦"),
                        TextFont {
                            font_size: 74.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.95, 0.95, 0.98)),
                    ));
                });

                // Divider line (neon)
                card.spawn((
                    MenuTag,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(2.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.20, 0.90, 0.95, 0.35)),
                ));

                card.spawn((
                    MenuTag,
                    Text::new(
                        "Collect the memories before time runs out.\nWASD/Arrows to move • 1/2/3 to change mood",
                    ),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.86, 0.86, 0.90)),
                ));

                // Name label
                card.spawn((
                    MenuTag,
                    Text::new("PLAYER NAME"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.76, 0.76, 0.80)),
                ));

                // Name input box
                card.spawn((
                    MenuTag,
                    NameBox,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(54.0),
                        padding: UiRect::horizontal(Val::Px(14.0)),
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(14.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.12, 0.12, 0.14)),
                    BorderColor::all(Color::srgb(0.22, 0.22, 0.28)),
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
                        TextColor(Color::srgb(0.93, 0.93, 0.96)),
                    ));
                });

                // Difficulty label
                card.spawn((
                    MenuTag,
                    Text::new("DIFFICULTY"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.76, 0.76, 0.80)),
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
                        let icon = match d {
                            Difficulty::Easy => "◆",
                            Difficulty::Normal => "◇",
                            Difficulty::Hard => "✹",
                        };

                        row.spawn((
                            MenuTag,
                            Button,
                            DifficultyButton(d),
                            Node {
                                width: Val::Px(170.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::all(Val::Px(14.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.14, 0.14, 0.16)),
                            BorderColor::all(Color::srgb(0.22, 0.22, 0.28)),
                        ))
                        .with_child((
                            MenuTag,
                            Text::new(format!("{icon}  {}", d.label())),
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
                            width: Val::Px(240.0),
                            height: Val::Px(58.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.0)),
                            border_radius: BorderRadius::all(Val::Px(14.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.12, 0.18, 0.20)),
                        BorderColor::all(Color::srgb(0.22, 0.22, 0.28)),
                    ))
                    .with_child((
                        MenuTag,
                        Text::new("START DREAM"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    row.spawn((
                        MenuTag,
                        Button,
                        QuitButton,
                        Node {
                            width: Val::Px(170.0),
                            height: Val::Px(58.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.0)),
                            border_radius: BorderRadius::all(Val::Px(14.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.16, 0.12, 0.14)),
                        BorderColor::all(Color::srgb(0.22, 0.22, 0.28)),
                    ))
                    .with_child((
                        MenuTag,
                        Text::new("QUIT"),
                        TextFont {
                            font_size: 24.0,
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
                    TextColor(Color::srgb(0.70, 0.70, 0.74)),
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
    time: Res<Time>,
    config: Res<GameConfig>,
    mut q_buttons: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&StartButton>,
            Option<&QuitButton>,
            Option<&DifficultyButton>,
        ),
        With<Button>,
    >,
    mut q_name_box: Query<&mut BorderColor, (With<NameBox>, Without<Button>)>,
    mut q_title: Query<&mut TextColor, (With<TitleText>, Without<TitleGlow>)>,
    mut q_glow: Query<&mut TextColor, (With<TitleGlow>, Without<TitleText>)>,
) {
    // Palette
    let dim_border = Color::srgb(0.22, 0.22, 0.28);
    let cyan = Color::srgb(0.20, 0.90, 0.95);
    let purple = Color::srgb(0.85, 0.25, 0.95);
    let red = Color::srgb(0.95, 0.35, 0.45);
    let green = Color::srgb(0.35, 0.92, 0.55);

    // Subtle pulse for title + name box border
    let pulse = (time.elapsed_secs() * 1.3).sin() * 0.5 + 0.5;
    let title_main = Color::srgb(
        lerp(0.95, 0.20, pulse),
        lerp(0.95, 0.90, pulse),
        lerp(0.98, 0.95, pulse),
    );
    let title_glow = Color::srgba(
        lerp(0.85, 0.20, pulse),
        lerp(0.25, 0.90, pulse),
        lerp(0.95, 0.95, pulse),
        lerp(0.35, 0.55, pulse),
    );

    for mut c in &mut q_title {
        c.0 = title_main;
    }
    for mut c in &mut q_glow {
        c.0 = title_glow;
    }

    // Name box pulse border
    let name_border = Color::srgb(
        lerp(0.22, 0.35, pulse),
        lerp(0.22, 0.85, pulse),
        lerp(0.28, 0.95, pulse),
    );
    for mut bc in &mut q_name_box {
        *bc = BorderColor::all(name_border);
    }

    // Buttons
    let hovered_bg = Color::srgb(0.20, 0.20, 0.24);
    let pressed_bg = Color::srgb(0.24, 0.24, 0.30);

    for (i, mut bg, mut border, start, quit, diff) in &mut q_buttons {
        // Defaults
        let mut base_bg = Color::srgb(0.14, 0.14, 0.16);
        let mut base_border = dim_border;

        if start.is_some() {
            base_bg = Color::srgb(0.12, 0.18, 0.20);
            base_border = Color::srgb(0.20, 0.60, 0.65);
        } else if quit.is_some() {
            base_bg = Color::srgb(0.16, 0.12, 0.14);
            base_border = Color::srgb(0.55, 0.22, 0.28);
        } else if let Some(d) = diff {
            let accent = match d.0 {
                Difficulty::Easy => green,
                Difficulty::Normal => cyan,
                Difficulty::Hard => red,
            };

            if d.0 == config.difficulty {
                base_bg = Color::srgb(0.16, 0.16, 0.20);
                base_border = accent;
            } else {
                base_bg = Color::srgb(0.13, 0.13, 0.15);
                base_border = dim_border;
            }
        }

        match *i {
            Interaction::Pressed => {
                bg.0 = pressed_bg;
                *border = BorderColor::all(purple);
            }
            Interaction::Hovered => {
                bg.0 = hovered_bg;
                *border = BorderColor::all(cyan);
            }
            Interaction::None => {
                bg.0 = base_bg;
                *border = BorderColor::all(base_border);
            }
        }
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

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
