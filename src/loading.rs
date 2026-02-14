use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

pub struct LoadingPlugin;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}

#[derive(Component)]
struct LoadingTag;

#[derive(Component)]
struct LoadingIconRow;

#[derive(Resource, Default)]
struct LoadingUiState {
    icons_spawned: bool,
}

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        // Load assets, then move to Menu
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<TextureAssets>(),
        );

        // Visible “Loading…” UI
        app.add_systems(OnEnter(GameState::Loading), setup_loading_ui);
        // Spawn icons once assets are actually available
        app.add_systems(
            Update,
            spawn_loading_icons.run_if(in_state(GameState::Loading)),
        );
        app.add_systems(OnExit(GameState::Loading), cleanup_loading_ui);
    }
}

fn setup_loading_ui(mut commands: Commands) {
    // Reset per-entry UI state (safe if Loading is entered again)
    commands.insert_resource(LoadingUiState::default());

    commands
        .spawn((
            LoadingTag,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.10)),
        ))
        .with_children(|ui| {
            ui.spawn((
                LoadingTag,
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(18.0),
                    ..default()
                },
            ))
            .with_children(|col| {
                col.spawn((
                    LoadingTag,
                    Text::new("Loading..."),
                    TextFont {
                        font_size: 42.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                // Icons will be inserted here once TextureAssets exists
                col.spawn((
                    LoadingTag,
                    LoadingIconRow,
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(18.0),
                        ..default()
                    },
                ));
            });
        });
}

fn spawn_loading_icons(
    mut commands: Commands,
    assets: Option<Res<TextureAssets>>,
    mut ui_state: ResMut<LoadingUiState>,
    q_row: Query<Entity, With<LoadingIconRow>>,
) {
    if ui_state.icons_spawned {
        return;
    }

    let Some(assets) = assets else {
        // Not loaded yet this frame
        return;
    };

    let Ok(row) = q_row.single() else {
        return;
    };

    ui_state.icons_spawned = true;

    commands.entity(row).with_children(|row| {
        row.spawn((
            LoadingTag,
            ImageNode::new(assets.bevy.clone()),
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                ..default()
            },
        ));

        row.spawn((
            LoadingTag,
            ImageNode::new(assets.github.clone()),
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                ..default()
            },
        ));
    });
}

fn cleanup_loading_ui(mut commands: Commands, q: Query<Entity, With<LoadingTag>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<LoadingUiState>();
}
