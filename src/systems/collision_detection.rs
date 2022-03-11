use bevy::prelude::*;

use crate::components::{rocket::{RocketL, RocketR}, players::{PlayerL, PlayerR}};
use bevy::sprite::collide_aabb::collide;

struct CollisionTimer(Timer);

// This is used to build the initial value of our local timer resource in `animation_system`
impl Default for CollisionTimer {
    fn default() -> Self {
        CollisionTimer(Timer::from_seconds(0.001, true))
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision_detection_system);
    }
}

fn collision_detection_system(
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
    )>
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
                println!("locket despawned");
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
                println!("rocket despawned");
                commands.entity(r_entity).despawn();
            }
        });
    };
}