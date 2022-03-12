use bevy::prelude::*;
use crate::players::systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_up_player_system);
        app.add_system(move_system_l);
        app.add_system(move_system_r);
        app.add_system(animation_system_l);
        app.add_system(animation_system_r);
        app.add_system(collision_detection_system);
    }
}
