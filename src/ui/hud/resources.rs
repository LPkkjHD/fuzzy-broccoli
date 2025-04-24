use bevy::{asset::Handle, image::Image, prelude::Resource};

#[derive(Resource)]
pub struct HealthBarAssets {
    pub background: Handle<Image>,
    pub border: Handle<Image>,
    pub heart: Handle<Image>,
}
