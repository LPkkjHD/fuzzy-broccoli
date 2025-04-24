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
    current_health: u8,
    max_health: u8,
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
    pub fn increase_health(self: &mut Self, amount: u8) {
        self.current_health += amount;
    }
    pub fn decrease_health(self: &mut Self, amount: u8) {
        self.current_health -= amount;
    }
    pub fn increase_max_health(self: &mut Self, amount: u8) {
        self.max_health += amount;
    }
    pub fn decrease_max_health(self: &mut Self, amount: u8) {
        self.max_health -= amount;
        if self.max_health < self.current_health {
            self.current_health = self.max_health;
        }
    }

    pub fn current_health(&self) -> u8 {
        self.current_health
    }

    pub fn max_health(&self) -> u8 {
        self.max_health
    }
}
