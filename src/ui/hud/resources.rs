use bevy::{asset::Handle, image::Image, prelude::Resource};

#[derive(Resource)]
pub struct HealthBarAssets {
    pub background: Handle<Image>,
    pub border: Handle<Image>,
    pub heart: Handle<Image>,
}

#[derive(Resource)]
pub struct GameTimer {
    pub remaining_seconds: f32,
}

impl Default for GameTimer {
    fn default() -> Self {
        Self {
            remaining_seconds: 300.0 // 5 minutes in seconds
        }
    }
}