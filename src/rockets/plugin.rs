use bevy::prelude::*;
use crate::rockets::systems::*;

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_up_rockets);
        app.add_system(spawn_rocket_l);
        app.add_system(spawn_rocket_r);
        app.add_system(rocket_movement_system_l);
        app.add_system(rocket_movement_system_r);
        app.add_system(rocket_animation_system_l);
        app.add_system(rocket_animation_system_r);
        app.add_system(rocket_deletion_system);
        app.add_system(collision_detection_system);
    }
}
