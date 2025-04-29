use bevy::prelude::*;

use crate::player::{components::Player, resources::PlayerFacingDirection};

use super::components::{AttacksPerSecond, Damage, Pistol};

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
            image: asset_server.load("zombie_apocalypse_tileset/organized_separated_sprites/Pickable Items and Weapons/Zombie-Tileset---_0336_Capa-337.png"),
            ..default()
        },
    );
    let pistol_component_id = commands.spawn(pistol_component).id();

    if let Ok(player) = player_query.get_single() {
        commands.entity(player).add_child(pistol_component_id);
    }
}

pub fn despawn_pistol(mut commands: Commands, pistol_query: Query<Entity, With<Pistol>>) {
    if let Ok(entity) = pistol_query.get_single() {
        commands.entity(entity).despawn();
    }
}
