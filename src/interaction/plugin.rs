use bevy::prelude::*;
use crate::interaction::systems::*;
use crate::states::GameState;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(GameState::InGame)
            .with_system(collision_detection_system.system()));
    }
}
