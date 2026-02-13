use bevy::prelude::*;

mod game_control;
use game_control::GameControl;
use game_control::get_movement;

pub struct ActionsPlugin;

#[derive(Resource, Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .add_systems(Update, set_movement_actions);
    }
}

pub fn set_movement_actions(mut actions: ResMut<Actions>, input: Res<ButtonInput<KeyCode>>) {
    let horizontal =
        get_movement(GameControl::Right, &input) - get_movement(GameControl::Left, &input);
    let vertical = get_movement(GameControl::Up, &input) - get_movement(GameControl::Down, &input);

    let movement = Vec2::new(horizontal, vertical);
    actions.player_movement = (movement != Vec2::ZERO).then_some(movement.normalize_or_zero());
}
