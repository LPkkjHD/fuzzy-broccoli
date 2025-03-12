use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_svg::prelude::{Origin, Svg2d};

fn main() {
    let mut app = App::new();
    // Add plugins
    app.add_plugins((DefaultPlugins, bevy_svg::prelude::SvgPlugin));

    app.add_systems(Startup, (setup_camera, empty_system, spawn_broccoli))
        .add_systems(Update, rotate_broccoli);

    // run the app
    app.run();
}

// This is an empty system to disable  the bevy app shutdown Immedatly.
pub fn empty_system() {}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d {});
}

#[derive(Component)]
pub struct Broccoli;

pub fn spawn_broccoli(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("img/broccoli.png")),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.1, 0.1, 0.1)),
        Origin::Center,
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
