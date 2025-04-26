use bevy::prelude::*;
use systems::*;

pub mod components;
mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // setup_board function is just for testing basic at the beginning.
        #[cfg(debug_assertions)]
        app.add_systems(Startup, setup_board);
        app.add_systems(
            Startup,
            (setup_player_sprites, spawn_player, spawn_player_camera).chain(),
        );
        app.add_systems(Update, (player_movement_system, move_camera).chain());
        app.add_systems(Update, zoom_control_system);
        app.add_systems(
            Update,
            (player_animation_tick_system, player_animation_system),
        );
        #[cfg(debug_assertions)]
        app.add_systems(Update, player_debug_system);
    }
}
