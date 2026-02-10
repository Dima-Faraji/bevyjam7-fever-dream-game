use crate::GameState;
use crate::actions::Actions;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct MoodText;

#[derive(Component)]
struct WorldBounds;

#[derive(Component)]
struct PlayingEntity; // tag everything spawned in Playing so we can clean it up

/// "Gravity is Mood"
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

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Mood>()
            .add_systems(
                OnEnter(GameState::Playing),
                (setup_playing_world, spawn_player, spawn_mood_ui),
            )
            .add_systems(
                Update,
                (mood_input, update_mood_ui, move_player, clamp_player)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_playing);
    }
}

fn setup_playing_world(mut commands: Commands) {
    let half_w = 420.0;
    let half_h = 260.0;
    let wall_thickness = 20.0;

    // Store bounds on an entity.
    commands.spawn((
        PlayingEntity,
        WorldBounds,
        Transform::from_xyz(half_w, half_h, 0.0),
        GlobalTransform::default(),
        Visibility::Hidden,
        InheritedVisibility::default(),
    ));

    // Floor
    commands.spawn((
        PlayingEntity,
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(half_w * 2.0 + wall_thickness * 2.0, wall_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, -half_h - wall_thickness / 2.0, 0.5),
    ));

    // Ceiling
    commands.spawn((
        PlayingEntity,
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(half_w * 2.0 + wall_thickness * 2.0, wall_thickness)),
            ..default()
        },
        Transform::from_xyz(0.0, half_h + wall_thickness / 2.0, 0.5),
    ));

    // Left wall
    commands.spawn((
        PlayingEntity,
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(wall_thickness, half_h * 2.0)),
            ..default()
        },
        Transform::from_xyz(-half_w - wall_thickness / 2.0, 0.0, 0.5),
    ));

    // Right wall
    commands.spawn((
        PlayingEntity,
        Sprite {
            color: Color::srgb(0.2, 0.2, 0.2),
            custom_size: Some(Vec2::new(wall_thickness, half_h * 2.0)),
            ..default()
        },
        Transform::from_xyz(half_w + wall_thickness / 2.0, 0.0, 0.5),
    ));
}

fn spawn_player(mut commands: Commands) {
    info!("Entered Playing: spawning player");

    commands.spawn((
        PlayingEntity,
        Player,
        Sprite {
            color: Color::srgb(0.95, 0.2, 0.6),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn spawn_mood_ui(mut commands: Commands) {
    commands.spawn((
        PlayingEntity,
        MoodText,
        Text::new("Mood: Normal (1/2/3 or F1/F2/F3)"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn mood_input(keys: Res<ButtonInput<KeyCode>>, mut mood: ResMut<Mood>) {
    let new_mood = if keys.just_pressed(KeyCode::Digit1) || keys.just_pressed(KeyCode::F1) {
        Some(Mood::Normal)
    } else if keys.just_pressed(KeyCode::Digit2) || keys.just_pressed(KeyCode::F2) {
        Some(Mood::Heavy)
    } else if keys.just_pressed(KeyCode::Digit3) || keys.just_pressed(KeyCode::F3) {
        Some(Mood::Sideways)
    } else {
        None
    };

    if let Some(new_mood) = new_mood {
        if *mood != new_mood {
            *mood = new_mood;
            info!("Mood changed to: {:?}", *mood);
        }
    }
}

fn update_mood_ui(mood: Res<Mood>, mut q: Query<&mut Text, With<MoodText>>) {
    if !mood.is_changed() {
        return;
    }

    let label = match *mood {
        Mood::Normal => "Normal",
        Mood::Heavy => "Heavy",
        Mood::Sideways => "Sideways",
    };

    for mut text in &mut q {
        *text = Text::new(format!("Mood: {label} (1/2/3 or F1/F2/F3)"));
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mood: Res<Mood>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut t) = player_query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let base_speed = 260.0;

    let input = actions.player_movement.unwrap_or(Vec2::ZERO);

    let (speed_mul, gravity_vec): (f32, Vec2) = match *mood {
        Mood::Normal => (1.0, Vec2::new(0.0, -80.0)),
        Mood::Heavy => (0.7, Vec2::new(0.0, -160.0)),
        Mood::Sideways => (0.9, Vec2::new(-140.0, 0.0)),
    };

    let input = match *mood {
        Mood::Sideways => Vec2::new(input.y, -input.x),
        _ => input,
    };

    let move_step = input * (base_speed * speed_mul) * dt;
    let gravity_step = gravity_vec * dt;

    let delta = Vec3::new(move_step.x + gravity_step.x, move_step.y + gravity_step.y, 0.0);
    t.translation += delta;
}

fn clamp_player(
    bounds_q: Query<&Transform, (With<WorldBounds>, Without<Player>)>,
    mut player_q: Query<&mut Transform, (With<Player>, Without<WorldBounds>)>,
) {
    let Ok(bounds_t) = bounds_q.single() else { return; };
    let Ok(mut player_t) = player_q.single_mut() else { return; };

    let half_w = bounds_t.translation.x;
    let half_h = bounds_t.translation.y;

    let margin = 30.0;
    player_t.translation.x = player_t.translation.x.clamp(-half_w + margin, half_w - margin);
    player_t.translation.y = player_t.translation.y.clamp(-half_h + margin, half_h - margin);
}

fn cleanup_playing(mut commands: Commands, q: Query<Entity, With<PlayingEntity>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
