use bevy::prelude::*;

use super::components::*;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("./top_down_shooter/skins.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(24), 19, 11, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        Player,
        Sprite {
            image: asset_server.load("./top_down_shooter/skins.png"),
            ..default()
        },
        Transform::from_xyz(1.0, 1.0, 1.0),
        PlayerMovement::new(100.0),
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
    mut player_query: Query<(&mut Transform, &PlayerMovement), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, movement) in player_query.iter_mut() {
        let mut direction: Vec3 = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * movement.speed * time.delta_secs();
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
    let square_size = 32.0; // Size of each square in pixels
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
        projection.scale += 0.2;
    }

    if input.pressed(KeyCode::Digit9) {
        projection.scale -= 0.2;
    }

    projection.scale = projection.scale.clamp(0.2, 5.);
}
