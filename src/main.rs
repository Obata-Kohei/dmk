use bevy::prelude::*;
mod component;
mod system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}