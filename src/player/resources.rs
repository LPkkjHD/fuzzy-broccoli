use avian2d::parry::utils::hashmap::HashMap;
use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerAnimationFrames(pub HashMap<FacingDirection, Vec<Handle<Image>>>);

#[derive(Component, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct DirectionalSprites {
    pub up: Handle<Image>,
    pub down: Handle<Image>,
    pub left: Handle<Image>,
    pub right: Handle<Image>,
}

