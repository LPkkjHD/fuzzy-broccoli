use bevy::prelude::*;
use crate::AppState;

pub struct GameOverScreenPlugin;

impl Plugin for GameOverScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), setup_game_over_screen)
            .add_systems(OnExit(AppState::GameOver), cleanup_game_over_screen);
    }
}

fn setup_game_over_screen(mut commands: Commands) {
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
                "Game Over!\nYou were defeated!",
            ));
        });
}

fn cleanup_game_over_screen(mut commands: Commands, root: Query<Entity, Without<Parent>>) {
    for root_entity in root.iter() {
        commands.entity(root_entity).despawn_recursive();
    }
}