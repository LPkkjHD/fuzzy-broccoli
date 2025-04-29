use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct PlayerAnimationFrames(pub HashMap<PlayerFacingDirection, Vec<Handle<Image>>>);

#[derive(Component, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PlayerFacingDirection {
    Up,
    Down,
    Left,
    Right,
}
