use bevy::prelude::*;
use bevy::window::WindowMode;
mod players;
use crate::players::plugin::PlayerPlugin;
mod rockets;
use crate::rockets::plugin::RocketPlugin;
mod interaction;
use crate::interaction::plugin::InteractionPlugin;
mod ui;
use crate::ui::plugin::UIPlugin;
mod explosions;
use crate::explosions::plugin::ExplosionsPlugin;
mod states;
use crate::states::GameState;
mod networking;
use crate::networking::plugin::NetworkingPlugin;

fn main() {
    App::new()
        .add_plugin(NetworkingPlugin)
        .insert_resource(WindowDescriptor {
            title: "War Rockets 2202".to_string(),
            width: 800.0,
            height: 600.0,
            vsync: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        // This gives you a game-loop, a window, audio, asset server and so on... (basically the default engine)
        // See https://github.com/bevyengine/bevy/blob/v0.6.0/examples/app/plugin_group.rs for more info on plugin groups
        .add_plugins(DefaultPlugins)
        // This adds a configuration resource to the App
        // More on resources: https://bevy-cheatbook.github.io/programming/res
        // Turn off MSAA (default is 4 samples)
        // see https://bevy-cheatbook.github.io/builtins.html?highlight=MSAA#configuration-resources
        .insert_resource(Msaa { samples: 1 })
        // For a good States explanation, see https://github.com/bevyengine/bevy/pull/1059#issuecomment-744113314
        .add_state(GameState::Menu)
        // This system runs once on startup
        .add_startup_system(start_up)
        // Clean up screen when changing states
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(teardown))
        .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(teardown))
        .add_system_set(SystemSet::on_exit(GameState::WonBy('L')).with_system(teardown))
        .add_system_set(SystemSet::on_exit(GameState::WonBy('R')).with_system(teardown))
        // Exit on escape:
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(RocketPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(InteractionPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(ExplosionsPlugin)
        // Let's goooooo
        .run();
}

fn start_up(
    mut commands: Commands,
) {
    // Spawn a 2D camera
    // Comment changed by Bene
    // `spawn_bundle` spawns an entity and then adds a bunch of Components (the bundle) to it
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}