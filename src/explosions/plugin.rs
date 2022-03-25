use bevy::prelude::*;
use crate::explosions::systems::*;
use crate::states::GameState;

pub struct ExplosionsPlugin;

impl Plugin for ExplosionsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::InGame)
            .with_system(start_up_explosions.system()))
        .add_system_set(SystemSet::on_update(GameState::InGame)
            .with_system(explosion_animation_and_despawn_system.system()));
    }
}
