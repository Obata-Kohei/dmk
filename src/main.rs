use bevy::prelude::*;

use crate::{component::{Acceleration, Collider, Player, Velocity}, system::{acceleration_system, movement_system, player_key_input_system, confine_player_system}};
mod component;
mod system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_key_input_system, acceleration_system, movement_system, confine_player_system).chain())
        .run();
}

fn setup(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    cmd.spawn(Camera2d::default());

    cmd.spawn((
        Player,
        Sprite::from_image(asset_server.load("test.png")),
        Transform::default(),
        Velocity(Vec2::ZERO),
        Acceleration(Vec2::ZERO),
        Collider {radius: 32.0},
    ));
}