use bevy::prelude::*;
use pistol::PistolPlugin;
use resources::WorldMouseCoordinates;
use systems::*;

pub mod components;
mod resources;
mod systems;

pub mod pistol;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PistolPlugin);
        // setup_board function is just for testing basic at the beginning.
        #[cfg(debug_assertions)]
        app.add_systems(Startup, setup_board);
        app.init_state::<PlayerState>();
        app.add_systems(
            Startup,
            (setup_player_sprites, spawn_player, spawn_player_camera).chain(),
        );
        app.add_systems(Update, (player_movement_system, move_camera).chain());
        app.add_systems(Update, zoom_control_system);
        app.add_systems(
            Update,
            (
                player_animation_tick_system,
                cursor_system,
                fire_weapon_system,
            ),
        );
        app.add_systems(
            Update,
            player_movement_animation_system.run_if(in_state(PlayerState::Moving)),
        );
        app.add_systems(
            OnEnter(PlayerState::Idle),
            set_player_animation_to_start_frame,
        );
        #[cfg(debug_assertions)]
        app.add_systems(Update, player_debug_system);

        app.init_resource::<WorldMouseCoordinates>();
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Moving,
}
