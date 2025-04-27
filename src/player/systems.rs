use std::collections::HashMap;

use avian2d::prelude::*;
use bevy::prelude::*;

use super::{
    components::*,
    resources::{FacingDirection, PlayerAnimationFrames},
};

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Sprite{
            image: asset_server.load("zombie_apocalypse_tileset/organized_separated_sprites/Player Character Walking Animation Frames/Zombie-Tileset---_0476_Capa-477.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
            // .with_scale(Vec3::new(2.0,2.0,1.0)),
        PlayerMovementSpeed(100.0),
        // Add PlayerHealth Component with default values of 3/3 lifes/max_lifes
        PlayerHealth::new(3),
        RigidBody::Kinematic,
        Collider::capsule(16.0, 16.0),
        Mass(10.0),
        PlayerAnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        PlayerAnimationFrame(0),
        FacingDirection::Down,
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

pub fn player_movement_system(
    time: Res<Time>,
    mut player_query: Query<
        (&mut Transform, &mut FacingDirection, &PlayerMovementSpeed),
        With<Player>,
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, mut facing_direction, movement) in player_query.iter_mut() {
        let mut direction: Vec3 = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
            *facing_direction = FacingDirection::Left;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            *facing_direction = FacingDirection::Right;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            *facing_direction = FacingDirection::Up;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
            *facing_direction = FacingDirection::Down;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * movement.0 * time.delta_secs();
    }
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
// System to set up the checkerboard background
pub fn setup_board(mut commands: Commands) {
    let square_size = 16.0; // Size of each square in pixels
    let board_size_squares_x = 100; // Number of squares in x-direction
    let board_size_squares_y = 100; // Number of squares in y-direction

    let color_light = Color::srgb(0.7, 0.7, 0.7); // Light gray
    let color_dark = Color::srgb(0.3, 0.3, 0.3); // Dark gray

    for row in 0..board_size_squares_y {
        for col in 0..board_size_squares_x {
            let color = if (row + col) % 2 == 0 {
                color_light
            } else {
                color_dark
            }; // Alternate colors
            let x_pos = (col as f32 - (board_size_squares_x as f32 / 2.0 - 0.5)) * square_size;
            let y_pos = (row as f32 - (board_size_squares_y as f32 / 2.0 - 0.5)) * square_size;

            commands.spawn((
                Sprite {
                    color,
                    rect: Some(Rect::new(0.0, 0.0, square_size, square_size)), // Define square size
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(x_pos, y_pos, -1.0)), // z-index -1 to put board behind player
            ));
        }
    }
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
    let mut player_frames: HashMap<FacingDirection, Vec<Handle<Image>>> = HashMap::new();

    for direction in [
        FacingDirection::Down,
        FacingDirection::Up,
        FacingDirection::Right,
    ] {
        let player_base = match direction {
            FacingDirection::Down => "476",
            FacingDirection::Right => "479",
            FacingDirection::Up => "482",
            FacingDirection::Left => continue,
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
    if let Some(left_frames) = player_frames.get(&FacingDirection::Right) {
        player_frames.insert(FacingDirection::Left, left_frames.clone());
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
pub fn player_animation_system(
    mut player_animation_query: Query<
        (&mut FacingDirection, &mut Sprite, &PlayerAnimationFrame),
        With<Player>,
    >,
    player_animation_frames: Res<PlayerAnimationFrames>,
) {
    let (mut facing_direction, mut sprite, player_animation_frame) =
        player_animation_query.single_mut();

    let frames = &player_animation_frames.0;
    let direction_frames = &frames[&facing_direction];
    let frame_index = match player_animation_frame.0 {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 0,
        _ => 0,
    };
    sprite.image = direction_frames[frame_index].clone();
    sprite.flip_x = *facing_direction == FacingDirection::Left;
}
