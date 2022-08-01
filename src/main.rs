use bevy::prelude::*;

fn main() {
    App::new()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "Companion!".to_string(),
            width: 320.0,
            height: 240.0,
            transparent: true,
            decorations: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}