use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Explosion{
    pub lifetime: Duration
}
