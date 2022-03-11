#![allow(unused)]
use bevy::prelude::*;
use crate::components::players::*;
use crate::components::rocket::*;

pub fn spawn_rocket_l(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    input: Res<Input<KeyCode>>,
    textures: Res<RocketTextures>,
    mut query: Query<&mut Transform, With<PlayerL>>
) {
    if input.just_pressed(KeyCode::LControl) {
        println!("L ROCKET!!!");
        // Render a rocket from a png file
        commands.spawn_bundle(SpriteSheetBundle {
            // Position of the tree
            // In 2D: x -> right, y -> up, z -> layer towards camera
            transform: Transform::from_scale(Vec3::new(5., 5., 5.)),
            texture_atlas: textures.rocket_texture_l.clone(),
            ..SpriteSheetBundle::default()
        }).insert(RocketL::default());
    }
}

pub fn spawn_rocket_r(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    input: Res<Input<KeyCode>>,
    textures: Res<RocketTextures>,
    mut query: Query<&mut Transform, With<PlayerR>>
) {
    if input.just_pressed(KeyCode::RControl) {
        println!("R ROCKET!!!");
        // Render a rocket from a png file
        commands.spawn_bundle(SpriteSheetBundle {
            // Position of the tree
            // In 2D: x -> right, y -> up, z -> layer towards camera
            transform: Transform::from_scale(Vec3::new(5., 5., 5.)),
            texture_atlas: textures.rocket_texture_r.clone(),
            ..SpriteSheetBundle::default()
        }).insert(RocketR::default());
    }
}