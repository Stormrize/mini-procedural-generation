//! Рендеринг тайлов.
//!
//! Каждый тайл — это 3D-бокс (Cuboid), высота которого зависит от биома.
//!
//! Оптимизация: один shared mesh (unit cube 1×1×1) + один материал на биом.
//! Размер тайла задаётся через Transform::scale, а не отдельными мешами.
//! Итого: 8 материалов + 1 меш для любого размера карты.
//!
//! ─────────────────────────────────────────────────────────────────────────
//! КАК ДОБАВИТЬ СВОИ ТЕКСТУРЫ:
//!   В функции `TileAssets::build` найди строку `base_color: b.color()` и
//!   замени блок на:
//!
//!   StandardMaterial {
//!       base_color_texture: Some(asset_server.load("textures/grass.png")),
//!       ..default()
//!   }
//!
//!   Передай AssetServer в функцию как дополнительный параметр.
//! ─────────────────────────────────────────────────────────────────────────

use bevy::prelude::*;

use super::biome::Biome;
use super::config::WorldConfig;
use super::map::MapData;

// ─────────────────────────────────────────────────────────────────────────────
// Компоненты
// ─────────────────────────────────────────────────────────────────────────────

/// Маркер на каждом тайле — для быстрого despawn всей карты.
#[derive(Component)]
pub struct TileMarker;

// ─────────────────────────────────────────────────────────────────────────────
// Общие ассеты
// ─────────────────────────────────────────────────────────────────────────────

/// Shared меш + материалы, создаваемые один раз на всю карту.
pub struct TileAssets {
    /// Unit-cube 1×1×1, используется для всех тайлов через Transform::scale.
    pub mesh: Handle<Mesh>,
    /// Один материал на каждый биом (индекс = Biome::index()).
    pub materials: [Handle<StandardMaterial>; 8],
}

impl TileAssets {
    /// Создать меш и материалы для всех биомов.
    pub fn build(
        meshes:    &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Self {
        // Один меш на всю карту — масштабируем через Transform.
        let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

        // Создаём один материал на каждый биом по порядку Biome::ALL.
        let mat_handles: Vec<Handle<StandardMaterial>> = Biome::ALL
            .iter()
            .map(|&biome| {
                materials.add(StandardMaterial {
                    // ── Плейсхолдер-цвет ─────────────────────────────────
                    // TODO: замени на base_color_texture для своих текстур
                    base_color: biome.color(),
                    // ─────────────────────────────────────────────────────
                    perceptual_roughness: match biome {
                        Biome::DeepWater | Biome::ShallowWater => 0.05, // вода — более гладкая
                        Biome::Peak => 0.6,
                        _           => 0.9,
                    },
                    metallic: 0.0,
                    ..default()
                })
            })
            .collect();

        // Vec → [T; 8] — безопасно, т.к. Biome::ALL всегда 8 элементов.
        let materials = mat_handles
            .try_into()
            .expect("Biome::ALL должен содержать ровно 8 элементов");

        Self { mesh, materials }
    }

    /// Получить материал для конкретного биома.
    #[inline]
    pub fn material_for(&self, biome: Biome) -> Handle<StandardMaterial> {
        self.materials[biome.index()].clone()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Spawn / Despawn
// ─────────────────────────────────────────────────────────────────────────────

`/// Заспавнить по одному entity-боксу на каждый тайл карты.
`pub fn spawn_tiles(
    commands:  &mut Commands,
    map_data:  &MapData,
    config:    &WorldConfig,
    assets:    &TileAssets,
) {
    let ts = config.tile_size; // размер тайла в мировых единицах

    for (col, row, tile) in map_data.iter_tiles() {
        let vh = tile.biome.visual_height(); // высота бокса

        // Мировые координаты тайла:
        //   X = колонка * tile_size
        //   Y = vh/2  (бокс стоит на земле — его центр на полувысоте)
        //   Z = строка * tile_size  (ось «глубины» в изометрии)
        let position = Vec3::new(
            col as f32 * ts,
            vh * 0.5,
            row as f32 * ts,
        );

        commands.spawn((
            TileMarker,
            Mesh3d(assets.mesh.clone()),
            MeshMaterial3d(assets.material_for(tile.biome)),
            Transform {
                translation: position,
                // Unit-cube масштабируем до нужного размера.
                // X и Z = tile_size (ширина/глубина), Y = visual_height.
                scale: Vec3::new(ts, vh, ts),
                ..default()
            },
        ));
    }
}

/// Удалить все тайловые entity.
pub fn despawn_tiles(
    commands:      &mut Commands,
    tile_entities: &Query<Entity, With<TileMarker>>,
) {
    for entity in tile_entities.iter() {
        commands.entity(entity).despawn();
    }
}
