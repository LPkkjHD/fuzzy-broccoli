use avian2d::prelude::{
    Collider, Collision, CollisionLayers, LinearVelocity, LockedAxes, RigidBody,
    SweptCcd,
};
use bevy::prelude::*;

use crate::{
    collision::GameLayer,
    enemy::components::{Enemy, EnemyHealth},
    player::components::Player,
};

use super::{
    components::{Damage, Pistol, Projectile},
    events::WeaponFiredEvent,
};

pub fn spawn_pistol(
    mut commands: Commands,
    player_query: Query<Entity, Added<Player>>,
    asset_server: Res<AssetServer>,
) {
    let pistol_component = (
        Pistol,
        Damage(20.0),
        Transform{ translation: Vec3::new(-6.0, -4.0, 1.0), ..default() },
        Sprite {
            image: asset_server.load("zombie_apocalypse_tileset/organized_separated_sprites/Pickable Items and Weapons/Zombie-Tileset---_0333_Capa-334.png"),
            ..default()
        },
    );
    let pistol_component_id = commands.spawn(pistol_component).id();

    if let Ok(player) = player_query.get_single() {
        commands.entity(player).add_child(pistol_component_id);
    }
}

const PROJECTILE_SPEED: f32 = 800.0;
pub fn spawn_projectile_component(
    mut commands: Commands,
    weapon_query: Query<&GlobalTransform, With<Pistol>>,
    asset_server: Res<AssetServer>,
    mut weapon_fired_event: EventReader<WeaponFiredEvent>,
) {
    if let Ok(transform) = weapon_query.get_single() {
        let weapon_position = transform.translation();
        let weapon_world_position = weapon_position.truncate();
        for event in weapon_fired_event.read() {
            let target_world_position = event.0;
            let direction_vector = target_world_position - weapon_world_position;
            let direction_normalized = direction_vector.normalize();
            let angle_radians = direction_normalized.y.atan2(direction_normalized.x);
            let rotation = Quat::from_rotation_z(angle_radians);
            let projectile_component = (
        Projectile,
        Sprite {
            image: asset_server.load("zombie_apocalypse_tileset/organized_separated_sprites/Pistol Shooting Animation Frames/Zombie-Tileset---_0372_Capa-373.png"),
            ..default()

        },
                Transform::from_translation(weapon_position).with_rotation(rotation),
        RigidBody::Dynamic,
        Collider::round_rectangle(4.0, 1.0, 0.5),
                LockedAxes::ROTATION_LOCKED,
        LinearVelocity(direction_normalized*PROJECTILE_SPEED),
                CollisionLayers::new(GameLayer::PROJECTILE, [GameLayer::ENEMY]),
                SweptCcd::default(),
);
            commands.spawn(projectile_component);
        }
    }
}

pub fn projectile_enemy_collision_damage_system(
    mut commands: Commands,
    projectile_query: Query<Entity, With<Projectile>>,
    weapon_damage_query: Query<&Damage, With<Pistol>>,
    mut enemy_query: Query<&mut EnemyHealth, With<Enemy>>,
    mut collision_events: EventReader<Collision>,
) {
    let damage = weapon_damage_query.single().0;
    for collision in collision_events.read() {
        if let Ok(projectile) = projectile_query.get(collision.0.entity1) {
            if let Ok(mut enemy_health) = enemy_query.get_mut(collision.0.entity2) {
                enemy_health.0 -= damage;
                commands.entity(projectile).despawn();
            }
        }
        if let Ok(projectile) = projectile_query.get(collision.0.entity2) {
            if let Ok(mut enemy_health) = enemy_query.get_mut(collision.0.entity1) {
                enemy_health.0 -= damage;
                commands.entity(projectile).despawn();
            }
        }
    }
}
