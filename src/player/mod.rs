use bevy::prelude::*;
use resources::default_system;

mod components;
mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, default_system);
    }
}
