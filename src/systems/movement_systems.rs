use bevy::prelude::*;



pub fn move_system_l(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<PlayerL>>) {
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
