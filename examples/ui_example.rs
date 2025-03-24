use bevy::prelude::*;


pub fn main() {
    let mut app = App::new();
    // default system setup
    app.add_plugins((DefaultPlugins, UiPlugin));

    app.add_systems(Startup, startup);

    app.run();
}

pub fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
