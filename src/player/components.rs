use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
#[require(Camera2d)]
pub struct PlayerCamera;

// Component to define player movement speed
#[derive(Component, Default)]
pub struct PlayerMovement {
    pub speed: f32,
}

impl PlayerMovement {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}
