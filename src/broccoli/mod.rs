use bevy::prelude::*;
use systems::{rotate_broccoli, spawn_broccoli};

pub mod components;
pub mod systems;

pub struct BroccoliPlugin;

impl Plugin for BroccoliPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_broccoli)
            .add_systems(Update, rotate_broccoli);
    }
}
