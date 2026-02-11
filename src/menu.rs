use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

#[derive(Component)]
struct MenuUi;

#[derive(Component)]
struct PlayButton;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

fn setup_menu(mut commands: Commands) {
    // Root container
    commands
        .spawn((
            MenuUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(14.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
        ))
        .with_children(|ui| {
            ui.spawn((
                MenuUi,
                Text::new("Fever Dream"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            ui.spawn((
                MenuUi,
                Text::new(
                    "Collect the memories.\nWASD/Arrows to move.\n1/2/3 to change mood.",
                ),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.92, 0.92, 0.92)),
            ));

            ui.spawn((
                MenuUi,
                Button,
                PlayButton,
                BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
                Node {
                    width: Val::Px(180.0),
                    height: Val::Px(64.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|b| {
                b.spawn((
                    MenuUi,
                    Text::new("Play"),
                    TextFont {
                        font_size: 42.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });

            ui.spawn((
                MenuUi,
                Text::new("Tip: Sideways mood is chaotic."),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
            ));
        });
}

fn menu_input(
    mut next_state: ResMut<NextState<GameState>>,
    mut q: Query<(&Interaction, &mut BackgroundColor), (With<PlayButton>, Changed<Interaction>)>,
) {
    for (i, mut bg) in &mut q {
        match *i {
            Interaction::Pressed => next_state.set(GameState::Playing),
            Interaction::Hovered => *bg = BackgroundColor(Color::srgb(0.26, 0.26, 0.26)),
            Interaction::None => *bg = BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
        }
    }
}

fn cleanup_menu(mut commands: Commands, q: Query<Entity, With<MenuUi>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
