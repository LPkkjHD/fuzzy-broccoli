use bevy::prelude::*;

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, startup);

    app.run();
}

pub fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
