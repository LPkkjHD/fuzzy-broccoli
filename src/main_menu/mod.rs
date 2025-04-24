use bevy::prelude::*;
use fuzzy_broccoli::AppState;
use systems::interactions::{interact_with_play_button, interact_with_quit_button};

pub mod components;
pub mod styles;
pub mod systems;

use crate::main_menu::systems::layouts::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                (interact_with_play_button, interact_with_quit_button)
                    .run_if(in_state(AppState::MainMenu)),
            );
    }
}
