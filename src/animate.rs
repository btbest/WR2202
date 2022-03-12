use bevy::prelude::*;
use crate::{AnimationTimer, App, Local, Query, Res, TextureAtlasSprite, Time, With};
use crate::components::players::*;
use crate::components::rocket::*;

pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animation_system_l);
        app.add_system(animation_system_r);
        app.add_system(rocket_animation_system_l);
        app.add_system(rocket_animation_system_r);
    }
}


