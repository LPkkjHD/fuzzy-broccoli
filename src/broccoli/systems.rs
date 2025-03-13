use std::f32::consts::PI;

use bevy::prelude::*;

use super::components::Broccoli;

pub fn spawn_broccoli(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("img/broccoli.png")),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.1, 0.1, 0.1)),
        Broccoli,
    ));
}

pub fn despawn_broccoli(mut commands: Commands, broccoli_query: Query<Entity, With<Broccoli>>) {
    commands
        .entity(broccoli_query.get_single().unwrap())
        .despawn();
}

pub const ROTATION_SPEED: f32 = PI / 5.0;
pub fn rotate_broccoli(mut broccoli_query: Query<&mut Transform, With<Broccoli>>, time: Res<Time>) {
    if let Ok(mut transform) = broccoli_query.get_single_mut() {
        transform.rotate_z(-ROTATION_SPEED * time.delta_secs());
    }
}
