use bevy::prelude::*;
use crate::explosions::components::*;
use crate::explosions::resources::*;


pub fn start_up_explosions(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Texture:
    let explosion_sprite_sheet = assets.load("textures/RocketExplosion.png");
    let explosion_texture_atlas = TextureAtlas::from_grid(explosion_sprite_sheet, Vec2::new(16.0, 16.0), 5, 1);
    let explosion_atlas_handle = texture_atlases.add(explosion_texture_atlas);
    let explosion_textures = ExplosionTextures {
        explosion_texture: explosion_atlas_handle,
    };
    commands.insert_resource(explosion_textures);
    // Sound:
    let explosion_sound = assets.load("audio/hit.ogg");
    let player_audio = ExplosionAudio {
        explosion_sound
    };
    commands.insert_resource(player_audio);
}


pub fn explosion_animation_and_despawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Explosion, &mut TextureAtlasSprite)>,
) {
    time.delta();
    query.for_each_mut(|(entity, mut explosion, mut atlas)| {
        explosion.lifetime = explosion.lifetime + time.delta();
        let max_lifetime = 500;  // in milliseconds
        let explosion_millis = explosion.lifetime.as_millis();
        println!("{:?}: explosion_millis: {}", entity, explosion_millis);
        if explosion_millis >= max_lifetime {
            println!("Despawn explosion!");
            commands.entity(entity).despawn();
        } else {
            atlas.index = ((explosion_millis as f32 / max_lifetime as f32) * 5.) as usize;
            // atlas.index = (atlas.index + 1) % 5;
            println!("explosion atlas.index: {}", atlas.index)
        }
    })
}
