use bevy::{prelude::*, window::WindowDescriptor};
use bevy::window::WindowId;
use bevy::winit::WinitWindows;

fn main() {
    setup_bevy();
}


fn setup_bevy(){
    App::new()
    // ClearColor must have 0 alpha, otherwise some color will bleed through
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
    .add_plugins(DefaultPlugins)
    .run();
}

fn setup_winit(windows: NonSend<WinitWindows>,) {
    //bevy doesn't have a way to set always_on_top, so use winit 
    let primary = windows.get_window(WindowId::primary()).unwrap();
    primary.set_always_on_top(true);
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle{
        texture: asset_server.load("sleepyKitty_Idle00.png"),
        ..default()
    });
}