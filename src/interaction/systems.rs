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
    mut timer: Local<CollisionTimer>,
    time: Res<Time>,
    mut commands: Commands,
    mut query_players: Query<(&Transform, &mut Player, &Team)>,
    mut query_rockets: Query<(Entity, &Transform, &Team), With<Rocket>>,
    player_audio: Res<PlayerAudio>,
    audio: Res<Audio>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        for (transform_p, mut player, team_p) in query_players.iter_mut() {
            for (entity_r, transform_r, team_r) in query_rockets.iter_mut() {
                if team_r.side == team_p.side { 
                    break // No friendly fire :-)
                } else {
                    if collide(
                        transform_p.translation, 
                        Vec2::new(transform_p.scale.x, transform_p.scale.y), 
                        transform_r.translation,
                        Vec2::new(transform_r.scale.x, transform_r.scale.y),
                    ).is_some(){
                        println!("Point for player {}!", team_p.side);
                        player.points += 1;
                        audio.play(player_audio.hit_sound.clone());
                        commands.entity(entity_r).despawn();
                    }
                }
            }
        }
    }
}
