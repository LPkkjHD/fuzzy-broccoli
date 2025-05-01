use bevy::prelude::Component;

#[derive(Component)]
pub struct HealthBarContainer;

#[derive(Component, Debug, Clone, Copy)]
pub struct HeartSlot {
    pub index: u8,
}
#[derive(Component, Debug, Clone, Copy)]
pub struct HeartForeground {
    pub index: u8,
}

#[derive(Component)]
pub struct ScoreHudContainer;

#[derive(Component)]
pub struct KillCountMarker;

#[derive(Component)]
pub struct TimerHudContainer;

#[derive(Component)]
pub struct TimerTextMarker;