use crate::map_genreation::components::{ResetTerrainEvent, Tile, TileComponent};
use crate::map_genreation::config::{CHUNK_H, CHUNK_W, SPRITE_SCALE_FACTOR, SPRITE_SHEET_PATH};
use crate::map_genreation::resources::{CurrentChunks, GenerationSeed, GroundTiles};
use crate::map_genreation::util::{center_to_top_left, grid_to_chunk, grid_to_world};
use crate::player::components::{CurrentPlayerChunkPos, PlayerChunkUpdateEvent};
use bevy::asset::{AssetServer, Assets};
use bevy::math::{UVec2, Vec3};
use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, ResMut, Sprite, TextureAtlas, TextureAtlasLayout, Transform, With};
use bevy::reflect::Array;
use bevy::utils::HashSet;
use noise::{NoiseFn, Perlin};
use rand::Rng;

#[allow(clippy::too_many_arguments)]
pub fn handle_terrain_reset_event(
    mut commands: Commands,
    mut reader: EventReader<ResetTerrainEvent>,
    mut ev_writer: EventWriter<PlayerChunkUpdateEvent>,
    player_pos: Res<CurrentPlayerChunkPos>,
    mut chunks: ResMut<CurrentChunks>,
    mut ground_tiles: ResMut<GroundTiles>,
    mut seed: ResMut<GenerationSeed>,
    tile_q: Query<Entity, With<TileComponent>>,
) {
    if reader.is_empty() {
        return;
    }

    reader.clear();
    for t in tile_q.iter() {
        commands.entity(t).despawn();
    }

    // Reset res
    chunks.0.clear();
    ground_tiles.0.clear();

    let mut rng = rand::thread_rng();
    seed.0 = rng.gen();

    // Trigger world re-generation
    let (x, y) = player_pos.0;
    ev_writer.send(PlayerChunkUpdateEvent((x, y)));
}

pub fn clean_ground_tiles(
    player_pos: Res<CurrentPlayerChunkPos>,
    mut ground_tiles: ResMut<GroundTiles>,
) {
    let (x, y) = player_pos.0;
    ground_tiles.0.retain(|pos| {
        let (px, py) = grid_to_chunk(pos.0 as f32, pos.1 as f32);
        px.abs_diff(x) <= 1 || py.abs_diff(y) <= 1
    });
}

pub fn despawn_chunks(
    mut commands: Commands,
    mut current_chunks: ResMut<CurrentChunks>,
    player_pos: Res<CurrentPlayerChunkPos>,
) {
    let mut keys_to_remove = Vec::new();
    let (x, y) = player_pos.0;

    for ((cx, cy), entities) in current_chunks.0.iter() {
        if cx.abs_diff(x) <= 1 && cy.abs_diff(y) <= 1 {
            continue;
        }

        for e in entities.iter() {
            commands.entity(*e).despawn();
        }
        keys_to_remove.push((*cx, *cy));
    }

    for (cx, cy) in keys_to_remove {
        current_chunks.0.remove(&(cx, cy));
    }
}

pub fn handle_player_chunk_update_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    seed: Res<GenerationSeed>,
    mut current_chunks: ResMut<CurrentChunks>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut ev_chunk_update: EventReader<PlayerChunkUpdateEvent>,
    mut ground_tiles: ResMut<GroundTiles>,
) {
    if ev_chunk_update.is_empty() {
        return;
    }

    let texture_handle = asset_server.load(SPRITE_SHEET_PATH);
    let texture_atlas_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        28,
        13,
        Some(UVec2 { x: 1, y: 3 }),
        None,
    );
    let texture_atlas_handle = texture_atlas_layouts.add(texture_atlas_layout);

    for new_chunk_pos in ev_chunk_update.read() {
        let (x, y) = new_chunk_pos.0;

        let chunk_nei = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, 1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (0, 0),
        ];
        let mut tiles = HashSet::new();
        let mut ground_map = HashSet::new();

        for (i, j) in chunk_nei {
            let (x, y) = (x + i, y + j);
            if current_chunks.0.contains_key(&(x, y)) {
                continue;
            }

            let start = (x * CHUNK_W as i32, y * CHUNK_H as i32);
            let (chunk_tiles, chunk_ground_map) = gen_chunk(seed.0, (start.0, start.1));
            tiles.extend(chunk_tiles);
            ground_map.extend(chunk_ground_map);
        }

        let mut updated_ground_map = HashSet::new();
        for (x, y) in ground_map.iter() {
            let (num_nei, tile) = process_tile((*x, *y), &ground_map);
            if num_nei == 1 {
                continue;
            }

            updated_ground_map.insert((*x, *y));
            tiles.insert(Tile::new((*x, *y), tile, 0));
        }
        ground_tiles.0.extend(updated_ground_map);

        for t in tiles.iter() {
            let (cx, cy) = grid_to_chunk(t.pos.0 as f32, t.pos.1 as f32);
            let (x, y) = grid_to_world(t.pos.0 as f32, t.pos.1 as f32);
            let (x, y) = center_to_top_left(x, y);


            let e = commands
                .spawn((
                    Sprite::from_atlas_image(
                        texture_handle.clone(),
                        TextureAtlas {
                            layout: texture_atlas_handle.clone(),
                            index: t.sprite,
                        },
                    ),
                    Transform::from_xyz(x, y, t.z_index as f32)
                        .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR * 1.01f32)),
                    TileComponent,
                ))
                .id();

            current_chunks
                .0
                .entry((cx, cy))
                .or_insert_with(Vec::new)
                .push(e);
        }
    }
}

pub fn gen_chunk(gen_seed: u32, start: (i32, i32)) -> (HashSet<Tile>, HashSet<(i32, i32)>) {
    let mut rng = rand::thread_rng();
    let noise = Perlin::new(gen_seed);

    let mut tiles = HashSet::new();
    let mut ground_map = HashSet::new();
    let end = (start.0 + CHUNK_W as i32, start.1 + CHUNK_H as i32);

    for x in start.0 - 1..end.0 + 1 {
        for y in start.1 - 1..end.1 + 1 {
            let noise_val1 = noise.get([x as f64 / 100.5, y as f64 / 100.5]);
            let noise_val2 = noise.get([x as f64 / 53.5, y as f64 / 53.5]);
            let noise_val3 = noise.get([x as f64 / 43.5, y as f64 / 43.5]);
            let noise_val4 = noise.get([x as f64 / 23.5, y as f64 / 23.5]);
            let noise_val = (noise_val1 + noise_val2 + noise_val3 + noise_val4) / 4.0;

            if noise_val > 0.0 {
                ground_map.insert((x, y));
            }
        }
    }

    for x in start.0 - 1..end.0 + 1 {
        for y in start.1 - 1..end.1 + 1 {
            if !ground_map.contains(&(x, y)) {
                // Check neighbors to determine shore tiles
                let shore_pattern = [
                    ground_map.contains(&(x, y - 1)), // North
                    ground_map.contains(&(x + 1, y)), // East
                    ground_map.contains(&(x, y + 1)), // South
                    ground_map.contains(&(x - 1, y)), // West
                ];

                let water_tile = match shore_pattern {
                    // Detect if the Water tile is a top shore
                    [true, false, false, false] => 338,
                    [true, false, false, true] => 338,
                    [true, true, false, false] => 338,
                    [true, true, false, true] => 338,
                    _ => match rng.gen_range(0..3) {
                        0 => 286,
                        1 => 314,
                        _ => 342,
                    },
                };

                tiles.insert(Tile::new((x, y), water_tile, 1));
                continue;
            }

            let noise_val1 = noise.get([x as f64 / 100.5, y as f64 / 100.5]);
            let noise_val2 = noise.get([x as f64 / 53.5, y as f64 / 53.5]);
            let noise_val3 = noise.get([x as f64 / 43.5, y as f64 / 43.5]);
            let noise_val4 = noise.get([x as f64 / 23.5, y as f64 / 23.5]);
            let noise_val = (noise_val1 + noise_val2 + noise_val3 + noise_val4) / 4.0;
            let chance = rng.gen_range(0.0..1.0);

            if noise_val < 0.05 {
                continue;
            }

            // Dense Forest
            if (noise_val > 0.5 || noise_val3 > 0.98) && chance > 0.2 {
                tiles.insert(Tile::new((x, y), 298, 5));
                continue;
            }

            // Patch Forest
            if noise_val3 > 0.5 && noise_val < 0.5 && chance > 0.4 {
                let chance2 = rng.gen_range(0.0..1.0);
                let tile = if chance2 > 0.7 {
                    let chance3 = rng.gen_range(0.0..1.0);
                    if chance3 > 0.6 {
                        rng.gen_range(320..=321)
                    } else if chance3 > 0.1 {
                        rng.gen_range(292..=294)
                    } else {
                        252
                    }
                } else {
                    298
                };
                tiles.insert(Tile::new((x, y), tile, 3));
                continue;
            }

            // Sparse Forest
            if noise_val4 > 0.4 && noise_val < 0.5 && noise_val3 < 0.5 && chance > 0.9 {
                let chance = rng.gen_range(0.0..1.0);
                let tile = if chance > 0.78 { 298 } else { rng.gen_range(320..=321) };
                tiles.insert(Tile::new((x, y), tile, 3));
                continue;
            }

            // Cans
            if noise_val > 0.3 && noise_val < 0.5 && noise_val3 < 0.5 && chance > 0.98 {
                let tile = rng.gen_range(280..=284);
                tiles.insert(Tile::new((x, y), tile, 1));
                continue;
            }

            // Gas Station or dead body or junk
            if noise_val > 0.1 && noise_val < 0.3 && noise_val3 < 0.4 && chance > 0.8 {
                let chance2 = rng.gen_range(0.0..1.0);
                if chance2 > 0.98 {
                    for x_stamp in 0..6 {
                        for y_stamp in 0..3 {
                            tiles.insert(Tile::new(
                                (x + x_stamp, y + y_stamp),
                                (28 * (2 + y_stamp) + 13 + x_stamp) as usize,
                                5,
                            ));
                        }
                    }
                } else if noise_val > 0.2 && noise_val < 0.3 && noise_val3 < 0.3 && chance > 0.9 {
                    let tile = rng.gen_range(41..=44);
                    tiles.insert(Tile::new((x, y), tile, 5));
                }
            }
        }
    }

    (tiles, ground_map)
}
pub fn process_tile((x, y): (i32, i32), occupied: &HashSet<(i32, i32)>) -> (i32, usize) {
    let nei_options = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut nei = [1, 1, 1, 1];
    let mut nei_count = 4;
    let mut rng = rand::thread_rng();
    for idx in 0..nei_options.len() {
        let (i, j) = nei_options[idx];
        if !occupied.contains(&(x + i, y + j)) {
            nei[idx] = 0;
            nei_count -= 1;
        }
    }
    /*
    Danke Rust
    268 |         for (idx, (i, j)) in nei_options.iter().enumerate() {
    |                   ^^^^^^     ------------------------------ this is an iterator with items of type `(usize, &dyn PartialReflect)`
    |                   |
    |                   expected `dyn PartialReflect`, found `(_, _)`
    |
    = note: expected trait object `dyn PartialReflect`
                      found tuple `(_, _)`
    = help: `(_, _)` implements `PartialReflect` so you could box the found value and coerce it to the trait object `Box<dyn PartialReflect>`, you will have to change the expected type as well
     */
    // for (idx, (i, j)) in nei_options.iter().enumerate() {
    //     if !occupied.contains(&(x + i, y + j)) {
    //         nei[idx] = 0;
    //         nei_count -= 1;
    //     }
    // }

    //select random ground tile
    let tile = match rng.gen_range(0..3) {
        0 => 291,
        1 => 319,
        _ => 347,
    };

    (nei_count, tile)
}
