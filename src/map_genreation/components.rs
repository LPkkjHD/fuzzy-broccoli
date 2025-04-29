use bevy::prelude::{Component, Event};

#[derive(Component)]
pub struct TileComponent;
#[derive(Event)]
pub struct ResetTerrainEvent;

#[derive(Eq, PartialEq, Hash)]
pub struct Tile {
    pub(crate) pos: (i32, i32),
    pub(crate) sprite: usize,
    z_index: i32,
}

impl Tile {
    pub fn new(pos: (i32, i32), sprite: usize, z_index: i32) -> Self {
        Self {
            pos,
            sprite,
            z_index,
        }
    }
}