use crate::{GameTexture, components::{Velocity, Cat}, BASE_SPEED, TIME_STEP};
use bevy::prelude::*;


pub struct CatPlugin;

impl Plugin for CatPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_cat)
        .add_system(cat_movement_system)
        .add_system(animate_sprite);
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
//https://bevyengine.org/examples/2d/sprite-sheet/
fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}


fn spawn_cat(
    mut commands: Commands,
    game_texture: Res<GameTexture>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let texture_handle = game_texture.cat.clone();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 42, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        })
        .insert(Cat)
        .insert(Velocity{x:0.,y:0.})
        .insert(AnimationTimer(Timer::from_seconds(0.6, true)));
}


fn cat_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Cat>>) {
    for (velocity, mut transform) in query.iter_mut(){
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP*BASE_SPEED;
        translation.y += velocity.y * TIME_STEP*BASE_SPEED;
    }
}


fn cat_state_system(){
    todo!();
}