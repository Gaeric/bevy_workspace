mod states;

use bevy::prelude::*;

fn main() {
    println!("Hello, world!");
    App::new()
        .insert_resource(WindowDescriptor {
            title: "hahaha".to_string(),
            resizable: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
