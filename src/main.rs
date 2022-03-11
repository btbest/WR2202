#![allow(unused)]
mod animate;
mod components;
mod systems;

use bevy::prelude::*;
use animate::AnimatePlugin;
use components::rocket::RocketL;
use components::rocket::RocketPlugin;
use components::rocket::RocketR;
use systems::movement_systems::*;
use systems::rocket_spawn::*;
use components::players::*;
use crate::components::rocket::*;

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
        // Run two systems every frame
        .add_system(move_system_l)
        .add_system(move_system_r)
        .add_system(spawn_rocket_l)
        .add_system(spawn_rocket_r)
        .add_plugin(RocketPlugin)
        // This is an example of how to structure your game in multiple files.
        // We moved a system into a custom plugin.
        .add_plugin(AnimatePlugin)
        // Let's goooooo
        .run();
}

fn start_up(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Spawn a 2D camera
    // Comment changed by Bene
    // `spawn_bundle` spawns an entity and then adds a bunch of Components (the bundle) to it
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let rocket_sprite_sheet_l = assets.load("RocketL.png");
    let rocket_sprite_sheet_r = assets.load("RocketR.png");
    // let rocket_sprite_sheet_explosion = assets.load("RocketExplosion.png");

    let rocket_texture_atlas_l = TextureAtlas::from_grid(rocket_sprite_sheet_l, Vec2::new(8.0, 8.0), 2, 1);
    let rocket_texture_atlas_r = TextureAtlas::from_grid(rocket_sprite_sheet_r, Vec2::new(8.0, 8.0), 2, 1);
    // let rocket_texture_atlas_explosion = TextureAtlas::from_grid(rocket_sprite_sheet_explosion, Vec2::new(16.0, 16.0), 5, 1);

    let rocket_atlas_handle_l = texture_atlases.add(rocket_texture_atlas_l);
    let rocket_atlas_handle_r = texture_atlases.add(rocket_texture_atlas_r);
    // let rocket_atlas_handle_explosion = texture_atlases.add(rocket_texture_atlas_explosion);

    // Render a rocket from a png file
    commands.spawn_bundle(SpriteSheetBundle {
        // Position of the tree
        // In 2D: x -> right, y -> up, z -> layer towards camera
        transform: Transform::from_scale(Vec3::new(5., 5., 5.)),
        texture_atlas: rocket_atlas_handle_l,
        ..SpriteSheetBundle::default()
    }).insert(RocketL {velocity:0.0});
    // Render a rocket from a png file
    commands.spawn_bundle(SpriteSheetBundle {
        // Position of the tree
        // In 2D: x -> right, y -> up, z -> layer towards camera
        transform: Transform::from_scale(Vec3::new(5., 5., 5.)),
        texture_atlas: rocket_atlas_handle_r,
        ..SpriteSheetBundle::default()
    }).insert(RocketR {velocity:0.0});

    // Load the sprite sheet as an image
    let space_ship_sprite_sheet_l = assets.load("SpaceShipL.png");
    let space_ship_sprite_sheet_r = assets.load("SpaceShipR.png");
    // Split it into a texture atlas be defining the grid dimensions
    let space_ship_texture_atlas_l = TextureAtlas::from_grid(space_ship_sprite_sheet_l, Vec2::new(32.0, 32.0), 2, 1);
    let space_ship_texture_atlas_r = TextureAtlas::from_grid(space_ship_sprite_sheet_r, Vec2::new(32.0, 32.0), 2, 1);

    // Add the new texture atlas to the asset's resource to get a Handle to it
    let space_ship_atlas_handle_l = texture_atlases.add(space_ship_texture_atlas_l);
    let space_ship_atlas_handle_r = texture_atlases.add(space_ship_texture_atlas_r);

    // Spawn player L
    commands
        .spawn_bundle(SpriteSheetBundle {
            // Draw the player above the tree
            transform: Transform::from_scale(Vec3::new(5., 5., 5.)),
            texture_atlas: space_ship_atlas_handle_l,
            ..SpriteSheetBundle::default()
        })
        // add a "Marker" component to our player
        .insert(PlayerL);

    // Spawn player R
    commands
        .spawn_bundle(SpriteSheetBundle {
            // Draw the player above the tree
            transform: Transform::from_scale(Vec3::new(5., 5., 5.)),
            texture_atlas: space_ship_atlas_handle_r,
            ..SpriteSheetBundle::default()
        })
        // add a "Marker" component to our player
        .insert(PlayerR);
}

struct AnimationTimer(Timer);

// This is used to build the initial value of our local timer resource in `animation_system`
impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, true))
    }
}
