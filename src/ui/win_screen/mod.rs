use bevy::prelude::*;
use crate::AppState;

pub struct WinScreenPlugin;

impl Plugin for WinScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameWon), setup_win_screen)
            .add_systems(OnExit(AppState::GameWon), cleanup_win_screen);
    }
}

fn setup_win_screen(mut commands: Commands) {
    commands
        .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
        })
        .with_children(|parent| {
            parent.spawn(Text::from(
                "You Won!\nYou survived for 5 minutes!",
            ));
        });
}

fn cleanup_win_screen(mut commands: Commands, root: Query<Entity, Without<Parent>>) {
    for entity in root.iter() {
        commands.entity(entity).despawn_recursive();
    }
}