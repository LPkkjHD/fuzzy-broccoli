use crate::map_genreation::components::ResetTerrainEvent;
use crate::map_genreation::resources::{CurrentChunks, GenerationSeed, GroundTiles};
use crate::map_genreation::systems::{
    clean_ground_tiles, despawn_chunks, handle_player_chunk_update_event,
    handle_terrain_reset_event,
};
use crate::player::components::PlayerChunkUpdateEvent;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::IntoSystemConfigs;
use bevy::time::common_conditions::on_timer;
use bevy::utils::{HashMap, HashSet};
use rand::Rng;
use std::time::Duration;

mod components;
mod config;
mod resources;
mod systems;
pub mod util;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::thread_rng();
        app.add_event::<PlayerChunkUpdateEvent>();
        app.insert_resource(GroundTiles(HashSet::new()))
            .insert_resource(CurrentChunks(HashMap::new()))
            .insert_resource(GenerationSeed(rng.gen()))
            .add_systems(Update, handle_terrain_reset_event)
            .add_systems(Update, despawn_chunks)
            .add_systems(
                Update,
                clean_ground_tiles.run_if(on_timer(Duration::from_secs_f32(2.0))),
            )
            .add_systems(Update, handle_player_chunk_update_event)
            .add_event::<ResetTerrainEvent>();
    }
}

