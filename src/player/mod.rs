use bevy::prelude::*;
use systems::*;

mod components;
mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // setup_board function is just for testing basic at the beginning. 
        app.add_systems(Startup, setup_board);
        app.add_systems(Startup, (spawn_player, spawn_player_camera).chain());
        app.add_systems(Update, (player_movement_system, move_camera).chain());
        app.add_systems(Update, zoom_control_system);
    }
}
