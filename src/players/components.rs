use bevy::prelude::*;


#[derive(Component)]
pub struct Player {
    pub hp: u8,
    pub keys: Keys,
    pub bounds: Bounds,
    pub on_cooldown: bool
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
