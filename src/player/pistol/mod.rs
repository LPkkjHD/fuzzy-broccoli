use bevy::prelude::*;
use events::WeaponFiredEvent;
use systems::{projectile_enemy_collision_damage_system, spawn_pistol, spawn_projectile_component};

use crate::AppState;

use super::{fire_weapon_system, spawn_player};

mod components;
pub mod events;
mod resources;
mod systems;
pub struct PistolPlugin;

impl Plugin for PistolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WeaponFiredEvent>();
        app.add_systems(OnEnter(AppState::InGame), spawn_pistol.after(spawn_player));
        app.add_systems(
            Update,
            spawn_projectile_component
                .after(fire_weapon_system)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            projectile_enemy_collision_damage_system.run_if(in_state(AppState::InGame)),
        );
    }
}
