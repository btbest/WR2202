#![allow(unused)]
use bevy::prelude::*;
use crate::components::players::*;

pub fn spawn_rocket_l(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<PlayerL>>) {
    if input.just_pressed(KeyCode::LControl) {
        println!("L ROCKET!!!")
    }
}

pub fn spawn_rocket_r(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<PlayerR>>) {
    if input.just_pressed(KeyCode::RControl) {
        println!("R ROCKET!!!")
    }
}