use crate::terrain::world_config::CHUNK_SIZE;
use bevy::math::IVec2;

#[inline]
pub fn min_max_chunk(width_tiles: i32, height_tiles: i32, center_tile: IVec2, margin: i32) -> (IVec2, IVec2) {
    let half_width = (width_tiles as f32 * 0.5).max(0.5) as i32;
    let half_height = (height_tiles as f32 * 0.5).max(0.5) as i32;
    let min_tile_x = center_tile.x - half_width;
    let min_tile_y = center_tile.y - half_height;
    let max_tile_x_exclusive =
        (center_tile.x + half_width).max(min_tile_x + 1);
    let max_tile_y_exclusive =
        (center_tile.y + half_height).max(min_tile_y + 1);

    let chunk_size = CHUNK_SIZE as i32;
    let min_chunk = IVec2::new(
        min_tile_x.div_euclid(chunk_size) - margin,
        min_tile_y.div_euclid(chunk_size) - margin,
    );
    let max_chunk = IVec2::new(
        (max_tile_x_exclusive - 1).div_euclid(chunk_size) + margin,
        (max_tile_y_exclusive - 1).div_euclid(chunk_size) + margin,
    );
    (min_chunk, max_chunk)
}

#[inline]
pub fn min_max_tile(width_tiles: i32, height_tiles: i32, center_tile: IVec2) -> (i32, i32, i32, i32) {
    let half_width = (width_tiles as f32 * 0.5).max(0.5) as i32;
    let half_height = (height_tiles as f32 * 0.5).max(0.5) as i32;
    let min_tile_x = center_tile.x - half_width;
    let min_tile_y = center_tile.y - half_height;
    let max_tile_x_exclusive =
        (center_tile.x + half_width).max(min_tile_x + 1);
    let max_tile_y_exclusive =
        (center_tile.y + half_height).max(min_tile_y + 1);

    (
        min_tile_x, min_tile_y, max_tile_x_exclusive, max_tile_y_exclusive
    )
}