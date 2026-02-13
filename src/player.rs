use crate::actions::Actions;
use crate::{Difficulty, GameConfig, GameState};

use bevy::app::AppExit;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::ParamSet;
use bevy::prelude::*;

/// Gameplay + countdown + game over
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct WorldBounds;

#[derive(Component)]
struct BackToMenuButton;

#[derive(Component)]
struct DifficultyChoiceButton(Difficulty);

#[derive(Component)]
struct PlayingEntity; // tag EVERYTHING spawned for Countdown/Playing

#[derive(Component)]
struct GameOverEntity; // tag EVERYTHING spawned for GameOver overlay

#[derive(Component)]
struct HudMood;
#[derive(Component)]
struct HudScore;
#[derive(Component)]
struct HudTime;
#[derive(Component)]
struct HurryText;

#[derive(Component)]
struct CountdownText;

#[derive(Component)]
struct ReplayButton;
#[derive(Component)]
struct QuitButton;

#[derive(Component)]
struct Memory;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Normal,
    Heavy,
    Sideways,
}

impl Default for Mood {
    fn default() -> Self {
        Mood::Normal
    }
}

#[derive(Resource, Default)]
struct Score(pub u32);

#[derive(Resource)]
struct CountdownTimer(pub Timer);

#[derive(Resource)]
struct GameTimer(pub Timer);

#[derive(Resource, Default)]
struct LowTimeAlerted(pub bool);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Mood>()
            .init_resource::<Score>()
            .init_resource::<LowTimeAlerted>()
            // Enter states
            .add_systems(OnEnter(GameState::Countdown), enter_countdown)
            .add_systems(OnEnter(GameState::Playing), enter_playing)
            .add_systems(OnEnter(GameState::GameOver), setup_game_over)
            // Update systems (always scheduled; gated internally by State<GameState>)
            .add_systems(Update, tick_countdown)
            .add_systems(Update, countdown_input_skip)
            .add_systems(Update, mood_input)
            .add_systems(Update, move_player)
            .add_systems(Update, clamp_player)
            .add_systems(Update, collect_memories)
            .add_systems(Update, tick_game_timer)
            .add_systems(Update, update_hud)
            .add_systems(Update, check_game_over)
            .add_systems(Update, game_over_input_keys)
            .add_systems(Update, game_over_buttons)
            .add_systems(Update, game_over_visuals)
            // Cleanup when leaving game over (this is where we also remove the play world)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
            .add_systems(OnExit(GameState::GameOver), cleanup_play_world);
    }
}

/* ----------------------- ENTER COUNTDOWN ----------------------- */

fn enter_countdown(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut mood: ResMut<Mood>,
    mut alerted: ResMut<LowTimeAlerted>,
    q_play: Query<Entity, With<PlayingEntity>>,
    q_over: Query<Entity, With<GameOverEntity>>,
) {
    // Safety cleanup (prevents duplicates if something went wrong earlier)
    for e in &q_over {
        commands.entity(e).despawn();
    }
    for e in &q_play {
        commands.entity(e).despawn();
    }

    // Reset run data
    score.0 = 0;
    *mood = Mood::Normal;
    alerted.0 = false;

    // Build world
    let half_w = 520.0;
    let half_h = 300.0;

    commands.spawn((
        PlayingEntity,
        WorldBounds,
        Transform::from_xyz(half_w, half_h, 0.0),
        GlobalTransform::default(),
        Visibility::Hidden,
        InheritedVisibility::default(),
    ));

    // background
    commands.spawn((
        PlayingEntity,
        Sprite {
            color: Color::srgb(0.18, 0.18, 0.20),
            custom_size: Some(Vec2::new(half_w * 2.0, half_h * 2.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // border walls
    let wall = 22.0;
    let wall_color = Color::srgb(0.10, 0.10, 0.11);

    commands.spawn((
        PlayingEntity,
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(half_w * 2.0 + wall * 2.0, wall)),
            ..default()
        },
        Transform::from_xyz(0.0, half_h + wall / 2.0, 0.5),
    ));
    commands.spawn((
        PlayingEntity,
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(half_w * 2.0 + wall * 2.0, wall)),
            ..default()
        },
        Transform::from_xyz(0.0, -half_h - wall / 2.0, 0.5),
    ));
    commands.spawn((
        PlayingEntity,
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(wall, half_h * 2.0)),
            ..default()
        },
        Transform::from_xyz(-half_w - wall / 2.0, 0.0, 0.5),
    ));
    commands.spawn((
        PlayingEntity,
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(wall, half_h * 2.0)),
            ..default()
        },
        Transform::from_xyz(half_w + wall / 2.0, 0.0, 0.5),
    ));

    // Player
    commands.spawn((
        PlayingEntity,
        Player,
        Sprite {
            color: Color::srgb(0.95, 0.2, 0.6),
            custom_size: Some(Vec2::new(44.0, 44.0)),
            ..default()
        },
        Transform::from_xyz(-420.0, 0.0, 1.0),
    ));

    // HUD top bar
    commands
        .spawn((
            PlayingEntity,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(64.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(18.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.09)),
        ))
        .with_children(|ui| {
            ui.spawn((
                PlayingEntity,
                HudMood,
                Text::new("Mood: Normal (1/2/3)"),
                TextFont {
                    font_size: 26.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            ui.spawn((
                PlayingEntity,
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    ..default()
                },
            ))
            .with_children(|right| {
                right.spawn((
                    PlayingEntity,
                    HudScore,
                    Text::new("Score: 0"),
                    TextFont {
                        font_size: 26.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
                right.spawn((
                    PlayingEntity,
                    HudTime,
                    Text::new("Time: --"),
                    TextFont {
                        font_size: 22.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });

    // Low-time alert text
    commands.spawn((
        PlayingEntity,
        HurryText,
        Text::new(""),
        TextFont {
            font_size: 42.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.3, 0.3)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(84.0),
            left: Val::Px(18.0),
            ..default()
        },
    ));

    // Countdown center text
    commands.spawn((
        PlayingEntity,
        CountdownText,
        Text::new(""),
        TextFont {
            font_size: 120.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            left: Val::Percent(0.0),
            width: Val::Percent(100.0),
            ..default()
        },
    ));

    // Start countdown timer
    commands.insert_resource(CountdownTimer(Timer::from_seconds(4.0, TimerMode::Once)));
}

/* ----------------------- COUNTDOWN UPDATE ----------------------- */

fn tick_countdown(
    state: Res<State<GameState>>,
    time: Res<Time>,
    mut timer: Option<ResMut<CountdownTimer>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut q_text: Query<&mut Text, With<CountdownText>>,
) {
    if *state.get() != GameState::Countdown {
        return;
    }

    let Some(mut timer) = timer else {
        return;
    };
    timer.0.tick(time.delta());

    let remaining = (4.0 - timer.0.elapsed_secs()).ceil().max(0.0) as i32;
    let msg = match remaining {
        4 => "3",
        3 => "2",
        2 => "1",
        1 => "GO!",
        _ => "",
    };

    for mut t in &mut q_text {
        *t = Text::new(msg);
    }

    if timer.0.just_finished() {
        next_state.set(GameState::Playing);
    }
}

fn countdown_input_skip(
    state: Res<State<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *state.get() != GameState::Countdown {
        return;
    }

    if keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
}

/* ----------------------- ENTER PLAYING ----------------------- */

fn enter_playing(
    mut commands: Commands,
    config: Res<GameConfig>,
    bounds_q: Query<&Transform, With<WorldBounds>>,
    q_mem: Query<Entity, With<Memory>>,
    mut q_cd: Query<&mut Text, With<CountdownText>>,
) {
    // Clear countdown text
    for mut t in &mut q_cd {
        *t = Text::new("");
    }

    // Remove any leftover memories (safety)
    for e in &q_mem {
        commands.entity(e).despawn();
    }

    // Difficulty affects time + number of memories
    let time_limit = match config.difficulty {
        Difficulty::Easy => 60.0,
        Difficulty::Normal => 45.0,
        Difficulty::Hard => 30.0,
    };
    commands.insert_resource(GameTimer(Timer::from_seconds(time_limit, TimerMode::Once)));

    let Ok(bounds) = bounds_q.single() else {
        return;
    };
    let half_w = bounds.translation.x;
    let half_h = bounds.translation.y;

    let mut points = vec![
        Vec2::new(-half_w + 120.0, half_h - 90.0),
        Vec2::new(-80.0, half_h - 60.0),
        Vec2::new(half_w - 140.0, half_h - 140.0),
        Vec2::new(half_w - 160.0, -half_h + 120.0),
        Vec2::new(-half_w + 170.0, -half_h + 110.0),
        Vec2::new(40.0, -half_h + 90.0),
        Vec2::new(0.0, 0.0),
    ];

    match config.difficulty {
        Difficulty::Easy => points.truncate(5),
        Difficulty::Normal => {}
        Difficulty::Hard => {
            points.extend([
                Vec2::new(-half_w + 260.0, 40.0),
                Vec2::new(half_w - 240.0, -20.0),
                Vec2::new(0.0, half_h - 170.0),
                Vec2::new(0.0, -half_h + 170.0),
            ]);
        }
    }

    for p in points {
        commands.spawn((
            PlayingEntity,
            Memory,
            Sprite {
                color: Color::srgb(0.35, 0.9, 0.95),
                custom_size: Some(Vec2::new(22.0, 22.0)),
                ..default()
            },
            Transform::from_xyz(p.x, p.y, 1.0),
        ));
    }
}

/* ----------------------- PLAYING UPDATE ----------------------- */

fn mood_input(
    state: Res<State<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mood: ResMut<Mood>,
) {
    if *state.get() != GameState::Playing {
        return;
    }

    if keys.just_pressed(KeyCode::Digit1) {
        *mood = Mood::Normal;
    } else if keys.just_pressed(KeyCode::Digit2) {
        *mood = Mood::Heavy;
    } else if keys.just_pressed(KeyCode::Digit3) {
        *mood = Mood::Sideways;
    }
}

fn move_player(
    state: Res<State<GameState>>,
    time: Res<Time>,
    actions: Res<Actions>,
    mood: Res<Mood>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    if *state.get() != GameState::Playing {
        return;
    }

    let Ok(mut t) = player_q.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let base_speed = 280.0;
    let input = actions.player_movement.unwrap_or(Vec2::ZERO);

    let (speed_mul, gravity): (f32, Vec2) = match *mood {
        Mood::Normal => (1.0, Vec2::new(0.0, -60.0)),
        Mood::Heavy => (0.75, Vec2::new(0.0, -140.0)),
        Mood::Sideways => (0.9, Vec2::new(-120.0, 0.0)),
    };

    let input = match *mood {
        Mood::Sideways => Vec2::new(input.y, -input.x),
        _ => input,
    };

    let move_step = input * (base_speed * speed_mul) * dt;
    let gravity_step = gravity * dt;

    t.translation += Vec3::new(
        move_step.x + gravity_step.x,
        move_step.y + gravity_step.y,
        0.0,
    );
}

fn clamp_player(
    state: Res<State<GameState>>,
    bounds_q: Query<&Transform, (With<WorldBounds>, Without<Player>)>,
    mut player_q: Query<&mut Transform, (With<Player>, Without<WorldBounds>)>,
) {
    if *state.get() != GameState::Playing {
        return;
    }

    let Ok(bounds) = bounds_q.single() else {
        return;
    };
    let Ok(mut p) = player_q.single_mut() else {
        return;
    };

    let half_w = bounds.translation.x;
    let half_h = bounds.translation.y;
    let margin = 28.0;

    p.translation.x = p.translation.x.clamp(-half_w + margin, half_w - margin);
    p.translation.y = p.translation.y.clamp(-half_h + margin, half_h - margin);
}

fn collect_memories(
    state: Res<State<GameState>>,
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    memories_q: Query<(Entity, &Transform), With<Memory>>,
    mut score: ResMut<Score>,
) {
    if *state.get() != GameState::Playing {
        return;
    }

    let Ok(p) = player_q.single() else {
        return;
    };
    let pr = 26.0;

    for (e, t) in &memories_q {
        let d = p.translation.truncate().distance(t.translation.truncate());
        if d <= pr {
            score.0 += 1;
            commands.entity(e).despawn();
        }
    }
}

fn tick_game_timer(
    state: Res<State<GameState>>,
    time: Res<Time>,
    timer: Option<ResMut<GameTimer>>,
) {
    if *state.get() != GameState::Playing {
        return;
    }
    let Some(mut timer) = timer else {
        return;
    };
    timer.0.tick(time.delta());
}

/* ----------------------- HUD UPDATE ----------------------- */

fn update_hud(
    state: Res<State<GameState>>,
    mood: Res<Mood>,
    score: Res<Score>,
    timer: Option<Res<GameTimer>>,
    mut alerted: ResMut<LowTimeAlerted>,
    mut set: ParamSet<(
        Query<&mut Text, With<HudMood>>,
        Query<&mut Text, With<HudScore>>,
        Query<(&mut Text, &mut TextColor), With<HudTime>>,
        Query<&mut Text, With<HurryText>>,
    )>,
) {
    if *state.get() != GameState::Playing && *state.get() != GameState::Countdown {
        return;
    }

    let mood_label = match *mood {
        Mood::Normal => "Normal",
        Mood::Heavy => "Heavy",
        Mood::Sideways => "Sideways",
    };

    for mut t in set.p0().iter_mut() {
        *t = Text::new(format!("Mood: {mood_label} (1/2/3)"));
    }

    for mut t in set.p1().iter_mut() {
        *t = Text::new(format!("Score: {}", score.0));
    }

    if *state.get() == GameState::Countdown {
        for (mut t, mut color) in set.p2().iter_mut() {
            *t = Text::new("Time: --");
            color.0 = Color::WHITE;
        }
        for mut ht in set.p3().iter_mut() {
            *ht = Text::new("");
        }
        return;
    }

    if let Some(timer) = timer {
        let total = timer.0.duration().as_secs_f32();
        let elapsed = timer.0.elapsed_secs();
        let remaining = (total - elapsed).max(0.0);

        for (mut t, mut color) in set.p2().iter_mut() {
            *t = Text::new(format!("Time: {:.1}", remaining));
            if remaining <= 7.0 {
                color.0 = Color::srgb(1.0, 0.3, 0.3);
                alerted.0 = true;
            } else {
                color.0 = Color::WHITE;
            }
        }

        for mut ht in set.p3().iter_mut() {
            *ht = Text::new(if remaining <= 7.0 { "HURRY UP!" } else { "" });
        }
    }
}

fn check_game_over(
    state: Res<State<GameState>>,
    timer: Option<Res<GameTimer>>,
    memories_q: Query<Entity, With<Memory>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if *state.get() != GameState::Playing {
        return;
    }
    let Some(timer) = timer else {
        return;
    };

    if timer.0.just_finished() || memories_q.is_empty() {
        next_state.set(GameState::GameOver);
    }
}

/* ----------------------- GAME OVER ----------------------- */

fn setup_game_over(mut commands: Commands, score: Res<Score>, config: Res<GameConfig>) {
    let name = if config.player_name.trim().is_empty() {
        "Player"
    } else {
        config.player_name.trim()
    };

    commands
        .spawn((
            GameOverEntity,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
        ))
        .with_children(|ui| {
            ui.spawn((
                GameOverEntity,
                Node {
                    width: Val::Px(620.0),
                    padding: UiRect::all(Val::Px(24.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(14.0),
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(Val::Px(16.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.10, 0.10, 0.12)),
            ))
            .with_children(|card| {
                card.spawn((
                    GameOverEntity,
                    Text::new(format!("Nice run, {name}.")),
                    TextFont {
                        font_size: 44.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                card.spawn((
                    GameOverEntity,
                    Text::new(format!("Final Score: {}", score.0)),
                    TextFont {
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                card.spawn((
                    GameOverEntity,
                    Text::new("Play again? Pick a difficulty:"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.85, 0.85, 0.85)),
                ));

                // Difficulty row
                card.spawn((
                    GameOverEntity,
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                ))
                .with_children(|row| {
                    for d in [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard] {
                        row.spawn((
                            GameOverEntity,
                            Button,
                            DifficultyChoiceButton(d),
                            BackgroundColor(Color::srgb(0.18, 0.18, 0.20)),
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(46.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                border_radius: BorderRadius::all(Val::Px(12.0)),
                                ..default()
                            },
                        ))
                        .with_child((
                            GameOverEntity,
                            Text::new(d.label()),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    }
                });

                // Action buttons
                card.spawn((
                    GameOverEntity,
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                ))
                .with_children(|row| {
                    row.spawn((
                        GameOverEntity,
                        Button,
                        ReplayButton,
                        BackgroundColor(Color::srgb(0.18, 0.18, 0.20)),
                        Node {
                            width: Val::Px(180.0),
                            height: Val::Px(56.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            border_radius: BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        },
                    ))
                    .with_child((
                        GameOverEntity,
                        Text::new("Play Again"),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    row.spawn((
                        GameOverEntity,
                        Button,
                        BackToMenuButton,
                        BackgroundColor(Color::srgb(0.18, 0.18, 0.20)),
                        Node {
                            width: Val::Px(180.0),
                            height: Val::Px(56.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            border_radius: BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        },
                    ))
                    .with_child((
                        GameOverEntity,
                        Text::new("Menu"),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    row.spawn((
                        GameOverEntity,
                        Button,
                        QuitButton,
                        BackgroundColor(Color::srgb(0.18, 0.18, 0.20)),
                        Node {
                            width: Val::Px(140.0),
                            height: Val::Px(56.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            border_radius: BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        },
                    ))
                    .with_child((
                        GameOverEntity,
                        Text::new("Quit"),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

                card.spawn((
                    GameOverEntity,
                    Text::new("Keys: Enter/R = Play Again    M = Menu    Esc/Q = Quit"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.75, 0.75, 0.78)),
                ));
            });
        });
}

fn game_over_input_keys(
    state: Res<State<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if *state.get() != GameState::GameOver {
        return;
    }

    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Countdown);
    }

    if keys.just_pressed(KeyCode::KeyM) {
        next_state.set(GameState::Menu);
    }

    if keys.just_pressed(KeyCode::KeyQ) || keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn game_over_buttons(
    state: Res<State<GameState>>,
    mut q: Query<
        (
            &Interaction,
            Option<&ReplayButton>,
            Option<&QuitButton>,
            Option<&BackToMenuButton>,
            Option<&DifficultyChoiceButton>,
        ),
        (Changed<Interaction>, With<Button>, With<GameOverEntity>),
    >,
    mut config: ResMut<GameConfig>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    if *state.get() != GameState::GameOver {
        return;
    }

    for (i, replay, quit, menu, diff) in &mut q {
        if *i != Interaction::Pressed {
            continue;
        }

        if let Some(d) = diff {
            config.difficulty = d.0;
            continue;
        }

        if replay.is_some() {
            next_state.set(GameState::Countdown);
        } else if menu.is_some() {
            next_state.set(GameState::Menu);
        } else if quit.is_some() {
            exit.write(AppExit::Success);
        }
    }
}

fn game_over_visuals(
    state: Res<State<GameState>>,
    config: Res<GameConfig>,
    mut q: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&ReplayButton>,
            Option<&QuitButton>,
            Option<&BackToMenuButton>,
            Option<&DifficultyChoiceButton>,
        ),
        (With<Button>, With<GameOverEntity>),
    >,
) {
    if *state.get() != GameState::GameOver {
        return;
    }

    for (interaction, mut bg, replay, quit, menu, diff) in &mut q {
        let base = if replay.is_some() || quit.is_some() || menu.is_some() {
            Color::srgb(0.18, 0.18, 0.20)
        } else if let Some(d) = diff {
            if d.0 == config.difficulty {
                Color::srgb(0.26, 0.26, 0.30)
            } else {
                Color::srgb(0.16, 0.16, 0.18)
            }
        } else {
            Color::srgb(0.18, 0.18, 0.20)
        };

        let hovered = Color::srgb(0.24, 0.24, 0.26);
        let pressed = Color::srgb(0.28, 0.28, 0.31);

        *bg = BackgroundColor(match *interaction {
            Interaction::Pressed => pressed,
            Interaction::Hovered => hovered,
            Interaction::None => base,
        });
    }
}

/* ----------------------- CLEANUP ----------------------- */

fn cleanup_play_world(mut commands: Commands, q: Query<Entity, With<PlayingEntity>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}

fn cleanup_game_over(mut commands: Commands, q: Query<Entity, With<GameOverEntity>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
