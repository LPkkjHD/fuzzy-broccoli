use bevy::prelude::*;
use fuzzy_broccoli::map_genreation;

pub fn main() {
    let mut app = App::new();
    app.add_plugins(map_genreation::TerrainPlugin);

    app.add_systems(Startup, startup);

    app.run();
}

pub fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

