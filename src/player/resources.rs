use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct PlayerAnimationFrames(pub HashMap<FacingDirection, Vec<Handle<Image>>>);

#[derive(Component, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}
