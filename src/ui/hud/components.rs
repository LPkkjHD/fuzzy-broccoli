use bevy::prelude::Component;

#[derive(Component)]
pub struct HealthBarContainer;

#[derive(Component)]
pub struct HealthBarElement {
    pub index: u8,
    pub element_type: HealthElementType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HealthElementType {
    Background,
    Border,
    Heart,
}
