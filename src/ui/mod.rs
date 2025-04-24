use bevy::prelude::*;
use hud::HudPlugin;
mod hud;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HudPlugin);
    }
}
