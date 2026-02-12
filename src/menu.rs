use bevy::app::AppExit;
use bevy::ecs::message::MessageWriter;
use bevy::prelude::*;

use crate::GameState;

pub struct MenuPlugin;

#[derive(Component)]
struct MenuTag;

#[derive(Component, Clone, Copy)]
enum MenuButton {
    Start,
    Quit,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            MenuTag,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(14.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.09)),
        ))
        .with_children(|ui| {
            ui.spawn((
                MenuTag,
                Text::new("Fever Dream"),
                TextFont {
                    font_size: 86.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            ui.spawn((
                MenuTag,
                Text::new("Collect the memories.\nWASD/Arrows to move.\n1/2/3 to change mood."),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.92, 0.92, 0.92)),
            ));

            // Start button
            ui.spawn((
                MenuTag,
                Button,
                MenuButton::Start,
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                Node {
                    width: Val::Px(220.0),
                    height: Val::Px(64.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|b| {
                b.spawn((
                    MenuTag,
                    Text::new("Start"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });

            // Quit button
            ui.spawn((
                MenuTag,
                Button,
                MenuButton::Quit,
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                Node {
                    width: Val::Px(220.0),
                    height: Val::Px(64.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|b| {
                b.spawn((
                    MenuTag,
                    Text::new("Quit"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

fn menu_input(
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
    mut q: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (With<Button>, Changed<Interaction>),
    >,
) {
    for (i, mut bg, kind) in &mut q {
        match *i {
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb(0.22, 0.22, 0.22));
                match kind {
                    MenuButton::Start => next_state.set(GameState::Countdown),
                    MenuButton::Quit => {
                        let _ = exit.write(AppExit::Success); // ignore MessageId
                    }
                }
            }
            Interaction::Hovered => *bg = BackgroundColor(Color::srgb(0.22, 0.22, 0.22)),
            Interaction::None => *bg = BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        }
    }
}

fn cleanup_menu(mut commands: Commands, q: Query<Entity, With<MenuTag>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
