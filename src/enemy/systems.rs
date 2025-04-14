use bevy::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;
use avian2d::collision::Collider;
use avian2d::prelude::{Mass, RigidBody};
use std::collections::HashMap;
use super::components::*;
use crate::player::components::Player;


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
            0.0,
        );

        // Choose enemy type based on current wave.
        let enemy_type = match wave_timer.wave {
            0..=2 => EnemyType::Zombie {
                speed: 50.0,
                health: 100.0,
            },
            3..=5 => EnemyType::Skeleton {
                speed: 75.0,
                health: 75.0,
            },
            _ => EnemyType::Boss {
                speed: 40.0,
                health: 200.0,
            },
        };

        // Select an initial sprite, default is "down'
        let sprite_path = match enemy_type {
            EnemyType::Boss { .. } => "zombie_apocalypse_tileset/organized_separated_sprites/Kid Zombie Animation Frames/Zombie-Tileset---_0430_Capa-431.png",
            EnemyType::Skeleton { .. } => "zombie_apocalypse_tileset/organized_separated_sprites/Kid Zombie Animation Frames/Zombie-Tileset---_0430_Capa-431.png",
            EnemyType::Zombie { .. } => "zombie_apocalypse_tileset/organized_separated_sprites/Kid Zombie Animation Frames/Zombie-Tileset---_0430_Capa-431.png",
        };

        let texture_handle = asset_server.load(sprite_path);

        commands.spawn((
            Enemy,
            enemy_type,
            FacingDirection::Down,
            Transform {
                translation: spawn_position,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            Sprite {
                custom_size: Some(Vec2::new(32.0, 32.0)),
                image: texture_handle,
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::capsule(16.0, 16.0),
            Mass(5.0),
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            AnimationFrame(0),
        ));
    }
}
/// Handle enemy movment and replace the sprite based on direction and animation state
pub fn enemy_movement_and_direction_system(
    mut param_set: ParamSet<(
        Query<(
            &mut Transform,
            &mut FacingDirection,
            &mut Sprite,
            &EnemyType,
            &AnimationFrame,
        )>,
        Query<&Transform, With<Player>>,
    )>,
    time: Res<Time>,
    animation_frames: Res<EnemyAnimationFrames>,
) {
    let player_pos = match param_set.p1().get_single() {
        Ok(transform) => transform.translation,
        Err(_) => return,
    };

    for (mut transform, mut facing, mut sprite, enemy_type, anim_frame) in param_set.p0().iter_mut() {
        let direction = (player_pos - transform.translation).normalize();
        let speed = match enemy_type {
            EnemyType::Zombie { speed, .. } => *speed,
            EnemyType::Skeleton { speed, .. } => *speed,
            EnemyType::Boss { speed, .. } => *speed,
        };
        transform.translation += direction * speed * time.delta_secs();
        transform.rotation = Quat::IDENTITY;

        let new_direction = if direction.x.abs() > direction.y.abs() {
            if direction.x > 0.0 {
                sprite.flip_x = true;
                FacingDirection::Right
            } else {
                sprite.flip_x = false;
                FacingDirection::Left
            }
        } else {
            sprite.flip_x = false;
            if direction.y > 0.0 {
                FacingDirection::Up
            } else {
                FacingDirection::Down
            }
        };

        *facing = new_direction;

        let frames = match enemy_type {
            EnemyType::Zombie { .. } => &animation_frames.zombie,
            EnemyType::Skeleton { .. } => &animation_frames.skeleton,
            EnemyType::Boss { .. } => &animation_frames.boss,
        };

        let direction_frames = &frames[&new_direction];
        let frame_index = match anim_frame.0 {
            0 => 0,
            1 => 1,
            2 => 0,
            3 => 2,
            _ => 0
        };

        sprite.image = direction_frames[frame_index].clone();
        sprite.flip_x = new_direction == FacingDirection::Right;
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
    for direction in [FacingDirection::Down, FacingDirection::Up, FacingDirection::Left] {
        let (zombie_base, skeleton_base, boss_base) = match direction {
            FacingDirection::Down => ("394", "430", "412"),
            FacingDirection::Up => ("400", "436", "418"),
            FacingDirection::Left => ("397", "433", "415"),
            _ => continue,
        };

        let enemy_types = [
            ("Skinny Walking Zombie Animation", zombie_base, &mut zombie_frames),
            ("Kid Zombie Animation Frames", skeleton_base, &mut skeleton_frames),
            ("Big Zombie Walking Animation Frames", boss_base, &mut boss_frames),
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
