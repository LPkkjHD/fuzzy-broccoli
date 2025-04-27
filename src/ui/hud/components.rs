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


// stuff for enemy kill counter

#[derive(Component)]
pub struct ScoreHudContainer;

#[derive(Component)]
pub struct KillCountMarker;
