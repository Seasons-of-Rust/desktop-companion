
use bevy::render::view::window;
use bevy::{prelude::*, window::WindowDescriptor,render::texture::ImageSettings,asset::LoadState};
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use cat::CatPlugin;

mod cat;
mod components;

const CAT_SPRITES: &str = "catSprites.png";


//region: --- Game Constants
const TIME_STEP:f32 = 1./60.;
const BASE_SPEED:f32 = 500.;
//endregion: --- Game Constants

struct GameTexture {
    cat: Handle<Image>,
}

fn main() {
    setup_bevy();
}

fn setup_bevy(){
    App::new()
    .insert_resource(ImageSettings::default_nearest())
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
    .add_startup_system_to_stage(StartupStage::PostStartup,setup_winit)
    .add_plugin(CatPlugin)
    .add_plugins(DefaultPlugins)
    .run();
}

fn setup_winit(windows: NonSend<WinitWindows>,) {
    //bevy doesn't have a way to set always_on_top, so use winit 
    let primary = windows.get_window(WindowId::primary()).unwrap();
    primary.set_always_on_top(true);
}
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn_bundle(Camera2dBundle::default());

    //add game texture resource
    let game_texture = GameTexture {
        cat: asset_server.load(CAT_SPRITES),
    };
    commands.insert_resource(game_texture);
}


