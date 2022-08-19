use bevy::prelude::Component;

//region: --- Common components
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
// endregion: --- Common components

//region: --- Cat components
#[derive(Component)]
pub struct Cat;
//endregion: --- Cat components