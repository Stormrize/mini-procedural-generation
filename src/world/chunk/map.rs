// //! Ресурс данных карты.
// //!
// //! Чисто данные — никакого рендеринга или ECS здесь нет.
// //! `MapData` читается рендерером и системами геймплея.
// 
// use bevy::prelude::*;
// 
// use super::biome::Biome;
// 
// /// Данные одного тайла после генерации.
// #[derive(Clone, Debug)]
// pub struct TileData {
//     /// Нормализованная высота (0.0–1.0).
//     pub height: f32,
//     /// Биом, определённый по высоте.
//     pub biome: Biome,
// }
// 
// /// Вся карта как плоский вектор в row-major порядке.
// ///
// /// Индексация: `tiles[row * width + col]`.
// #[derive(Resource, Default)]
// pub struct MapData {
//     pub width:  usize,
//     pub height: usize,
//     pub tiles:  Vec<TileData>,
// }
// 
// // impl MapData {
// //     /// Пустая карта — используется как начальное значение ресурса.
// //     pub fn empty() -> Self {
// //         Self {
// //             width:  0,
// //             height: 0,
// //             tiles:  Vec::new(),
// //         }
// //     }
// // 
// //     /// Получить ссылку на тайл по координатам (col, row).
// //     #[inline]
// //     pub fn get(&self, col: usize, row: usize) -> &TileData {
// //         &self.tiles[row * self.width + col]
// //     }
// // 
// //     /// Итератор по всем тайлам: (col, row, &TileData).
// //     pub fn iter_tiles(&self) -> impl Iterator<Item = (usize, usize, &TileData)> {
// //         self.tiles
// //             .iter()
// //             .enumerate()
// //             .map(|(i, tile)| (i % self.width, i / self.width, tile))
// //     }
// // 
// //     /// Общее количество тайлов.
// //     #[inline]
// //     pub fn tile_count(&self) -> usize {
// //         self.width * self.height
// //     }
// // }
