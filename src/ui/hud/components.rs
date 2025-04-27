use bevy::prelude::Component;

#[derive(Component)]
pub struct HealthBarContainer;

#[derive(Component)]
pub struct HealthBarElement {
    pub index: u8,
    pub element_type: HealthElementType,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct HeartSlot {
    pub index: u8,
}
#[derive(Component, Debug, Clone, Copy)]
pub struct HeartForeground {
    pub index: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HealthElementType {
    Background,
    Border,
    Heart,
}

// stuff for enemy kill counter

#[derive(Component)]
pub struct ScoreHudContainer;
