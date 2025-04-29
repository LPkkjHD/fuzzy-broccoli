use bevy::prelude::Component;

#[derive(Component)]
pub struct Pistol;

#[derive(Component)]
pub struct Damage(pub f32);

/// Attacks per second for example 5.0 means 5 attacks per second.
#[derive(Component)]
pub struct AttacksPerSecond(pub f32);
