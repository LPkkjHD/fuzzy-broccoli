use bevy::prelude::*;
// use components::*;
use resources::*;
use systems::*;

mod components;
mod resources;
mod systems;
use crate::{enemy::resources::EnemyKillCount, AppState};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_health_bar_assets);

        app.add_systems(OnEnter(AppState::InGame), spawn_health_bar_container_system)
            .add_systems(OnExit(AppState::InGame), despawn_hud_system)
            .add_systems(OnEnter(AppState::InGame), spawn_score_widget_system)
            .add_systems(OnExit(AppState::InGame), despawn_score_widget_system)
            .add_systems(
                Update,
                update_health_bar_system
                    .after(spawn_health_bar_container_system)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                update_health_system
                    .after(update_health_bar_system)
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                update_score_widget_system.run_if(resource_changed::<EnemyKillCount>),
            );
    }
}
