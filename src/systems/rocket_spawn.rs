#![allow(unused)]
use bevy::prelude::*;
use crate::components::players::*;
use crate::components::rocket::*;

pub fn spawn_rocket_l(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    input: Res<Input<KeyCode>>,
    textures: Res<RocketTextures>,
    rocket_audio: Res<RocketAudio>,
    audio: Res<Audio>,
    mut query: Query<&mut Transform, With<PlayerL>>
) {
    if input.just_pressed(KeyCode::LControl) {
        println!("L ROCKET!!!");
        audio.play(rocket_audio.rocket_sound.clone());
        // Render a rocket from a png file
        let translation = query.single().translation;
        let transform = Transform {
            translation: Vec3::new(translation.x, translation.y, 0.),
            scale: Vec3::new(5., 5., 5.),
            ..Default::default()
        };
        commands.spawn_bundle(SpriteSheetBundle {
            // Position of the tree
            // In 2D: x -> right, y -> up, z -> layer towards camera
            transform,
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
    rocket_audio: Res<RocketAudio>,
    audio: Res<Audio>,
    mut query: Query<&mut Transform, With<PlayerR>>
) {
    if input.just_pressed(KeyCode::RControl) {
        println!("R ROCKET!!!");
        audio.play(rocket_audio.rocket_sound.clone());
        // Render a rocket from a png file
        let translation = query.single().translation;
        let transform = Transform {
            translation: Vec3::new(translation.x, translation.y, 0.),
            scale: Vec3::new(5., 5., 5.),
            ..Default::default()
        };
        commands.spawn_bundle(SpriteSheetBundle {
            // Position of the tree
            // In 2D: x -> right, y -> up, z -> layer towards camera
            transform,
            texture_atlas: textures.rocket_texture_r.clone(),
            ..SpriteSheetBundle::default()
        }).insert(RocketR::default());
    }
}

struct CooldownTimer(Timer);

impl Default for CooldownTimer {
    fn default() -> Self {
        CooldownTimer(Timer::from_seconds(0.5, false))
    }
}