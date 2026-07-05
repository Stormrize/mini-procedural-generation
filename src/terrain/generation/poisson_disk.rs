use bevy::prelude::Res;
use faster_poisson::Poisson2D;
use crate::terrain::generation::heightmap::PerlinPlaceholder;
use crate::terrain::generation::noise::fbm;
use crate::terrain::world_config::{WorldConfig, WorldShape};

pub fn poison_disk_sampling(config: &WorldConfig) -> Vec<[f64; 2]> {
    let WorldShape::Island {
        width_tiles,
        height_tiles,
        ..
    } = config.world_shape
    else {
        return Vec::new();
    };

    let vec: Vec<[f64; 2]> = Poisson2D::new()
        .dims([width_tiles as f64, height_tiles as f64])
        .radius(500.0)
        .seed(Some(config.seed as u64))
        .run()
        .into_iter()
        .map(|point| {
            [
                point[0] - width_tiles as f64 * 0.5,
                point[1] - height_tiles as f64 * 0.5,
            ]
        })
        .collect();

    vec
}

pub fn factions_points(config: &WorldConfig, perlin_placeholder: &Res<PerlinPlaceholder>) {

    let points = poison_disk_sampling(config);

    //
    // let biome_height = fbm(
    //     &perlin_placeholder.perlin_main,
    //
    // );

    for i in points {
    }
}