use super::components::*;
use super::resources::EnemyKillCount;
use crate::collision::GameLayer;
use crate::player::components::Player;
use avian2d::collision::Collider;
use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;

/// Spawns an enemy with its enemy type, texture, and related components.
pub fn spawn_enemy_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut wave_timer: ResMut<WaveTimer>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    spawn_timer.0.tick(time.delta());
    wave_timer.timer.tick(time.delta());

    if wave_timer.timer.just_finished() {
        wave_timer.wave += 1;
    }

    if spawn_timer.0.just_finished() {
        let player_transform = player_query.single();
        let spawn_distance = 500.0;
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..2.0 * PI);
        let spawn_position = Vec3::new(
            player_transform.translation.x + spawn_distance * angle.cos(),
            player_transform.translation.y + spawn_distance * angle.sin(),
            10.0,
        );

        let enemy_health: f32;
        // Choose enemy type based on current wave.
        let enemy_type = match wave_timer.wave {
            0..=2 => {
                enemy_health = 100.0;
                EnemyType::Zombie { speed: 50.0 }
            }
            3..=5 => {
                enemy_health = 75.0;
                EnemyType::Skeleton { speed: 75.0 }
            }
            _ => {
                enemy_health = 200.0;
                EnemyType::Boss { speed: 40.0 }
            }
        };

        // Select an initial sprite, default is "down'
        let sprite_path = match enemy_type {
            EnemyType::Boss { .. } => "zombie_apocalypse_tileset/organized_separated_sprites/Kid Zombie Animation Frames/Zombie-Tileset---_0430_Capa-431.png",
            EnemyType::Skeleton { .. } => "zombie_apocalypse_tileset/organized_separated_sprites/Kid Zombie Animation Frames/Zombie-Tileset---_0430_Capa-431.png",
            EnemyType::Zombie { .. } => "zombie_apocalypse_tileset/organized_separated_sprites/Kid Zombie Animation Frames/Zombie-Tileset---_0430_Capa-431.png",
        };

        // Use different sizes for enemy types
        let enemy_size = match enemy_type {
            EnemyType::Zombie { .. } => Vec2::new(32.0, 32.0),
            EnemyType::Skeleton { .. } => Vec2::new(32.0, 32.0),
            EnemyType::Boss { .. } => Vec2::new(64.0, 64.0),
        };

        // Set hitbox size based on enemy type
        let hitbox_size = match enemy_type {
            EnemyType::Zombie { .. } => Vec2::new(16.0, 20.0),
            EnemyType::Skeleton { .. } => Vec2::new(16.0, 20.0),
            EnemyType::Boss { .. } => Vec2::new(32.0, 40.0),
        };

        let texture_handle = asset_server.load(sprite_path);

        commands.spawn((
            Enemy,
            enemy_type,
            EnemyHealth(enemy_health),
            FacingDirection::Down,
            Transform {
                translation: spawn_position,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            Sprite {
                custom_size: Some(Vec2::new(enemy_size.x, enemy_size.y)),
                image: texture_handle,
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::round_rectangle(hitbox_size.x, hitbox_size.y, 5.0),
            Mass(5.0),
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            AnimationFrame(0),
            LinearVelocity(Vec2::ZERO), // Start with zero velocity
            LockedAxes::ROTATION_LOCKED,
            CollisionLayers::new(
                GameLayer::ENEMY,
                [
                    GameLayer::ENEMY,
                    GameLayer::PROJECTILE,
                    GameLayer::PLAYER,
                    GameLayer::ENEMY,
                ],
            ),
        ));
    }
}
/// Handle enemy movment and replace the sprite based on direction and animation state
pub fn enemy_movement_and_direction_system(
    mut enemy_query: Query<(
        &Transform,
        &mut LinearVelocity,
        &mut FacingDirection,
        &mut Sprite,
        &EnemyType,
        &AnimationFrame,
    )>,
    player_query: Query<&Transform, With<Player>>,
    animation_frames: Res<EnemyAnimationFrames>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        // Optionally set all enemy velocities to zero if player disappears?
        // for (_, mut vel, ..) in enemy_query.iter_mut() { vel.0 = Vec2::ZERO; }
        return;
    };
    let player_pos_2d = player_transform.translation.truncate(); // Use Vec2 for calculations

    // Iterate through enemies
    for (enemy_transform, mut velocity, mut facing, mut sprite, enemy_type, anim_frame) in
        enemy_query.iter_mut()
    {
        let enemy_pos_2d = enemy_transform.translation.truncate();

        // Calculate direction vector from enemy to player
        let direction = (player_pos_2d - enemy_pos_2d).normalize_or_zero();

        // Get speed based on enemy type
        let speed = match enemy_type {
            EnemyType::Zombie { speed, .. } => *speed,
            EnemyType::Skeleton { speed, .. } => *speed,
            EnemyType::Boss { speed, .. } => *speed,
        };

        // Set Linear Velocity
        let target_velocity = direction * speed;
        velocity.x = target_velocity.x;
        velocity.y = target_velocity.y;

        //Update Facing Direction and Sprite
        let new_direction = if direction.x.abs() > direction.y.abs() {
            // Horizontal movement is dominant
            if direction.x > 0.0 {
                FacingDirection::Right
            } else {
                FacingDirection::Left
            }
        } else {
            // Vertical movement is dominant (or exactly diagonal)
            if direction.y > 0.0 {
                FacingDirection::Up
            } else {
                FacingDirection::Down
            }
        };

        // Update facing direction component only if it changed
        if *facing != new_direction {
            *facing = new_direction;
        }

        // Load and set the correct animation frame sprite
        let frames_map = match enemy_type {
            EnemyType::Zombie { .. } => &animation_frames.zombie,
            EnemyType::Skeleton { .. } => &animation_frames.skeleton,
            EnemyType::Boss { .. } => &animation_frames.boss,
        };

        // Get frames for the current facing direction, handle potential missing entries gracefully
        if let Some(direction_frames) = frames_map.get(&new_direction) {
            // Determine animation frame index
            let frame_index = match anim_frame.0 {
                0 => 0,
                1 => 1,
                2 => 0, // Ping-pong effect
                3 => 2,
                _ => 0, // Default case
            };

            // Ensure frame index is valid for the loaded frames
            if frame_index < direction_frames.len() {
                // Update the sprite's texture handle
                sprite.image = direction_frames[frame_index].clone();
                // Flip sprite horizontally if facing Right
                sprite.flip_x = new_direction == FacingDirection::Right;
            } else {
                warn_once!(
                    "Animation frame index {} out of bounds for direction {:?}!",
                    frame_index,
                    new_direction
                );
            }
        } else {
            warn_once!(
                "No animation frames found for direction {:?}!",
                new_direction
            );
        }
    }
}

/// Timer system to manage enemy spawn intervals
pub fn wave_timer_system(mut wave_timer: ResMut<WaveTimer>, time: Res<Time>) {
    if wave_timer.timer.tick(time.delta()).just_finished() {
        wave_timer.wave += 1;
        info!("Wave {} started!", wave_timer.wave);
    }
}

pub fn prevent_enemy_overlap_system(
    mut query: Query<(&mut Transform, &LinearVelocity, Entity), With<Enemy>>,
) {
    let enemies: Vec<(Entity, Vec3, Vec2)> = query
        .iter()
        .map(|(transform, velocity, entity)| (entity, transform.translation, velocity.0))
        .collect();

    for i in 0..enemies.len() {
        for j in (i + 1)..enemies.len() {
            let (entity_a, pos_a, vel_a) = enemies[i];
            let (entity_b, pos_b, vel_b) = enemies[j];

            let distance = pos_a.distance(pos_b);
            let min_distance = 24.0; // Reduced from 32.0 to allow closer proximity

            if distance < min_distance {
                let direction = (pos_b - pos_a).normalize_or_zero();
                let overlap = min_distance - distance;

                // Only apply separation if enemies are moving towards each other
                let relative_velocity = vel_b - vel_a;
                let approaching = relative_velocity.dot(direction.truncate()) < 0.0;

                if approaching {
                    // Calculate separation force based on overlap
                    let separation = direction * (overlap * 0.5);

                    // Apply separation with damping
                    if let Ok((mut transform_a, _, _)) = query.get_mut(entity_a) {
                        transform_a.translation -= separation * 0.7;
                    }
                    if let Ok((mut transform_b, _, _)) = query.get_mut(entity_b) {
                        transform_b.translation += separation * 0.7;
                    }
                }
            }
        }
    }
}

/// Animation sustem for enemies
pub fn animate_enemy_system(
    mut query: Query<(&mut AnimationTimer, &mut AnimationFrame)>,
    time: Res<Time>,
) {
    for (mut timer, mut frame) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            frame.0 = match frame.0 {
                0 => 1,
                1 => 2,
                2 => 3,
                3 => 0,
                _ => 0,
            };
        }
    }
}

/// Preload all required sprites for the enemies so no flickering occurs when swapping the sprites
pub fn setup_enemy_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut zombie_frames: HashMap<FacingDirection, Vec<Handle<Image>>> = HashMap::new();
    let mut skeleton_frames: HashMap<FacingDirection, Vec<Handle<Image>>> = HashMap::new();
    let mut boss_frames: HashMap<FacingDirection, Vec<Handle<Image>>> = HashMap::new();

    // Load frames for each direction and enemy type
    for direction in [
        FacingDirection::Down,
        FacingDirection::Up,
        FacingDirection::Left,
    ] {
        let (zombie_base, skeleton_base, boss_base) = match direction {
            FacingDirection::Down => ("394", "430", "412"),
            FacingDirection::Up => ("400", "436", "418"),
            FacingDirection::Left => ("397", "433", "415"),
            _ => continue,
        };

        let enemy_types = [
            (
                "Skinny Walking Zombie Animation",
                zombie_base,
                &mut zombie_frames,
            ),
            (
                "Kid Zombie Animation Frames",
                skeleton_base,
                &mut skeleton_frames,
            ),
            (
                "Big Zombie Walking Animation Frames",
                boss_base,
                &mut boss_frames,
            ),
        ];

        for (folder, base, frames) in enemy_types {
            let frames_vec: Vec<Handle<Image>> = (0..3)
                .map(|i| {
                    let base = base.parse::<i32>().unwrap() + i;
                    let next = base + 1;
                    asset_server.load(format!(
                        "zombie_apocalypse_tileset/organized_separated_sprites/{}/Zombie-Tileset---_0{}_Capa-{}.png",
                        folder, base, next
                    ))
                })
                .collect();
            frames.insert(direction, frames_vec);
        }
    }

    // Right frames are just left frames but mirrored
    if let Some(left_frames) = zombie_frames.get(&FacingDirection::Left) {
        zombie_frames.insert(FacingDirection::Right, left_frames.clone());
    }
    if let Some(left_frames) = skeleton_frames.get(&FacingDirection::Left) {
        skeleton_frames.insert(FacingDirection::Right, left_frames.clone());
    }
    if let Some(left_frames) = boss_frames.get(&FacingDirection::Left) {
        boss_frames.insert(FacingDirection::Right, left_frames.clone());
    }

    commands.insert_resource(EnemyAnimationFrames {
        zombie: zombie_frames,
        skeleton: skeleton_frames,
        boss: boss_frames,
    });
}

pub fn kill_enemy_system(
    mut commands: Commands,
    mut kill_count: ResMut<EnemyKillCount>,
    mut enemy_query: Query<(&EnemyHealth, Entity)>,
) {
    for (enemy_health, enemy) in enemy_query.iter_mut() {
        if enemy_health.0 <= 0.0 {
            kill_count.0 += 1;
            info!("Despawn Entity {:?}", enemy);
            commands.entity(enemy).despawn();
        }
    }
}

pub fn debug_enemy_keybinds_system(
    input: Res<ButtonInput<KeyCode>>,
    mut enemy_health_query: Query<&mut EnemyHealth>,
) {
    if input.just_pressed(KeyCode::KeyQ) {
        info!("Pressed Q");
        if let Some(mut enemy_health) = enemy_health_query.iter_mut().next() {
            enemy_health.0 = -3.0;
            info!("Removed Health from Enemy");
        }
    }
}
