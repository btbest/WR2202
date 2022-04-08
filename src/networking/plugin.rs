use bevy::prelude::*;
use crate::networking::systems::*;


pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_session);
    }
}