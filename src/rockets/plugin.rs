use bevy::prelude::*;
use crate::rockets::systems::*;

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_up_rockets);
        app.add_system(spawn_rocket);
        app.add_system(rocket_movement_system);
        app.add_system(rocket_animation_system);
        app.add_system(rocket_offscreen_system);
    }
}
