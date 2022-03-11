use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerL;

#[derive(Component)]
pub struct PlayerR;

pub struct PlayerAudio {
    pub hit_sound: Handle<AudioSource>
}