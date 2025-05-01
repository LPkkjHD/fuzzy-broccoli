use bevy::prelude::*;
use fuzzy_broccoli::ui::UiPlugin;


pub fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, UiPlugin));

    app.add_systems(Startup, startup);

    app.run();
}

pub fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
