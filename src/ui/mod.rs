use bevy::prelude::*;
use hud::HudPlugin;
use crate::ui::game_over::GameOverScreenPlugin;
use crate::ui::win_screen::WinScreenPlugin;

mod hud;
mod win_screen;
mod game_over;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HudPlugin,
            WinScreenPlugin,
            GameOverScreenPlugin,
        ));
    }
}

