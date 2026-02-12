use bevy::prelude::*;

mod loading;
mod menu;
mod player;
mod actions;

pub use loading::*;
pub use menu::*;
pub use player::*;
pub use actions::*;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Countdown,
    Playing,
    GameOver,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(ClearColor(Color::srgb(0.12, 0.12, 0.12)))
            .add_plugins((LoadingPlugin, ActionsPlugin, MenuPlugin, PlayerPlugin));
    }
}
