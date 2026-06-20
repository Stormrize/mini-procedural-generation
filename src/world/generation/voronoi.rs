use crate::world::math::math::min_max_tile;
use crate::world::world_config::{WorldConfig, WorldShape};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const DETAIL_POINT_COUNT: usize = 100;

#[derive(Default, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn random_points(config: &WorldConfig) -> Vec<Point> {
    let WorldShape::Island {
        center_tile,
        width_tiles,
        height_tiles,
        ..
    } = &config.world_shape
    else {
        return Vec::new();
    };

    let (min_tile_x, min_tile_y, max_tile_x, max_tile_y) = min_max_tile(
        *width_tiles as i32,
        *height_tiles as i32,
        *center_tile,
    );

    let mut rng = StdRng::seed_from_u64(config.seed as u64);
    let mut positions = Vec::with_capacity(DETAIL_POINT_COUNT);

    for _ in 0..DETAIL_POINT_COUNT {
        positions.push(Point {
            x: rng.random_range(min_tile_x..max_tile_x),
            y: rng.random_range(min_tile_y..max_tile_y),
        });
    }

    positions
}
