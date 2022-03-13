use bevy::prelude::*;
use crate::interaction::systems::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_detection_system);
    }
}
