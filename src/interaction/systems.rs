use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::interaction::components::*;
use crate::players::components::*;
use crate::players::resources::*;
use crate::rockets::components::*;


pub struct CollisionTimer(Timer);

// This is used to build the initial value of our local timer resource in `animation_system`
impl Default for CollisionTimer {
    fn default() -> Self {
        CollisionTimer(Timer::from_seconds(0.001, true))
    }
}

pub fn collision_detection_system(
    mut commands: Commands,
    mut query_players: Query<(&Transform, &mut Player, &Team)>,
    mut query_rockets: Query<(Entity, &Transform, &Team), With<Rocket>>,
    player_audio: Res<PlayerAudio>,
    audio: Res<Audio>,
) {
    for (transform_p, mut player, team_p) in query_players.iter_mut() {
        for (entity_r, transform_r, team_r) in query_rockets.iter_mut() {
            if team_r.side == team_p.side { 
                break // No friendly fire :-)
            } else {
                if collide(
                    transform_p.translation, 
                    Vec2::new(32.*transform_p.scale.x, 32.*transform_p.scale.y), 
                    transform_r.translation,
                    Vec2::new(8.*transform_r.scale.x, 8.*transform_r.scale.y),
                ).is_some(){
                    println!("Player {} was hit!", team_p.side);
                    player.hp -= 1;
                    audio.play(player_audio.hit_sound.clone());
                    commands.entity(entity_r).despawn();
                }
            }
        }
    }
}
