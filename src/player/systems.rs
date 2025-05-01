use std::collections::HashMap;

use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};
use crate::map_genreation::util::{center_to_top_left_grid, grid_to_chunk, world_to_grid};
use crate::map_genreation::resources::GroundTiles;

use crate::{
    collision::GameLayer,
    enemy::components::Enemy,
};

use super::{
    components::*,
    pistol::events::WeaponFiredEvent,
    resources::{PlayerAnimationFrames, PlayerFacingDirection, WorldMouseCoordinates},
    PlayerState,
};

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Sprite{
            image: asset_server.load("zombie_apocalypse_tileset/organized_separated_sprites/Player Character Walking Animation Frames/Zombie-Tileset---_0476_Capa-477.png"),
            custom_size: Some(Vec2::new(32.0,32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 9.0),
        PlayerMovementSpeed(100.0),
        PlayerHealth::new(3),
        RigidBody::Kinematic,
        Collider::round_rectangle(17.0, 20.0, 4.0),
        Mass(10.0),
        PlayerAnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        PlayerAnimationFrame(0),
        PlayerFacingDirection::Down,
        CollisionLayers::new(GameLayer::PLAYER, [GameLayer::ENEMY, GameLayer::PLAYER]),
    ));
}

pub fn spawn_player_camera(
    mut commands: Commands,
    player_location_query: Query<&Transform, With<Player>>,
) {
    let player_position = player_location_query.get_single().unwrap();
    commands.spawn((
        PlayerCamera,
        Transform::from_xyz(
            player_position.translation.x,
            player_position.translation.y,
            player_position.translation.z,
        )
        .with_scale(Vec3 {
            x: 0.75,
            y: 0.75,
            z: 1.0,
        }),
    ));
}

pub fn player_ground_collision_system(
    mut player_query: Query<(&mut Transform, &PlayerFacingDirection), With<Player>>,
    ground_tiles: Res<GroundTiles>,
) {
    if let Ok((mut transform, facing_direction)) = player_query.get_single_mut() {
        let current_pos = transform.translation;

        let (grid_x, grid_y) = world_to_grid(current_pos.x, current_pos.y);
        let (grid_x, grid_y) = center_to_top_left_grid(grid_x, grid_y);
        let grid_coords = (grid_x.round() as i32, grid_y.round() as i32);

        if !ground_tiles.0.contains(&grid_coords) {
            let offset = match facing_direction {
                PlayerFacingDirection::Left => Vec3::new(16.0, 0.0, 0.0),
                PlayerFacingDirection::Right => Vec3::new(-16.0, 0.0, 0.0),
                PlayerFacingDirection::Up => Vec3::new(0.0, -16.0, 0.0),
                PlayerFacingDirection::Down => Vec3::new(0.0, 16.0, 0.0),
            };
            transform.translation += offset;
        }
    }
}


pub fn player_movement_system(
    mut player_query: Query<
        (
            &mut LinearVelocity,
            &mut PlayerFacingDirection,
            &PlayerMovementSpeed,
        ),
        With<Player>,
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<PlayerState>>,
) {
    if let Ok((mut velocity, mut facing_direction, movement_speed)) = player_query.get_single_mut()
    {
        let mut direction: Vec2 = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            *facing_direction = PlayerFacingDirection::Left;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            *facing_direction = PlayerFacingDirection::Right;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
            *facing_direction = PlayerFacingDirection::Up;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
            *facing_direction = PlayerFacingDirection::Down;
        }
        let move_delta = direction.normalize_or_zero() * movement_speed.0;

        velocity.x = move_delta.x;
        velocity.y = move_delta.y;

        if direction != Vec2::ZERO {
            next_app_state.set(PlayerState::Moving);
        } else {
            next_app_state.set(PlayerState::Idle);
        }
    }
}

pub fn update_player_chunk_pos(
    mut chunk_pos: ResMut<CurrentPlayerChunkPos>,
    mut ev_chunk_update: EventWriter<PlayerChunkUpdateEvent>,
    player_query: Query<&Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let transform = player_query.single();
    let (x, y) = (transform.translation.x, transform.translation.y);
    let (a, b) = world_to_grid(x, y);
    let (a, b) = center_to_top_left_grid(a, b);
    let (x, y) = grid_to_chunk(a, b);

    let (old_x, old_y) = chunk_pos.0;
    if old_x == x && old_y == y {
        return;
    }

    ev_chunk_update.send(PlayerChunkUpdateEvent((x, y)));
    chunk_pos.0 = (x, y);
}


pub fn move_camera(
    time: Res<Time>,
    mut player_camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player_location_query: Query<&Transform, With<Player>>,
) {
    let Ok(mut camera) = player_camera_query.get_single_mut() else {
        return;
    };

    let Ok(player) = player_location_query.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera.translation = camera.translation.lerp(direction, time.delta_secs() * 4.0);
}

pub fn zoom_control_system(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut OrthographicProjection, With<PlayerCamera>>,
) {
    let mut projection = camera_query.single_mut();

    if input.pressed(KeyCode::Digit0) {
        projection.scale += 0.05;
    }

    if input.pressed(KeyCode::Digit9) {
        projection.scale -= 0.05;
    }

    projection.scale = projection.scale.clamp(0.2, 5.);
}

pub fn player_debug_system(
    input: Res<ButtonInput<KeyCode>>,
    mut player_components_query: Query<&mut PlayerHealth, With<Player>>,
) {
    let mut player_health = player_components_query.single_mut();
    if input.just_pressed(KeyCode::KeyH) {
        player_health.decrease_health(1);
    }
    if input.just_pressed(KeyCode::KeyJ) {
        player_health.increase_health(1);
    }
    if input.just_pressed(KeyCode::KeyK) {
        player_health.decrease_max_health(1);
    }
    if input.just_pressed(KeyCode::KeyL) {
        player_health.increase_max_health(1);
    }
}

pub fn setup_player_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut player_frames: HashMap<PlayerFacingDirection, Vec<Handle<Image>>> = HashMap::new();

    for direction in [
        PlayerFacingDirection::Down,
        PlayerFacingDirection::Up,
        PlayerFacingDirection::Right,
    ] {
        let player_base = match direction {
            PlayerFacingDirection::Down => "476",
            PlayerFacingDirection::Right => "479",
            PlayerFacingDirection::Up => "482",
            PlayerFacingDirection::Left => continue,
        };

        let frames_vec: Vec<Handle<Image>> = (0..3)
            .map(|i| {
            let base = player_base.parse::<i32>().unwrap()+i;
            let next = base+1;
            let animation_path = format!("zombie_apocalypse_tileset/organized_separated_sprites/Player Character Walking Animation Frames/Zombie-Tileset---_0{}_Capa-{}.png", base,next);
            asset_server.load(animation_path)
        }).collect();
        player_frames.insert(direction, frames_vec);
    }
    if let Some(left_frames) = player_frames.get(&PlayerFacingDirection::Right) {
        player_frames.insert(PlayerFacingDirection::Left, left_frames.clone());
    }
    commands.insert_resource(PlayerAnimationFrames(player_frames));
}
pub fn player_animation_tick_system(
    mut query: Query<(&mut PlayerAnimationTimer, &mut PlayerAnimationFrame)>,
    time: Res<Time>,
) {
    for (mut timer, mut frame) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            frame.0 = match frame.0 {
                0 => 1,
                1 => 2,
                2 => 3,
                _ => 0,
            };
        }
    }
}
pub fn player_movement_animation_system(
    mut player_animation_query: Query<
        (
            &mut PlayerFacingDirection,
            &mut Sprite,
            &PlayerAnimationFrame,
        ),
        With<Player>,
    >,
    player_animation_frames: Res<PlayerAnimationFrames>,
) {
    let (facing_direction, mut sprite, player_animation_frame) =
        player_animation_query.single_mut();

    let frames = &player_animation_frames.0;
    let direction_frames = &frames[&facing_direction];
    let frame_index = match player_animation_frame.0 {
        0 => 0,
        1 => 1,
        2 => 0,
        3 => 2,
        _ => 0,
    };
    sprite.image = direction_frames[frame_index].clone();
    sprite.flip_x = *facing_direction == PlayerFacingDirection::Left;
}

pub fn set_player_animation_to_start_frame(
    mut player_frame_query: Query<&mut PlayerAnimationFrame>,
) {
    if let Ok(mut frame) = player_frame_query.get_single_mut() {
        frame.0 = 0;
    }
}

// For some reason mouse has insane input lag on macos. FML
pub fn fire_weapon_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<WeaponFiredEvent>,
    cursor_position: Res<WorldMouseCoordinates>,
) {
    if mouse_input.just_pressed(MouseButton::Left) || key_input.just_pressed(KeyCode::Enter)  {
        event_writer.send(WeaponFiredEvent(cursor_position.0));
    }
}

pub fn cursor_system(
    mut mycoords: ResMut<WorldMouseCoordinates>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}

pub fn player_enemy_collision_damage_system(
    mut commands: Commands,
    time: Res<Time>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut player_query: Query<(Entity, &mut PlayerHealth, Option<&mut DamageInvulnerability>), With<Player>>,
    mut collision_events: EventReader<Collision>,
) {
    let Ok((player_entity, mut player_health, invulnerability)) = player_query.get_single_mut() else { return };

    if let Some(mut invulnerability) = invulnerability {
        invulnerability.timer.tick(time.delta());
        if !invulnerability.timer.finished() {
            return;
        }
        commands.entity(player_entity).remove::<DamageInvulnerability>();
    }

    for collision in collision_events.read() {
        let entity1 = collision.0.entity1;
        let entity2 = collision.0.entity2;

        let is_enemy_collision = (entity1 == player_entity && enemy_query.get(entity2).is_ok()) ||
            (entity2 == player_entity && enemy_query.get(entity1).is_ok());

        if is_enemy_collision {
            player_health.decrease_health(1);
            commands.entity(player_entity).insert(DamageInvulnerability::default());
            break;
        }
    }
}