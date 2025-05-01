use bevy::prelude::{Component, Event};

#[derive(Component)]
pub struct TileComponent;
#[derive(Event)]
pub struct ResetTerrainEvent;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Tile {
    pub pos: (i32, i32),
    pub sprite: usize,
    pub z_index: i32,
    pub(crate) rotation: i32,
}

impl Tile {
    pub fn new(pos: (i32, i32), sprite: usize, z_index: i32) -> Self {
        Self {
            pos,
            sprite,
            z_index,
            rotation: 0,
        }
    }

    pub fn with_rotation(pos: (i32, i32), sprite: usize, z_index: i32, rotation: i32) -> Self {
        Self {
            pos,
            sprite,
            z_index,
            rotation,
        }
    }
}