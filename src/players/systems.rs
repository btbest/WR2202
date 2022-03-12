use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::players::components::*;
use crate::rockets::components::*;
use crate::players::resources::*;


// This system ticks a timer on every frame. If the timer completed, it will change the sprite index
//   of our player's `SpriteSheetBundle`
//
// `Local` is a local resource scoped to this system (see https://bevy-cheatbook.github.io/programming/local)
// `With` is a filter for queries (see https://bevy-cheatbook.github.io/programming/queries.html#query-filters)
pub fn animation_system_l(
    mut timer: Local<AnimationTimer>,
    time: Res<Time>,
    mut query: Query<&mut TextureAtlasSprite, With<PlayerL>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.single_mut().index = (query.single_mut().index + 1) % 2;
    };
}

pub fn animation_system_r(
    mut timer: Local<AnimationTimer>,
    time: Res<Time>,
    mut query: Query<&mut TextureAtlasSprite, With<PlayerR>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.single_mut().index = (query.single_mut().index + 1) % 2;
    };
}


pub fn start_up_player_system(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load the sprite sheet as an image
    let space_ship_sprite_sheet_l = assets.load("textures/SpaceShipL.png");
    let space_ship_sprite_sheet_r = assets.load("textures/SpaceShipR.png");
    // Split it into a texture atlas be defining the grid dimensions
    let space_ship_texture_atlas_l = TextureAtlas::from_grid(space_ship_sprite_sheet_l, Vec2::new(32.0, 32.0), 2, 1);
    let space_ship_texture_atlas_r = TextureAtlas::from_grid(space_ship_sprite_sheet_r, Vec2::new(32.0, 32.0), 2, 1);
    // Add the new texture atlas to the asset's resource to get a Handle to it
    let space_ship_atlas_handle_l = texture_atlases.add(space_ship_texture_atlas_l);
    let space_ship_atlas_handle_r = texture_atlases.add(space_ship_texture_atlas_r);

    let hit_sound = assets.load("audio/hit.ogg");
    let player_audio = PlayerAudio {
        hit_sound
    };
    commands.insert_resource(player_audio);

    // Spawn player L
    commands
        .spawn_bundle(SpriteSheetBundle {
            // Draw the player in the center
            transform: Transform::from_scale(Vec3::new(5., 5., 5.)),
            texture_atlas: space_ship_atlas_handle_l,
            ..SpriteSheetBundle::default()
        })
        // add a "Marker" component to our player
        .insert(PlayerL);

    // Spawn player R
    commands
        .spawn_bundle(SpriteSheetBundle {
            // Draw the player above the center
            transform: Transform::from_scale(Vec3::new(5., 5., 5.)),
            texture_atlas: space_ship_atlas_handle_r,
            ..SpriteSheetBundle::default()
        })
        // add a "Marker" component to our player
        .insert(PlayerR);
}


pub fn move_system_l(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<PlayerL>>, windows: Res<Windows>) {
    let speed = 10.;
    let window = windows.get_primary().unwrap();
    let bounds = Bounds {
        x0: -window.width()/2. + 25.,
        x1: -window.width()/6. - 25.,
        y0: -window.height()/2. + 25.,
        y1: window.height()/2. - 25.,
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


struct Bounds {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32
}

pub fn move_system_r(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<PlayerR>>, windows: Res<Windows>) {
    let speed = 10.;
    let window = windows.get_primary().unwrap();
    let bounds = Bounds {
        x0: window.width()/6. + 25.,
        x1: window.width()/2. - 25.,
        y0: -window.height()/2. + 25.,
        y1: window.height()/2. - 25.,
    };
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


pub struct AnimationTimer(Timer);

// This is used to build the initial value of our local timer resource in `animation_system`
impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, true))
    }
}


pub fn collision_detection_system(
    mut timer: Local<CollisionTimer>,
    time: Res<Time>,
    mut commands: Commands,
    mut q_playel: Query<(
        Entity, 
        &Transform, 
        &mut PlayerL
    )>,
    mut q_prayer: Query<(
        Entity, 
        &Transform,
        &mut PlayerR
    )>,
    mut q_locket: Query<(
        Entity, 
        &Transform,
        &mut RocketL
    )>,
    mut q_rocket: Query<(
        Entity, 
        &Transform,
        &mut RocketR
    )>,
    player_audio: Res<PlayerAudio>,
    audio: Res<Audio>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let playel = q_playel.single_mut();
        let prayer = q_prayer.single_mut();
        q_locket.for_each_mut(|(
            r_entity, 
            r_transform,
            mut locket
        )|{
            // println!("testing locket collision");
            let p_transform = prayer.1;
            if collide(
                p_transform.translation, 
                Vec2::new(p_transform.scale.x, p_transform.scale.y), 
                r_transform.translation,
                Vec2::new(r_transform.scale.x, r_transform.scale.y),
            ).is_some(){
                println!("locket despawned, point for player L");
                audio.play(player_audio.hit_sound.clone());
                // TODO: Increase pointer for player L
                commands.entity(r_entity).despawn();
            }
        });
        q_rocket.for_each_mut(|(
            r_entity, 
            r_transform,
            mut rocket
        )|{
            let p_transform = playel.1;
            if collide(
                p_transform.translation, 
                Vec2::new(p_transform.scale.x, p_transform.scale.y), 
                r_transform.translation,
                Vec2::new(r_transform.scale.x, r_transform.scale.y),
            ).is_some(){
                println!("rocket despawned, point for player R");
                audio.play(player_audio.hit_sound.clone());
                // TODO: Increase pointer for player R
                commands.entity(r_entity).despawn();
            }
        });
    };
}

pub struct CollisionTimer(Timer);

// This is used to build the initial value of our local timer resource in `animation_system`
impl Default for CollisionTimer {
    fn default() -> Self {
        CollisionTimer(Timer::from_seconds(0.001, true))
    }
}