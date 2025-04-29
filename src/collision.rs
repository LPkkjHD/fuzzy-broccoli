use avian2d::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    DEFAULT,
    PLAYER,
    ENEMY,
    PROJECTILE,
}
