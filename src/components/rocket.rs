use bevy::prelude::*;
use crate::{AnimationTimer};

pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(rocket_movement_system_l);
    }
}

// This system ticks a timer on every frame. If the timer completed, it will change the sprite index
//   of our player's `SpriteSheetBundle`
//
// `Local` is a local resource scoped to this system (see https://bevy-cheatbook.github.io/programming/local)
// `With` is a filter for queries (see https://bevy-cheatbook.github.io/programming/queries.html#query-filters)
fn rocket_movement_system_l(
    mut timer: Local<AnimationTimer>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<RocketL>>,
) {
    let speed = 10.;
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.single_mut().translation.x += speed;
    };
}
fn rocket_movement_system_r(
    mut timer: Local<AnimationTimer>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<RocketR>>,
) {
}


#[derive(Component)]
pub struct RocketL;

#[derive(Component)]
pub struct RocketR;