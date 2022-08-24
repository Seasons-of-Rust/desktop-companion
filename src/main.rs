use bevy::{prelude::*, window::WindowDescriptor,render::texture::ImageSettings,asset::LoadState};
use bevy::window::WindowId;
use bevy::winit::WinitWindows;


use cat::CatPlugin;
use crate::scraper::ScraperPlugin;

mod cat;
mod components;
mod scraper;



const CAT_SPRITES: &str = "catSprites.png";

struct CatState{
    on: bool,
    last_shot : f64,
}

impl CatState{
    pub fn sleepy(&mut self, time: f64){
        self.on = false;
        self.last_shot = time;
    }

    pub fn walk_left(&mut self, time: f64){
        self.on = true;
    }

    pub fn walk_right(&mut self, time: f64){
        self.on = true;
        self.last_shot = time;
    }
}


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
        width: 800.0,
        height: 800.0,
        resizable: false,
        ..default()
    })
    .add_startup_system(setup)
    .add_startup_system_to_stage(StartupStage::PostStartup,setup_winit)
    .add_plugin(CatPlugin)
    .add_plugin(ScraperPlugin)
    .add_plugins(DefaultPlugins)
    .run();
}

fn setup_winit(windows: NonSend<WinitWindows>,) {
    //bevy doesn't have a way to set always_on_top, so use winit 
    let primary = windows.get_window(WindowId::primary()).unwrap();
    primary.set_always_on_top(true);
    let _clickthrough = match primary.set_cursor_hittest(false){
        Err(why) => panic!("couldn't open: {}",why),
        Ok(file) => file,
    };
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

