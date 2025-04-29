use bevy::prelude::*;

#[derive(Event)]
pub struct WeaponFiredEvent(pub Vec2);
