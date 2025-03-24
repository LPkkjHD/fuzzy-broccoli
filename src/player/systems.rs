use bevy::prelude::*;

use super::components::Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Sprite{
            image: asset_server.load("zombie_apocalypse_tileset/organized_separated_sprites/Player Character Walking Animation Frames/Zombie-Tileset---_0476_Capa-477.png"),
            ..default()
        },
        Transform::from_xyz(100.0, 100.0, 0.0),
    ));
}
