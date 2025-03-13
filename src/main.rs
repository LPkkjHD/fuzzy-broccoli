
mod broccoli;
use bevy::prelude::*;
use broccoli::BroccoliPlugin;
fn main() {
    let app_name = if cfg!(debug_assertions) {
        "dev App"
    } else {
        "App"
    };
    let mut app = App::new();
    // Add plugins
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: app_name.into(),
                ..default()
            }),
            ..default()
        }),
        bevy_svg::prelude::SvgPlugin,
    ));

    // Add custom plugins
    app.add_plugins(BroccoliPlugin);

    app.add_systems(Startup, (setup_camera, empty_system));

    // run the app
    app.run();
}

// This is an empty system to disable  the bevy app shutdown Immedatly.
pub fn empty_system() {}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d {});
}
