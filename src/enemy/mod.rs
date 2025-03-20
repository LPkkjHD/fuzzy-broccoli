use bevy::prelude::*;
use systems::default_system;

mod systems;

pub(crate) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, default_system);
    }
}
