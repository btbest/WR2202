use bevy::prelude::*;
use crate::players::systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_up_player_system);
        app.add_system(move_system);
        app.add_system(animation_system);
    }
}
