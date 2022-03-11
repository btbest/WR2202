use bevy::prelude::*;
struct RocketPhysicsTimer(Timer);

// This is used to build the initial value of our local timer resource in `animation_system`
impl Default for RocketPhysicsTimer {
    fn default() -> Self {
        RocketPhysicsTimer(Timer::from_seconds(0.05, true))
    }
}


pub struct RocketPlugin;

impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(rocket_movement_system_l);
        app.add_system(rocket_movement_system_r);
    }
}

// This system ticks a timer on every frame. If the timer completed, it will change the sprite index
//   of our player's `SpriteSheetBundle`
//
// `Local` is a local resource scoped to this system (see https://bevy-cheatbook.github.io/programming/local)
// `With` is a filter for queries (see https://bevy-cheatbook.github.io/programming/queries.html#query-filters)
fn rocket_movement_system_l(
    mut timer: Local<RocketPhysicsTimer>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut RocketL)>,
) {
    let acceleration = 4.;
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.for_each_mut(|(mut transform, mut rocket)|{
            transform.translation.x += rocket.velocity;
            rocket.velocity += acceleration;
        });
    };
}
fn rocket_movement_system_r(
    mut timer: Local<RocketPhysicsTimer>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut RocketR)>,
) {
    let acceleration = 4.;
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.for_each_mut(|(mut transform, mut rocket)|{
            transform.translation.x -= rocket.velocity;
            rocket.velocity += acceleration;
        });
    };
}

#[derive(Component)]
pub struct RocketL {
    pub velocity: f32
}

#[derive(Component)]
pub struct RocketR{
    pub velocity: f32
}

pub struct RocketTextures {
    pub rocket_texture_l: Handle<TextureAtlas>,
    pub rocket_texture_r: Handle<TextureAtlas>
}