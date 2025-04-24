use crate::player::components::PlayerHealth;

use super::{components::*, HealthBarAssets};
use bevy::prelude::*;

pub fn spawn_hud_system(
    mut commands: Commands,
    player_health_query: Query<&PlayerHealth, Added<PlayerHealth>>,
    container_query: Query<Entity, With<HealthBarContainer>>,
    health_assets: Res<HealthBarAssets>,
) {
    if let Ok(player_health) = player_health_query.get_single() {
        if container_query.is_empty() {
            info!(
                "Player added with health {}, setting HUD.",
                player_health.max_health
            );
            commands
                .spawn((
                    Node {
                        // min_width: Val::Px(200.0),
                        // min_height: Val::Px(16.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(10.0),
                        top: Val::Px(10.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(4.0),
                        align_items: AlignItems::Center,
                        // width: Val::Auto,
                        // height: Val::Px(32.0),
                        ..default()
                    },
                    // BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                    HealthBarContainer,
                    Name::new("HealthBarContainer"),
                ))
                .with_children(|parent| {
                    for i in 0..player_health.max_health {
                        parent
                            .spawn(Node {
                                width: Val::Px(32.0),
                                height: Val::Px(32.0),
                                position_type: PositionType::Relative,
                                ..default()
                            })
                            .with_children(|slot| {
                                // Background Based on max health
                                slot.spawn((
                                    HealthBarElement {
                                        index: i,
                                        element_type: HealthElementType::Background,
                                    },
                                    ImageNode {
                                        image: health_assets.background.clone(),
                                        ..default()
                                    },
                                ));
                                // Border (Visible based on current
                                // health)
                                slot.spawn((
                                    HealthBarElement {
                                        index: i,
                                        element_type: HealthElementType::Border,
                                    },
                                    ImageNode {
                                        image: health_assets.border.clone(),
                                        ..default()
                                    },
                                ));
                                // Heart (Visible based on current
                                // health)
                                slot.spawn((
                                    HealthBarElement {
                                        index: i,
                                        element_type: HealthElementType::Heart,
                                    },
                                    ImageNode {
                                        image: health_assets.heart.clone(),
                                        ..default()
                                    },
                                ));
                            });
                    }
                });
        }
    };
}

pub fn load_health_bar_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = HealthBarAssets {
        background: asset_server.load("Hearts/PNG/basic/background.png"),
        border: asset_server.load("Hearts/PNG/basic/border.png"),
        heart: asset_server.load("Hearts/PNG/basic/heart.png"),
    };
    commands.insert_resource(assets);
}
