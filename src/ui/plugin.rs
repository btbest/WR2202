use bevy::prelude::*;
use crate::ui::systems::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_start_up_system);
        app.add_system(text_update_system);
    }
}
