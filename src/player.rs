use crate::actions::Actions;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct PlayingEntity;

#[derive(Component)]
struct WorldBounds;

#[derive(Component)]
struct MoodText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct TimerText;

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
pub struct Score(pub u32);

#[derive(Resource)]
pub struct RoundTimer(pub Timer);

impl Default for RoundTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(30.0, TimerMode::Once))
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Mood>()
            .init_resource::<Score>()
            .init_resource::<RoundTimer>()
            .add_systems(
                OnEnter(GameState::Playing),
                (
                    reset_round,
                    setup_playing_world,
                    spawn_player,
                    spawn_hud,
                    spawn_memories,
                ),
            )
            .add_systems(
                Update,
                (
                    mood_input,
                    move_player,
                    clamp_player,
                    collect_memories,
                    tick_timer,
                    update_hud,
                    check_round_end,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_playing);
    }
}

fn reset_round(mut score: ResMut<Score>, mut timer: ResMut<RoundTimer>, mut mood: ResMut<Mood>) {
    score.0 = 0;
    timer.0.reset();
    *mood = Mood::Normal;
}

fn setup_playing_world(mut commands: Commands) {
    let half_w = 420.0;
    let half_h = 260.0;
    let wall_thickness = 20.0;

    commands.spawn((
        PlayingEntity,
        WorldBounds,
        Transform::from_xyz(half_w, half_h, 0.0),
        GlobalTransform::default(),
        Visibility::Hidden,
        InheritedVisibility::default(),
    ));

    let wall_color = Color::srgb(0.18, 0.18, 0.18);

    // floor
    commands.spawn((
        PlayingEntity,
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(half_w * 2.0 + wall_thickness * 2.0, wall_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, -half_h - wall_thickness / 2.0, 0.5),
    ));

    // ceiling
    commands.spawn((
        PlayingEntity,
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(half_w * 2.0 + wall_thickness * 2.0, wall_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, half_h + wall_thickness / 2.0, 0.5),
    ));

    // left wall
    commands.spawn((
        PlayingEntity,
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(wall_thickness, half_h * 2.0)),
            ..default()
        },
        Transform::from_xyz(-half_w - wall_thickness / 2.0, 0.0, 0.5),
    ));

    // right wall
    commands.spawn((
        PlayingEntity,
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(wall_thickness, half_h * 2.0)),
            ..default()
        },
        Transform::from_xyz(half_w + wall_thickness / 2.0, 0.0, 0.5),
    ));
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        PlayingEntity,
        Player,
        Sprite {
            color: Color::srgb(0.95, 0.2, 0.6),
            custom_size: Some(Vec2::new(44.0, 44.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        PlayingEntity,
        MoodText,
        Text::new("Mood: Normal (1/2/3)"),
        TextFont {
            font_size: 26.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

    commands.spawn((
        PlayingEntity,
        ScoreText,
        Text::new("Score: 0"),
        TextFont {
            font_size: 26.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(12.0),
            ..default()
        },
    ));

    commands.spawn((
        PlayingEntity,
        TimerText,
        Text::new("Time: 30.0"),
        TextFont {
            font_size: 22.0,
            ..default()
        },
        TextColor(Color::srgb(0.92, 0.92, 0.92)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(44.0),
            right: Val::Px(12.0),
            ..default()
        },
    ));
}

fn spawn_memories(mut commands: Commands) {
    let points = [
        Vec2::new(-260.0, 160.0),
        Vec2::new(260.0, 160.0),
        Vec2::new(-260.0, -160.0),
        Vec2::new(260.0, -160.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 200.0),
        Vec2::new(0.0, -200.0),
        Vec2::new(-340.0, 0.0),
        Vec2::new(340.0, 0.0),
    ];

    for p in points {
        commands.spawn((
            PlayingEntity,
            Memory,
            Sprite {
                color: Color::srgb(0.35, 0.85, 0.95),
                custom_size: Some(Vec2::new(18.0, 18.0)),
                ..default()
            },
            Transform::from_xyz(p.x, p.y, 1.0),
        ));
    }
}

fn mood_input(keys: Res<ButtonInput<KeyCode>>, mut mood: ResMut<Mood>) {
    if keys.just_pressed(KeyCode::Digit1) {
        *mood = Mood::Normal;
    } else if keys.just_pressed(KeyCode::Digit2) {
        *mood = Mood::Heavy;
    } else if keys.just_pressed(KeyCode::Digit3) {
        *mood = Mood::Sideways;
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mood: Res<Mood>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut t) = player_q.single_mut() else { return; };

    let dt = time.delta_secs();
    let base_speed = 260.0;

    let input = actions.player_movement.unwrap_or(Vec2::ZERO);

    let (speed_mul, gravity_vec) = match *mood {
        Mood::Normal => (1.0, Vec2::new(0.0, -60.0)),
        Mood::Heavy => (0.75, Vec2::new(0.0, -140.0)),
        Mood::Sideways => (0.9, Vec2::new(-140.0, 0.0)),
    };

    let input = match *mood {
        Mood::Sideways => Vec2::new(input.y, -input.x),
        _ => input,
    };

    let move_step = input * (base_speed * speed_mul) * dt;
    let gravity_step = gravity_vec * dt;

    t.translation += Vec3::new(move_step.x + gravity_step.x, move_step.y + gravity_step.y, 0.0);
}

fn clamp_player(
    bounds_q: Query<&Transform, (With<WorldBounds>, Without<Player>)>,
    mut player_q: Query<&mut Transform, (With<Player>, Without<WorldBounds>)>,
) {
    let Ok(bounds_t) = bounds_q.single() else { return; };
    let Ok(mut player_t) = player_q.single_mut() else { return; };

    let half_w = bounds_t.translation.x;
    let half_h = bounds_t.translation.y;

    let margin = 28.0;
    player_t.translation.x = player_t.translation.x.clamp(-half_w + margin, half_w - margin);
    player_t.translation.y = player_t.translation.y.clamp(-half_h + margin, half_h - margin);
}

fn collect_memories(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player_q: Query<&Transform, With<Player>>,
    memories_q: Query<(Entity, &Transform), With<Memory>>,
) {
    let Ok(player_t) = player_q.single() else { return; };

    let pickup_dist2 = 34.0 * 34.0;
    for (e, t) in &memories_q {
        let d2 = player_t.translation.truncate().distance_squared(t.translation.truncate());
        if d2 <= pickup_dist2 {
            score.0 += 1;
            commands.entity(e).despawn();
        }
    }
}

fn tick_timer(time: Res<Time>, mut timer: ResMut<RoundTimer>) {
    timer.0.tick(time.delta());
}

fn update_hud(
    mood: Res<Mood>,
    score: Res<Score>,
    timer: Res<RoundTimer>,
    mut set: ParamSet<(
        Query<&mut Text, With<MoodText>>,
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<TimerText>>,
    )>,
) {
    let mood_label = match *mood {
        Mood::Normal => "Normal",
        Mood::Heavy => "Heavy",
        Mood::Sideways => "Sideways",
    };

    for mut t in &mut set.p0() {
        *t = Text::new(format!("Mood: {mood_label} (1/2/3)"));
    }

    for mut t in &mut set.p1() {
        *t = Text::new(format!("Score: {}", score.0));
    }

    let secs_left = (timer.0.duration().as_secs_f32() - timer.0.elapsed_secs()).max(0.0);
    for mut t in &mut set.p2() {
        *t = Text::new(format!("Time: {:.1}", secs_left));
    }
}

fn check_round_end(
    mut next_state: ResMut<NextState<GameState>>,
    timer: Res<RoundTimer>,
    memories_q: Query<Entity, With<Memory>>,
) {
    // ✅ Bevy 0.18 safe:
    if timer.0.just_finished() || memories_q.is_empty() {
        next_state.set(GameState::Menu);
    }
}

fn cleanup_playing(mut commands: Commands, q: Query<Entity, With<PlayingEntity>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
