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
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(start_menu.system()))
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(start_game_by_pressing_return_system.system()))
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_score_counter.system()))
        .add_system_set(SystemSet::on_update(GameState::InGame).with_system(score_counter.system()))
        .add_system_set(SystemSet::on_enter(GameState::WonBy('L')).with_system(gameover_menu.system()))
        .add_system_set(SystemSet::on_enter(GameState::WonBy('R')).with_system(gameover_menu.system()))
        .add_system_set(SystemSet::on_update(GameState::WonBy('L')).with_system(start_game_by_pressing_return_system.system()))
        .add_system_set(SystemSet::on_update(GameState::WonBy('R')).with_system(start_game_by_pressing_return_system.system()));
    }
}