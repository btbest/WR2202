use bevy::prelude::*;
use crate::explosions::systems::*;

pub struct ExplosionsPlugin;

impl Plugin for ExplosionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_up_explosions);
        app.add_system(explosion_animation_and_despawn_system);
    }
}
