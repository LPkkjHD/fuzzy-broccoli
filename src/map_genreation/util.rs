use crate::map_genreation::config::*;

pub fn grid_to_world(x: f32, y: f32) -> (f32, f32) {
    (
        x * TILE_W as f32 * SPRITE_SCALE_FACTOR,
        y * TILE_H as f32 * SPRITE_SCALE_FACTOR,
    )
}

pub fn world_to_grid(x: f32, y: f32) -> (f32, f32) {
    (
        (x / (TILE_W as f32 * SPRITE_SCALE_FACTOR)).floor(),
        (y / (TILE_H as f32 * SPRITE_SCALE_FACTOR)).floor(),
    )
}

pub fn center_to_top_left_grid(x: f32, y: f32) -> (f32, f32) {
    let x_center = x + GRID_COLS as f32 / 2.0;
    let y_center = GRID_ROWS as f32 / 2.0 - y;
    (x_center, y_center)
}

pub fn center_to_top_left(x: f32, y: f32) -> (f32, f32) {
    let x_center = x - (GRID_W as f32 * SPRITE_SCALE_FACTOR) / 2.0;
    let y_center = (GRID_H as f32 * SPRITE_SCALE_FACTOR) / 2.0 - y;
    (x_center, y_center)
}

pub fn grid_to_chunk(x: f32, y: f32) -> (i32, i32) {
    let (x, y) = (x / CHUNK_W as f32, y / CHUNK_H as f32);
    (x.floor() as i32, y.floor() as i32)
}
