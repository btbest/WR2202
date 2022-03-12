use bevy::prelude::*;
use crate::rockets::components::*;
use crate::players::components::*;
use crate::rockets::resources::*;


pub fn spawn_rocket_l(
    mut commands: Commands,
    query: Query<&mut Transform, With<PlayerL>>,
    input: Res<Input<KeyCode>>,
    mut timer: Local<CooldownTimer>,
    time: Res<Time>,
    textures: Res<RocketTextures>,
    rocket_audio: Res<RocketAudio>,
    audio: Res<Audio>,
) {
    timer.0.tick(time.delta());
    if input.just_pressed(KeyCode::LControl) && timer.0.finished() {
        timer.0.reset();
        println!("L ROCKET!!!");
        audio.play(rocket_audio.rocket_sound.clone());
        // Render a rocket from a png file
        let translation = query.single().translation;
        let transform = Transform {
            translation: Vec3::new(translation.x, translation.y, 0.),
            scale: Vec3::new(5., 5., 5.),
            ..Default::default()
        };
        commands.spawn_bundle(SpriteSheetBundle {
            // Position of the rocket
            // In 2D: x -> right, y -> up, z -> layer towards camera
            transform,
            texture_atlas: textures.rocket_texture_l.clone(),
            ..SpriteSheetBundle::default()
        }).insert(RocketL::default());
    }
}

pub fn spawn_rocket_r(
    mut commands: Commands,
    query: Query<&mut Transform, With<PlayerR>>,
    time: Res<Time>,
    mut timer: Local<CooldownTimer>,
    input: Res<Input<KeyCode>>,
    textures: Res<RocketTextures>,
    rocket_audio: Res<RocketAudio>,
    audio: Res<Audio>,
) {
    timer.0.tick(time.delta());
    if input.just_pressed(KeyCode::RControl) && timer.0.finished() {
        timer.0.reset();
        println!("R ROCKET!!!");
        audio.play(rocket_audio.rocket_sound.clone());
        // Render a rocket from a png file
        let translation = query.single().translation;
        let transform = Transform {
            translation: Vec3::new(translation.x, translation.y, 0.),
            scale: Vec3::new(5., 5., 5.),
            ..Default::default()
        };
        commands.spawn_bundle(SpriteSheetBundle {
            // Position of the rocket
            // In 2D: x -> right, y -> up, z -> layer towards camera
            transform,
            texture_atlas: textures.rocket_texture_r.clone(),
            ..SpriteSheetBundle::default()
        }).insert(RocketR::default());
    }
}


// This system ticks a timer on every frame. If the timer completed, it will change the sprite index
// of the rockets `SpriteSheetBundle`
// `Local` is a local resource scoped to this system (see https://bevy-cheatbook.github.io/programming/local)
// `With` is a filter for queries (see https://bevy-cheatbook.github.io/programming/queries.html#query-filters)
pub fn rocket_movement_system_l(
    mut query: Query<(&mut Transform, &mut RocketL)>,
    mut timer: Local<RocketPhysicsTimer>,
    time: Res<Time>,
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

pub fn rocket_movement_system_r(
    mut query: Query<(&mut Transform, &mut RocketR)>,
    mut timer: Local<RocketPhysicsTimer>,
    time: Res<Time>,
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


pub fn rocket_animation_system_l(
    mut timer: Local<AnimationTimer>,
    time: Res<Time>,
    mut query: Query<&mut TextureAtlasSprite, With<RocketL>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.for_each_mut(|mut atlas| {atlas.index = (atlas.index + 1) % 2});
    };
}

pub fn rocket_animation_system_r(
    mut timer: Local<AnimationTimer>,
    time: Res<Time>,
    mut query: Query<&mut TextureAtlasSprite, With<RocketR>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.for_each_mut(|mut atlas| {atlas.index = (atlas.index + 1) % 2});
    };
}


pub fn rocket_deletion_system(
    mut timer: Local<RocketPhysicsTimer>,
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, Option<&RocketL>, Option<&RocketR>)>, 
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.for_each_mut(|(entity, transform, locket, rocket)|{
            if matches!(locket, Some(locket)) && transform.translation.x > window.width()/2. + 25.0 {
                commands.entity(entity).despawn();
            }
            if matches!(rocket, Some(rocket)) && transform.translation.x < -window.width()/2. - 25.0 {
                commands.entity(entity).despawn();
            }
        })
    };
}


pub fn start_up_rockets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let rocket_sprite_sheet_l = assets.load("textures/RocketL.png");
    let rocket_texture_atlas_l = TextureAtlas::from_grid(rocket_sprite_sheet_l, Vec2::new(8.0, 8.0), 2, 1);
    let rocket_atlas_handle_l = texture_atlases.add(rocket_texture_atlas_l);

    let rocket_sprite_sheet_r = assets.load("textures/RocketR.png");
    let rocket_texture_atlas_r = TextureAtlas::from_grid(rocket_sprite_sheet_r, Vec2::new(8.0, 8.0), 2, 1);
    let rocket_atlas_handle_r = texture_atlases.add(rocket_texture_atlas_r);

    let rocket_textures = RocketTextures { 
        rocket_texture_l: rocket_atlas_handle_l, 
        rocket_texture_r: rocket_atlas_handle_r};
    commands.insert_resource(rocket_textures);

    let rocket_sound = assets.load("audio/pew.ogg");
    let rocket_audio = RocketAudio {
        rocket_sound
    };
    commands.insert_resource(rocket_audio)
}


pub struct AnimationTimer(Timer);

// This is used to build the initial value of our local timer resource in `animation_system`
impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, true))
    }
}

pub struct CooldownTimer(Timer);

impl Default for CooldownTimer {
    fn default() -> Self {
        CooldownTimer(Timer::from_seconds(0.5, false))
    }
}

pub struct RocketPhysicsTimer(Timer);

// This is used to build the initial value of our local timer resource in `animation_system`
impl Default for RocketPhysicsTimer {
    fn default() -> Self {
        RocketPhysicsTimer(Timer::from_seconds(0.05, true))
    }
}
