use avian2d::prelude::{Collider, LinearVelocity, LockedAxes, RigidBody};
use bevy::prelude::*;

use crate::player::components::Player;

use super::{
    components::{AttacksPerSecond, Damage, Pistol, Projectile},
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
        AttacksPerSecond(3.0),
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
            let projectile_component = (
        Projectile,
        Sprite {
            image: asset_server.load("zombie_apocalypse_tileset/organized_separated_sprites/Pistol Shooting Animation Frames/Zombie-Tileset---_0372_Capa-373.png"),
            ..default()

        },
                Transform::from_translation(weapon_position),
        RigidBody::Dynamic,
        Collider::round_rectangle(4.0, 1.0, 0.5),
        LinearVelocity(direction_normalized*PROJECTILE_SPEED),

);
            commands.spawn(projectile_component);
            // event.0
        }
    }
}

pub fn despawn_projectile_component(
    mut commands: Commands,
    mut projectile_query: Query<Entity, With<Projectile>>,
) {
    projectile_query
        .iter_mut()
        .for_each(|projectile| commands.entity(projectile).despawn());
}

pub fn despawn_pistol(mut commands: Commands, pistol_query: Query<Entity, With<Pistol>>) {
    if let Ok(entity) = pistol_query.get_single() {
        commands.entity(entity).despawn();
    }
}
