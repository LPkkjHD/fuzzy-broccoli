use bevy::prelude::*;

use crate::main_menu::{
    components::{PlayButton, QuitButton},
    AppState,
};

pub fn interact_with_play_button(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => next_app_state.set(AppState::Game),
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn interact_with_quit_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok(interaction) = button_query.get_single() {
        match *interaction {
            Interaction::Pressed => {
                app_exit_event_writer.send(AppExit::Success);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
