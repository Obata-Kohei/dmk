use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity(pub f32);

#[derive(Component, Debug)]
pub struct Acceleration(pub f32);

#[derive(Component, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Component, Debug)]
pub struct Name(pub String);

#[derive(Component, Debug)]
pub struct Shooter;

#[derive(Component, Debug)]
pub struct Bullet;

#[derive(Component, Debug)]
pub enum Trajectory {
    Straight { v0: Velocity, a: Acceleration },
    //Curveとか
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Setup,
    MainMenu,
    InGame,
    GameOver,
}
