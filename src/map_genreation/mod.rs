use bevy::prelude::*;
use systems::default_system;
mod components;
mod resources;
mod systems;

pub(crate) struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, default_system);
    }
}
