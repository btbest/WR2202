use bevy::prelude::*;

struct Bounds {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32
}

pub fn move_system_l(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<PlayerL>>, windows: Res<Windows>) {
    let speed = 10.;
    let window = windows.get_primary().unwrap();
    let height = window.height();
    let width = window.width();  
    let bounds = Bounds {
        x0: -window.width()/2.,
        x1: -window.width()/6.,
        y0: -window.height()/2.,
        y1: window.height()/2.,
    };
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
    if query.single_mut().translation.x < bounds.x0 {
        query.single_mut().translation.x = bounds.x0;
    }
    else if query.single_mut().translation.x > bounds.x1 {
        query.single_mut().translation.x = bounds.x1;
    }
    if query.single_mut().translation.y < bounds.y0 {
        query.single_mut().translation.y = bounds.y0;
    }
    else if query.single_mut().translation.y > bounds.y1 {
        query.single_mut().translation.y = bounds.y1;
    }

}


pub fn move_system_r(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<PlayerR>>) {
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
pub struct PlayerL;

#[derive(Component)]
pub struct PlayerR;
