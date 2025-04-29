use bevy::prelude::*;
use systems::spawn_pistol;

use crate::AppState;

use super::spawn_player;

mod components;
mod resources;
mod systems;

pub struct PistolPlugin;

impl Plugin for PistolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_pistol.after(spawn_player));
    }
}

