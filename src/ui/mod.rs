use bevy::prelude::*;

use crate::AppState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_ui_system)
            .add_systems(OnExit(AppState::InGame), dewspawn_ui_system);
    }
}

#[derive(Component)]
struct HealthBarUIRoot;

#[derive(Component)]
struct HealthHeartUI;

fn spawn_ui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            // min_width: Val::Px(200.0),
            // min_height: Val::Px(16.0),
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            row_gap: Val::Px(4.0),
            align_items: AlignItems::Center,
            width: Val::Auto,
            height: Val::Px(32.0),
            ..default()
        },
        // BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
        ImageNode {
            image: asset_server.load("Hearts/PNG/basic/background.png"),
            ..Default::default()
        },
        HealthBarUIRoot,
        Name::new("HealthBarUI"),
    ));
}
// This will despawn the ui after the InGameState is exited.
fn dewspawn_ui_system(mut commands: Commands, ui_query: Query<Entity, With<HealthBarUIRoot>>) {
    commands.entity(ui_query.single()).despawn_recursive();
}
