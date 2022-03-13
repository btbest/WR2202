use bevy::prelude::*;
use crate::players::components::*;
use crate::players::resources::*;
use crate::interaction::components::*;


pub fn start_up_player_system(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    windows: Res<Windows>
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
    // Sounds:
    let hit_sound = assets.load("audio/hit.ogg");
    let player_audio = PlayerAudio {
        hit_sound
    };
    commands.insert_resource(player_audio);
    // Spawn players:
    let window = windows.get_primary().unwrap();
    commands
        .spawn_bundle(SpriteSheetBundle {
            // Draw the player in the center
            transform: Transform::from_scale(Vec3::new(5., 5., 5.)),
            texture_atlas: space_ship_atlas_handle_l,
            ..SpriteSheetBundle::default()
        })
        // add a "Marker" component to our player
        .insert(Player {
            points: 0,
            keys: Keys {
                up: KeyCode::W, 
                left: KeyCode::A, 
                down: KeyCode::S, 
                right: KeyCode::D, 
                fire: KeyCode::LControl
            },
            bounds: Bounds {
                x0: -window.width()/2. + 25.,
                x1: -window.width()/6. - 25.,
                y0: -window.height()/2. + 25.,
                y1: window.height()/2. - 25.,
            }
        }).insert(Team {
            side: 'L',
        });
    // Spawn player R
    commands
        .spawn_bundle(SpriteSheetBundle {
            // Draw the player above the center
            transform: Transform::from_scale(Vec3::new(5., 5., 5.)),
            texture_atlas: space_ship_atlas_handle_r,
            ..SpriteSheetBundle::default()
        })
        // add a "Marker" component to our player
        .insert(Player {
            points: 0,
            keys: Keys {
                up: KeyCode::Up, 
                left: KeyCode::Left, 
                down: KeyCode::Down, 
                right: KeyCode::Right, 
                fire: KeyCode::RControl
            },
            bounds: Bounds {
                x0: window.width()/6. + 25.,
                x1: window.width()/2. - 25.,
                y0: -window.height()/2. + 25.,
                y1: window.height()/2. - 25.,
            }
        }).insert(Team {
            side: 'R',
        });
}


pub struct AnimationTimer(Timer);

// This is used to build the initial value of our local timer resource in `animation_system`
impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer(Timer::from_seconds(0.1, true))
    }
}

// This system ticks a timer on every frame. If the timer completed, it will change the sprite index
//   of our player's `SpriteSheetBundle`
//
// `Local` is a local resource scoped to this system (see https://bevy-cheatbook.github.io/programming/local)
// `With` is a filter for queries (see https://bevy-cheatbook.github.io/programming/queries.html#query-filters)
pub fn animation_system(
    mut timer: Local<AnimationTimer>,
    time: Res<Time>,
    mut query: Query<&mut TextureAtlasSprite, With<Player>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        query.for_each_mut(|mut atlas| {atlas.index = (atlas.index + 1) % 2});
    };
}


pub fn move_system(input: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &Player)>) {
    let speed = 10.;
    for (mut transform, player) in query.iter_mut() {
        if input.pressed(player.keys.up) {
            transform.translation.y += speed;
        }
        if input.pressed(player.keys.left) {
            transform.translation.x -= speed;
        }
        if input.pressed(player.keys.down) {
            transform.translation.y -= speed;
        }
        if input.pressed(player.keys.right) {
            transform.translation.x += speed;
        }
        if transform.translation.x < player.bounds.x0 {
            transform.translation.x = player.bounds.x0;
        }
        else if transform.translation.x > player.bounds.x1 {
            transform.translation.x = player.bounds.x1;
        }
        if transform.translation.y < player.bounds.y0 {
            transform.translation.y = player.bounds.y0;
        }
        else if transform.translation.y > player.bounds.y1 {
            transform.translation.y = player.bounds.y1;
        }
    }
}
