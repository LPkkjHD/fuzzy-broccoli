use bevy::prelude::{Entity, Resource};
use bevy::utils::{HashMap, HashSet};

#[derive(Resource)]
pub struct GroundTiles(pub HashSet<(i32, i32)>);
#[derive(Resource)]
pub struct CurrentChunks(pub HashMap<(i32, i32), Vec<Entity>>);
#[derive(Resource)]
pub struct GenerationSeed(pub u32);