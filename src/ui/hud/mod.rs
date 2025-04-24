use bevy::prelude::*;
use components::*;
use resources::*;
use systems::*;

mod components;
mod resources;
mod systems;
use crate::AppState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_health_bar_assets);

        app.add_systems(OnEnter(AppState::InGame), spawn_hud_system);
    }
}
