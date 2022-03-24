use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::interaction::components::*;
use crate::players::components::*;
use crate::rockets::components::*;
use crate::explosions::components::*;
use crate::explosions::resources::*;

pub fn collision_detection_system(
    mut commands: Commands,
    mut query_players: Query<(&Transform, &mut Player, &Team)>,
    mut query_rockets: Query<(Entity, &Transform, &Team), With<Rocket>>,
    explosion_audio: Res<ExplosionAudio>,
    audio: Res<Audio>,
    textures: Res<ExplosionTextures>,
) {
    for (transform_p, mut player, team_p) in query_players.iter_mut() {
        for (entity_r, transform_r, team_r) in query_rockets.iter_mut() {
            if team_r.side == team_p.side { 
                break // No friendly fire :-)
            } else {
                println!("positions: player: {}, rocket: {}", transform_p.translation, transform_r.translation);
                if collide(
                    transform_p.translation, 
                    Vec2::new(16.*transform_p.scale.x, 32.*transform_p.scale.y), 
                    transform_r.translation,
                    Vec2::new(8.*transform_r.scale.x, 4.*transform_r.scale.y),
                ).is_some(){
                    // Hit player and despawn rocket:
                    println!("Player {} was hit!", team_p.side);
                    player.hp -= 1;
                    audio.play(explosion_audio.explosion_sound.clone());
                    commands.entity(entity_r).despawn();
                    // Spawn explosion:
                    let transform = Transform {
                        translation: Vec3::new(transform_r.translation.x, transform_r.translation.y, 0.),
                        scale: Vec3::new(5., 5., 5.),
                        ..Default::default()
                    };
                    commands.spawn_bundle(SpriteSheetBundle {
                        transform,
                        texture_atlas: textures.explosion_texture.clone(),
                        ..SpriteSheetBundle::default()
                    }).insert(Explosion::default());
                }
            }
        }
    }
}
