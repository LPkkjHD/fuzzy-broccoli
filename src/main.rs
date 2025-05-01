use avian2d::prelude::*;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use fuzzy_broccoli::{audio::AudioPlugin, AppState};
use main_menu::MainMenuPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;
use crate::map_genreation::TerrainPlugin;

mod main_menu;

mod audio;
mod enemy;
mod map_genreation;
pub mod player;
mod ui;
pub mod collision;

fn main() {
    let app_name = if cfg!(debug_assertions) {
        "dev App"
    } else {
        "App"
    };
    let mut app = App::new();
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
        PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()),
    ));

    #[cfg(debug_assertions)]
    app.add_plugins(PhysicsDebugPlugin::default());

    app.add_plugins((
        MainMenuPlugin,
        PlayerPlugin,
        EnemyPlugin,
        TerrainPlugin,
        AudioPlugin,
        UiPlugin,
    ));
    app.insert_resource(Gravity(Vec2::ZERO));

    app.init_state::<AppState>();

    app.run();
}
