use bevy::prelude::*;
use crate::rockets::systems::*;
use crate::states::GameState;

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::InGame)
            .with_system(start_up_rockets.system()))
        .add_system_set(SystemSet::on_update(GameState::InGame)
            .with_system(spawn_rocket.system())
            .with_system(rocket_movement_system.system())
            .with_system(rocket_animation_system.system())
            .with_system(rocket_offscreen_system.system()));
    }
}
