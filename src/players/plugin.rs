use bevy::prelude::*;
use crate::players::systems::*;
use crate::states::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::InGame)
            .with_system(start_up_player_system.system()))
        .add_system_set(SystemSet::on_update(GameState::InGame)
            .with_system(move_system.system())
            .with_system(animation_system.system()));
    }
}
