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

#[derive(Component)]
pub struct PlayerHealth {
    pub current_health: u8,
    pub max_health: u8,
}

impl Default for PlayerHealth {
    fn default() -> Self {
        Self {
            current_health: 3,
            max_health: 3,
        }
    }
}

impl PlayerHealth {
    pub fn new(max_health: u8) -> Self {
        Self {
            current_health: max_health,
            max_health,
        }
    }
}
