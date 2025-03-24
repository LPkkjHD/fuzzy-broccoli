use bevy::prelude::*;
use fuzzy_broccoli::player::PlayerPlugin;

pub fn main() {
    let mut app = App::new();
    // default system setup
    app.add_plugins(DefaultPlugins);
    app.add_plugins(PlayerPlugin);
    app.run();
}
