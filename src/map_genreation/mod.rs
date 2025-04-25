use bevy::prelude::*;
use systems::default_system;
use crate::map_genreation::systems::worley_system;

mod components;
mod resources;
mod systems;

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, worley_system);
    }
}
