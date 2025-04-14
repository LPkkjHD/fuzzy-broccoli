use bevy::prelude::*;
use systems::*;
use components::*;

mod components;
mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(WaveTimer {
                timer: Timer::from_seconds(30.0, TimerMode::Repeating),
                wave: 0,
            })
            .add_systems(Startup, setup_enemy_sprites)
            .add_systems(Update, (
                spawn_enemy_system,
                enemy_movement_and_direction_system,
                animate_enemy_system,
            ));
    }
}