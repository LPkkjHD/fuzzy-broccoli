use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Debug)]
pub struct EnemyHealth(pub f32);

#[derive(Component)]
pub enum EnemyType {
    Zombie { speed: f32 },
    Skeleton { speed: f32 },
    Boss { speed: f32 },
}

#[derive(Component, Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Resource)]
pub struct WaveTimer {
    pub timer: Timer,
    pub wave: u32,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationFrame(pub u8);

#[derive(Resource)]
pub struct EnemyAnimationFrames {
    pub zombie: HashMap<FacingDirection, Vec<Handle<Image>>>,
    pub skeleton: HashMap<FacingDirection, Vec<Handle<Image>>>,
    pub boss: HashMap<FacingDirection, Vec<Handle<Image>>>,
}

#[derive(Component, PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}
