use bevy::prelude::*;
use crate::rockets::components::*;
use crate::rockets::resources::*;
use crate::players::components::*;
use crate::interaction::components::*;


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


pub struct CooldownTimer(Timer);

impl Default for CooldownTimer {
    fn default() -> Self {
        CooldownTimer(Timer::from_seconds(0.5, false))
    }
}

pub fn spawn_rocket(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Player, &Team)>,
    input: Res<Input<KeyCode>>,
    mut timer: Local<CooldownTimer>,
    time: Res<Time>,
    textures: Res<RocketTextures>,
    rocket_audio: Res<RocketAudio>,
    audio: Res<Audio>,
) {
    timer.0.tick(time.delta());
    for (transform, player, team) in query.iter_mut() {
        if input.just_pressed(player.keys.fire) {
            timer.0.reset();
            println!("{} ROCKET!!!", team.side);
            audio.play(rocket_audio.rocket_sound.clone());
            // Render a rocket from a png file
            let transform = Transform {
                translation: Vec3::new(transform.translation.x, transform.translation.y, 0.),
                scale: Vec3::new(5., 5., 5.),
                ..Default::default()
            };
            let texture_atlas = match team.side {
                'L' => textures.rocket_texture_l.clone(),
                _ => textures.rocket_texture_r.clone(),
                };
            commands.spawn_bundle(SpriteSheetBundle {
                // Position of the rocket
                // In 2D: x -> right, y -> up, z -> layer towards camera
                transform,
                texture_atlas,
                ..SpriteSheetBundle::default()
            }).insert(Rocket::default())
            .insert(Team {
                side: team.side,
            });
        }
    }
}


pub struct RocketPhysicsTimer(Timer);

// This is used to build the initial value of our local timer resource in `animation_system`
impl Default for RocketPhysicsTimer {
    fn default() -> Self {
        RocketPhysicsTimer(Timer::from_seconds(0.05, true))
    }
}

// This system ticks a timer on every frame. If the timer completed, it will change the sprite index
// of the rockets `SpriteSheetBundle`
// `Local` is a local resource scoped to this system (see https://bevy-cheatbook.github.io/programming/local)
// `With` is a filter for queries (see https://bevy-cheatbook.github.io/programming/queries.html#query-filters)
pub fn rocket_movement_system(
    mut query: Query<(&mut Transform, &mut Rocket, &Team)>,
    mut timer: Local<RocketPhysicsTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.for_each_mut(|(mut transform, mut rocket, team)|{
            let acceleration = match team.side {
                'L' => 4.,
                'R' => -4.,
                _ => 0.,
            };
            transform.translation.x += rocket.velocity;
            rocket.velocity += acceleration;
        });
    };
}


pub struct AnimationTimer(Timer);

// This is used to build the initial value of our local timer resource in `animation_system`
impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, true))
    }
}

pub fn rocket_animation_system(
    mut timer: Local<AnimationTimer>,
    time: Res<Time>,
    mut query: Query<&mut TextureAtlasSprite, With<Rocket>>,
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
    mut query: Query<(Entity, &Transform), With<Rocket>>, 
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.for_each_mut(|(entity, transform)|{
            let x = transform.translation.x;
            if x > window.width()/2. + 25.0 || x < -window.width()/2. - 25.0 {
                commands.entity(entity).despawn();
            }
        })
    };
}
