use bevy::prelude::*;
use components::*;
use resources::EnemyKillCount;
use systems::*;

use crate::AppState;

pub mod components;
pub mod resources;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(WaveTimer {
                timer: Timer::from_seconds(60.0, TimerMode::Repeating),
                wave: 0,
            })
            .insert_resource(EnemyKillCount(0))
            .add_systems(Startup, setup_enemy_sprites)
            .add_systems(
                Update,
                (
                    spawn_enemy_system,
                    enemy_movement_and_direction_system,
                    animate_enemy_system,
                    prevent_enemy_overlap_system,
                    kill_enemy_system,
                    wave_timer_system,
                )
                    .run_if(in_state(AppState::InGame)),
            );
        #[cfg(debug_assertions)]
        app.add_systems(Update, debug_enemy_keybinds_system);
    }
}
