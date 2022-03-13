use bevy::prelude::*;


#[derive(Component)]
pub struct Player {
    pub points: u8,
    pub keys: Keys,
    pub bounds: Bounds,
    // pub cooldown: f32
}

pub struct Keys {
    pub up: KeyCode,
    pub left: KeyCode,
    pub down: KeyCode,
    pub right: KeyCode,
    pub fire: KeyCode
}

pub struct Bounds {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32
}