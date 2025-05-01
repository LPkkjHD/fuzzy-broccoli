use crate::AppState;
use crate::AlignSelf::Auto;
use crate::{
    enemy::resources::EnemyKillCount,
    player::components::{Player, PlayerHealth},
};

use super::{components::*, HealthBarAssets};
use bevy::prelude::*;
use crate::ui::hud::resources::GameTimer;

pub fn load_health_bar_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = HealthBarAssets {
        background: asset_server.load("Hearts/PNG/basic/background.png"),
        border: asset_server.load("Hearts/PNG/basic/border.png"),
        heart: asset_server.load("Hearts/PNG/basic/heart.png"),
    };
    commands.insert_resource(assets);
}

pub fn spawn_health_bar_container_system(
    mut commands: Commands,
    player_health_query: Query<&PlayerHealth, Added<PlayerHealth>>,
    container_query: Query<Entity, With<HealthBarContainer>>,
) {
    if let Ok(player_health) = player_health_query.get_single() {
        if container_query.is_empty() {
            info!(
                "Player added with health {}, setting HUD.",
                player_health.max_health()
            );
            commands.spawn((
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
            ));
        }
    };
}

//  Helper function to spawn a single heart slot with its children
fn spawn_single_heart_slot(parent: &mut ChildBuilder, index: u8, health_assets: &HealthBarAssets) {
    parent
        .spawn((
            Node {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                // align_items: AlignItems::Center,
                ..default()
            },
            HeartSlot { index },
            Name::new(format!("HeartSlot_{}", index)),
        ))
        .with_children(|slot| {
            // Background Based on max health
            slot.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ImageNode {
                    image: health_assets.background.clone(),
                    ..default()
                },
                Name::new(format!("HeartBg_{}", index)),
            ));
            // Border (Visible based on current
            // health)
            slot.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ImageNode {
                    image: health_assets.border.clone(),
                    ..default()
                },
                Name::new(format!("HeartBorder_{}", index)),
            ));
            // Heart (Visible based on current
            // health)
            slot.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ImageNode {
                    image: health_assets.heart.clone(),
                    ..default()
                },
                HeartForeground { index },
                Name::new(format!("HeartFg_{}", index)),
            ));
        });
}

pub fn despawn_hud_system(
    mut commands: Commands,
    hud_query: Query<Entity, With<HealthBarContainer>>,
) {
    commands
        .entity(hud_query.get_single().unwrap())
        .despawn_recursive();
}

pub fn update_health_bar_system(
    mut commands: Commands,
    player_query: Query<&PlayerHealth, (With<Player>, Changed<PlayerHealth>)>,
    container_query: Query<Entity, With<HealthBarContainer>>,
    slot_query: Query<(Entity, &HeartSlot)>,
    health_assets: Res<HealthBarAssets>,
) {
    if let Ok(player_health) = player_query.get_single() {
        let Ok(container_entity) = container_query.get_single() else {
            warn!("HealthBarContainer not found, cannot update HUD elements.");
            return;
        };

        info!(
            "Player health changed to {}/{}. Updating HUD visibility.",
            player_health.current_health(),
            player_health.max_health()
        );
        let new_max_health = player_health.max_health();
        let mut current_slots = Vec::new();
        for (entity, slot) in slot_query.iter() {
            current_slots.push((entity, slot.index));
        }
        let current_slot_count = current_slots.len() as u8;

        // 1. Despawn excess slots
        if new_max_health < current_slot_count {
            info!(
                "Max health decreased. Despawning {} slots",
                current_slot_count - new_max_health
            );
            current_slots.sort_by(|a, b| b.1.cmp(&a.1));
            for (entity, index) in current_slots.iter() {
                if *index >= new_max_health {
                    commands.entity(*entity).despawn_recursive();
                } else {
                    break;
                }
            }
        }
        // 2. Spawn missing slots:
        if new_max_health > current_slot_count {
            info!(
                "Max health increased. Spawning {} slots.",
                new_max_health - current_slot_count
            );
            if let Some(mut container_commands) = commands.get_entity(container_entity) {
                container_commands.with_children(|parent| {
                    for i in current_slot_count..new_max_health {
                        spawn_single_heart_slot(parent, i, &health_assets);
                    }
                });
            } else {
                error!("Could not get commands for HealthBarncontainer entity while trying to add children")
            }
        }
    }
}

pub fn update_health_system(
    mut heart_fg_query: Query<(&HeartForeground, &mut Visibility)>,
    player_query: Query<&PlayerHealth, (With<Player>, With<PlayerHealth>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(player_health) = player_query.get_single() {
        if player_health.current_health() == 0 {
            next_state.set(AppState::GameOver);
            return;
        }

        for (heart_fg, mut visibility) in heart_fg_query.iter_mut() {
            *visibility = if heart_fg.index < player_health.current_health() {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            }
        }
    }
}

pub fn spawn_score_widget_system(mut commands: Commands, kill_count_resource: Res<EnemyKillCount>) {
    let score_container = (
        ScoreHudContainer,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
    );
    let score_text = (
        KillCountMarker,
        Text::new(format!("Kills: {}", kill_count_resource.0)),
    );
    commands.spawn(score_container).with_child(score_text);
}

pub fn despawn_score_widget_system(
    mut commands: Commands,
    score_hud_query: Query<Entity, With<ScoreHudContainer>>,
) {
    if let Ok(score_hud_entity) = score_hud_query.get_single() {
        commands.entity(score_hud_entity).despawn_recursive();
    }
}

pub fn update_score_widget_system(
    mut score_text_query: Query<&mut Text, With<KillCountMarker>>,
    score_resource: Res<EnemyKillCount>,
) {
    info!(
        "Score update system triggered for {} kills",
        score_resource.0
    ); // Add this log!
    for mut text in &mut score_text_query {
        **text = format!("Kills: {}", score_resource.0);
    }
}

pub fn spawn_timer_widget_system(mut commands: Commands) {
    let timer_container = (
        TimerHudContainer,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            margin: UiRect::horizontal(Val::Auto),
            justify_content: JustifyContent::Center,
            ..default()
        },
    );

    let timer_text = (
        TimerTextMarker,
        Text::new("Time: 5:00"),
    );

    commands.spawn(timer_container).with_child(timer_text);
}

pub fn despawn_timer_widget_system(
    mut commands: Commands,
    timer_query: Query<Entity, With<TimerHudContainer>>,
) {
    if let Ok(timer_entity) = timer_query.get_single() {
        commands.entity(timer_entity).despawn_recursive();
    }
}

pub fn update_timer_system(
    time: Res<Time>,
    mut game_timer: ResMut<GameTimer>,
    mut timer_query: Query<&mut Text, With<TimerTextMarker>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    game_timer.remaining_seconds -= time.delta_secs();

    if game_timer.remaining_seconds <= 0.0 {
        game_timer.remaining_seconds = 0.0;
        next_state.set(AppState::GameWon);
        return;
    }

    let minutes = (game_timer.remaining_seconds / 60.0).floor();
    let seconds = (game_timer.remaining_seconds % 60.0).floor();

    for mut text in &mut timer_query {
        **text = format!("Time: {:.0}:{:02.0}", minutes, seconds);
    }
}