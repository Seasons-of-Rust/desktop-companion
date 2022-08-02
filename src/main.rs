use bevy::{prelude::*, window::WindowDescriptor,render::texture::ImageSettings};
use bevy::window::WindowId;
use bevy::winit::WinitWindows;

fn main() {
    setup_bevy();
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


fn setup_bevy(){
    App::new()
    // ClearColor must have 0 alpha, otherwise some color will bleed through
    .insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
    .insert_resource(ClearColor(Color::NONE))
    .insert_resource(WindowDescriptor {
        transparent: true,
        decorations: false,
        width: 200.0,
        height: 200.0,
        resizable: false,
        ..default()
    })
    .add_startup_system(setup)
    .add_system(setup_winit)
    .add_system(animate_sprite)
    .add_plugins(DefaultPlugins)
    .run();
}

fn setup_winit(windows: NonSend<WinitWindows>,) {
    //bevy doesn't have a way to set always_on_top, so use winit 
    let primary = windows.get_window(WindowId::primary()).unwrap();
    primary.set_always_on_top(true);
}

enum Status {
    Walking,Idle
}

/*fn match_anim(status: Status){
    match status{
        Walking => {let texture_handle = asset_server.load("walking_spritesheet.png");
                    texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 4, 4);}
        Idle => {let texture_handle = asset_server.load("witchKitty_aIdles_spritesheet");texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 5);}
    }
    
}*/

fn setup(mut commands: Commands, asset_server: Res<AssetServer>,mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
    let texture_handle = asset_server.load("witchKitty_aIdles_spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.6, true)));
}