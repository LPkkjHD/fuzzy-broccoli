use bevy::prelude::*;

// This is needed to acces the modules in src/module_name in the examples folder. This way we can
// write seperated unit test like things for our base behaviour and skip state stuff later on in
// game development.
pub mod audio;
pub mod enemy;
pub mod map_genreation;
pub mod player;
pub mod ui;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}
