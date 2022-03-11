use bevy::prelude::*;
use crate::{AnimationTimer, App, Local, PlayerL, PlayerR, Query, Res, TextureAtlasSprite, Time, With};

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animation_system_l);
        app.add_system(animation_system_r);
    }
}

// This system ticks a timer on every frame. If the timer completed, it will change the sprite index
//   of our player's `SpriteSheetBundle`
//
// `Local` is a local resource scoped to this system (see https://bevy-cheatbook.github.io/programming/local)
// `With` is a filter for queries (see https://bevy-cheatbook.github.io/programming/queries.html#query-filters)
fn animation_system_l(
    mut timer: Local<AnimationTimer>,
    time: Res<Time>,
    mut query: Query<&mut TextureAtlasSprite, With<PlayerL>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.single_mut().index = (query.single_mut().index + 1) % 1;
    };
}

fn animation_system_r(
    mut timer: Local<AnimationTimer>,
    time: Res<Time>,
    mut query: Query<&mut TextureAtlasSprite, With<PlayerR>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.single_mut().index = (query.single_mut().index + 1) % 1;
    };
}
