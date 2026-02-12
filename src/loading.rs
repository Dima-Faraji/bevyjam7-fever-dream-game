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

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        // Load assets, then move to Menu
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<TextureAssets>(),
        );

        // Optional: visible “Loading…” UI
        app.add_systems(OnEnter(GameState::Loading), setup_loading_ui);
        app.add_systems(OnExit(GameState::Loading), cleanup_loading_ui);
    }
}

fn setup_loading_ui(mut commands: Commands) {
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
                LoadingTag, // tag child too so cleanup is easy
                Text::new("Loading..."),
                TextFont {
                    font_size: 42.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn cleanup_loading_ui(mut commands: Commands, q: Query<Entity, With<LoadingTag>>) {
    for e in &q {
        commands.entity(e).despawn();
    }
}
