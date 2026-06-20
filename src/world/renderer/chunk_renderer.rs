//! Рендер чанков.
//!
//! Вместо `Entity` на каждый tile создаётся до 8 mesh-entity на чанк:
//! по одному merged mesh на каждый биом, который реально есть в чанке.

use bevy::mesh::Indices;
use bevy::{asset::RenderAssetUsages, render::render_resource::PrimitiveTopology};
use bevy::prelude::{default, Assets, Commands, Component, Entity, Handle, IVec2, Mesh, Mesh3d, MeshMaterial3d, Resource, StandardMaterial, Transform, Vec3};
use crate::world::chunk::Chunk;
use crate::world::world_config::{WorldConfig, CHUNK_SIZE};
use crate::world::world_enums::biome::Biome;

#[derive(Component)]
#[allow(dead_code)]
pub struct ChunkMesh {
    pub coord: IVec2,
    pub biome: Biome,
}

/// Shared материалы, создаваемые один раз на весь мир.
#[derive(Resource)]
pub struct TileAssets {
    /// Один материал на каждый биом (индекс = Biome::index()).
    pub materials: [Handle<StandardMaterial>; 8],
}

pub struct SpawnedChunkMeshes {
    pub entities: Vec<Entity>,
    pub mesh_handles: Vec<Handle<Mesh>>,
}

impl TileAssets {
    pub fn build(materials: &mut Assets<StandardMaterial>) -> Self {
        let mat_handles: Vec<Handle<StandardMaterial>> = Biome::ALL
            .iter()
            .map(|&biome| {
                materials.add(StandardMaterial {
                    base_color: biome.color(),
                    perceptual_roughness: match biome {
                        Biome::DeepWater | Biome::ShallowWater => 0.08,
                        Biome::Peak => 0.6,
                        _ => 0.9,
                    },
                    metallic: 0.0,
                    ..default()
                })
            })
            .collect();

        Self {
            materials: mat_handles
                .try_into()
                .expect("Biome::ALL должен содержать ровно 8 элементов"),
        }
    }

    #[inline]
    pub fn material_for(&self, biome: Biome) -> Handle<StandardMaterial> {
        self.materials[biome.index()].clone()
    }
}

pub fn spawn_chunk_meshes(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    chunk: &Chunk,
    config: &WorldConfig,
    assets: &TileAssets,
) -> SpawnedChunkMeshes {
    let chunk_origin = Vec3::new(
        chunk.coord.x as f32 * config.chunk_world_size(),
        0.0,
        chunk.coord.y as f32 * config.chunk_world_size(),
    );

    let mut spawned = SpawnedChunkMeshes {
        entities: Vec::with_capacity(Biome::ALL.len()),
        mesh_handles: Vec::with_capacity(Biome::ALL.len()),
    };

    for biome in Biome::ALL {
        let Some(mesh) = build_biome_surface_mesh(chunk, biome, config) else {
            continue;
        };

        let mesh_handle = meshes.add(mesh);
        let entity = commands
            .spawn((
                ChunkMesh {
                    coord: chunk.coord,
                    biome,
                },
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(assets.material_for(biome)),
                Transform::from_translation(chunk_origin),
            ))
            .id();

        spawned.entities.push(entity);
        spawned.mesh_handles.push(mesh_handle);
    }

    spawned
}

pub fn despawn_chunk_meshes(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    spawned: SpawnedChunkMeshes,
) {
    for entity in spawned.entities {
        commands.entity(entity).despawn();
    }
    for mesh_handle in spawned.mesh_handles {
        meshes.remove(mesh_handle.id());
    }
}

fn build_biome_surface_mesh(chunk: &Chunk, biome: Biome, config: &WorldConfig) -> Option<Mesh> {
    let tile_size = config.tile_size;
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for (col, row, tile) in chunk.iter_tiles() {
        if tile.biome != biome {
            continue;
        }

        let base = positions.len() as u32;
        let x0 = col as f32 * tile_size;
        let x1 = x0 + tile_size;
        let z0 = row as f32 * tile_size;
        let z1 = z0 + tile_size;
        const INV_128: f32 = 1.0 / 128.0;
        const INV_U8_MAX: f32 = 1.0 / 255.0;
        const HEIGHT_SCALE: f32 = 0.04;

        let biome_height = f32::from(tile.biome.visual_height()) * INV_128;
        let height_offset = f32::from(tile.height) * INV_U8_MAX * HEIGHT_SCALE;

        let y = biome_height + height_offset;
        positions.extend_from_slice(&[[x0, y, z0], [x0, y, z1], [x1, y, z1], [x1, y, z0]]);
        normals.extend_from_slice(&[[0.0, 1.0, 0.0]; 4]);
        uvs.extend_from_slice(&[[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    if positions.is_empty() {
        return None;
    }

    debug_assert!(positions.len() <= CHUNK_SIZE * CHUNK_SIZE * 4);

    Some(
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices)),
    )
}
