use bevy::prelude::*;
use crate::ui::systems::*;
use crate::states::GameState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(ui_start_up_system.system())
        // .add_system_set(SystemSet::on_enter(GameState::InGame)
        //     .with_system()
        .add_system_set(SystemSet::on_update(GameState::InGame)
            .with_system(text_update_system.system()))
        .add_system_set(SystemSet::on_update(GameState::Menu)
            .with_system(restart_game.system()));
    }
}
