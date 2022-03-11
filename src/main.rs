mod animate;
mod components;
mod systems;

use bevy::prelude::*;
use crate::animate::AnimatePlugin;

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

    // Render a tree from a png file
    commands.spawn_bundle(SpriteBundle {
        // Position of the tree
        // In 2D: x -> right, y -> up, z -> layer towards camera
        transform: Transform::from_translation(Vec3::new(150., 50., 0.)),
        texture: assets.load("tree.png"),
        ..SpriteBundle::default()
    });

    // Load the sprite sheet as an image
    let sprite_sheet_l = assets.load("SpaceShipL.png");
    let sprite_sheet_r = assets.load("SpaceShipR.png");
    // Split it into a texture atlas be defining the grid dimensions
    let texture_atlas_l = TextureAtlas::from_grid(sprite_sheet_l, Vec2::new(8.0, 8.0), 2, 1);
    let texture_atlas_r = TextureAtlas::from_grid(sprite_sheet_r, Vec2::new(8.0, 8.0), 2, 1);

    // Add the new texture atlas to the asset's resource to get a Handle to it
    let atlas_handle_l = texture_atlases.add(texture_atlas_l);
    let atlas_handle_r = texture_atlases.add(texture_atlas_r);

    // Spawn player L
    commands
        .spawn_bundle(SpriteSheetBundle {
            // Draw the player above the tree
            transform: Transform::from_scale( Vec3::new(5., 5., 5.)),
            texture_atlas: atlas_handle_l,
            ..SpriteSheetBundle::default()
        })
        // add a "Marker" component to our player
        .insert(PlayerL);

    // Spawn player R
    commands
        .spawn_bundle(SpriteSheetBundle {
            // Draw the player above the tree
            transform: Transform::from_scale( Vec3::new(5., 5., 5.)),
            texture_atlas: atlas_handle_r,
            ..SpriteSheetBundle::default()
        })
        // add a "Marker" component to our player
        .insert(PlayerR);
}

fn move_system_l(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<PlayerL>>) {
    let speed = 10.;
    if input.pressed(KeyCode::W) {
        query.single_mut().translation.y += speed;
    }
    if input.pressed(KeyCode::A) {
        query.single_mut().translation.x -= speed;
    }
    if input.pressed(KeyCode::S) {
        query.single_mut().translation.y -= speed;
    }
    if input.pressed(KeyCode::D) {
        query.single_mut().translation.x += speed;
    }
}


fn move_system_r(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<PlayerR>>) {
    let speed = 10.;
    if input.pressed(KeyCode::Up) {
        query.single_mut().translation.y += speed;
    }
    if input.pressed(KeyCode::Left) {
        query.single_mut().translation.x -= speed;
    }
    if input.pressed(KeyCode::Down) {
        query.single_mut().translation.y -= speed;
    }
    if input.pressed(KeyCode::Right) {
        query.single_mut().translation.x += speed;
    }
}

#[derive(Component)]
struct PlayerL;

#[derive(Component)]
struct PlayerR;

struct AnimationTimer(Timer);

// This is used to build the initial value of our local timer resource in `animation_system`
impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.2, true))
    }
}
