//#![allow(unused)]
use bevy::prelude::*;
mod players;
mod rockets;
use crate::players::plugin::PlayerPlugin;
use crate::rockets::plugin::RocketPlugin;


fn main() {
    App::new()
        // This gives you a game-loop, a window, audio, asset server and so on... (basically the default engine)
        // See https://github.com/bevyengine/bevy/blob/v0.6.0/examples/app/plugin_group.rs for more info on plugin groups
        .add_plugins(DefaultPlugins)
        // This adds a configuration resource to the App
        // More on resources: https://bevy-cheatbook.github.io/programming/res
        // Turn off MSAA (default is 4 samples)
        // see https://bevy-cheatbook.github.io/builtins.html?highlight=MSAA#configuration-resources
        .insert_resource(Msaa { samples: 1 })
        // This system runs once on startup
        .add_startup_system(start_up)
        // This is an example of how to structure your game in multiple files.
        // We moved a system into a custom plugin.
        .add_plugin(RocketPlugin)
        .add_plugin(PlayerPlugin)
        // Let's goooooo
        .run();
}

fn start_up(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // Spawn a 2D camera
    // Comment changed by Bene
    // `spawn_bundle` spawns an entity and then adds a bunch of Components (the bundle) to it
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // Text with one section
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    right: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "0 - 0",
                TextStyle {
                    font: assets.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(CounterText {
            score_l: 0,
            score_r: 0
        });
}

#[derive(Component)]
struct CounterText {
    score_l: u8,
    score_r: u8
}

// fn text_update_system(mut query: Query<&mut Text, &CounterText>) {
//     for mut text in query.iter_mut() {
//         if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
//             if let Some(average) = fps.average() {
//                 // Update the value of the second section
//                 text.sections[1].value = format!("{:.2}", average);
//             }
//         }
//     }
// }