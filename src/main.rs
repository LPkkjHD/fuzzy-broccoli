use avian2d::prelude::*;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use fuzzy_broccoli::audio::AudioPlugin;
use main_menu::MainMenuPlugin;
use map_genreation::MapGenerationPlugin;
use player::PlayerPlugin;

mod main_menu;

mod audio;
mod enemy;
mod map_genreation;
pub mod player;
mod ui;

fn main() {
    let app_name = if cfg!(debug_assertions) {
        "dev App"
    } else {
        "App"
    };
    let mut app = App::new();
    // Add plugins
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: app_name.into(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        bevy_svg::prelude::SvgPlugin,
        PhysicsPlugins::default(),
    ));

    // Add custom plugins
    app.add_plugins((
        MainMenuPlugin,
        PlayerPlugin,
        EnemyPlugin,
        MapGenerationPlugin,
        AudioPlugin,
    ));
    app.insert_resource(Gravity(Vec2::ZERO));


    // run the app
    app.run();
}
