//! Корень модуля генерации мира.
//!
//! WorldPlugin регистрирует все ресурсы, события и системы.
//!
//! Структура модуля:
//!   config.rs    — параметры генерации (WorldConfig)
//!   noise.rs     — утилиты шума (fBm, normalise)
//!   heightmap.rs — генерация карты высот
//!   biome.rs     — биомы и пороги высот
//!   map.rs       — ресурс MapData (данные карты)
//!   generator.rs — чистая функция WorldConfig → MapData
//!   renderer.rs  — спавн 3D-боксов по MapData

pub mod config;
pub mod generator;
pub mod heightmap;
pub mod map;
pub mod noise;
pub mod renderer;

use bevy::prelude::*;

use config::WorldConfig;
use generator::generate_map_data;
use map::MapData;
use renderer::{despawn_tiles, spawn_tiles, TileAssets, TileMarker};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Ресурсы
            .insert_resource(WorldConfig::default())
            .insert_resource(MapData::empty())
            // Сообщения
            .add_message::<RegenerateWorldEvent>()
            // Системы
            .add_systems(Startup, startup_generate_and_render)
            .add_systems(Update, (handle_input, on_regenerate).chain());
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Сообщения
// ─────────────────────────────────────────────────────────────────────────────

/// Послать это событие, чтобы полностью перегенерировать мир.
#[derive(Message)]
pub struct RegenerateWorldEvent;

// ─────────────────────────────────────────────────────────────────────────────
// Системы
// ─────────────────────────────────────────────────────────────────────────────

/// Запускается один раз при старте: генерирует карту и спавнит тайлы.
fn startup_generate_and_render(
    mut commands:  Commands,
    config:        Res<WorldConfig>,
    mut map_data:  ResMut<MapData>,
    mut meshes:    ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1. Генерация данных карты (чистые вычисления, без ECS).
    *map_data = generate_map_data(&config);

    // 2. Создать shared меш + материалы.
    let assets = TileAssets::build(&mut meshes, &mut materials);

    // 3. Заспавнить боксы.
    spawn_tiles(&mut commands, &map_data, &config, &assets);

    info!(
        "Карта {}×{} сгенерирована. Seed: {}. Тайлов: {}.",
        config.map_width,
        config.map_height,
        config.seed,
        map_data.tile_count()
    );
}

/// Обработка клавиатуры для перегенерации.
fn handle_input(
    keyboard:    Res<ButtonInput<KeyCode>>,
    mut config:  ResMut<WorldConfig>,
    mut events:  MessageWriter<RegenerateWorldEvent>,
) {
    // R → детерминированный следующий seed
    if keyboard.just_pressed(KeyCode::KeyR) {
        config.seed = config.seed.wrapping_add(1);
        events.write(RegenerateWorldEvent);
        info!("Новый seed: {}", config.seed);
    }

    // Space → «случайный» seed (LCG на текущем seed)
    if keyboard.just_pressed(KeyCode::Space) {
        config.seed = config.seed
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        events.write(RegenerateWorldEvent);
        info!("Случайный seed: {}", config.seed);
    }
}

/// Реагирует на RegenerateWorldEvent: удаляет старые тайлы и строит новые.
fn on_regenerate(
    mut events:    MessageReader<RegenerateWorldEvent>,
    mut commands:  Commands,
    mut map_data:  ResMut<MapData>,
    config:        Res<WorldConfig>,
    tile_entities: Query<Entity, With<TileMarker>>,
    mut meshes:    ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ничего не делаем, если событий нет.
    if events.read().count() == 0 {
        return;
    }

    // Удаляем все старые тайлы.
    despawn_tiles(&mut commands, &tile_entities);

    // Перегенерируем данные.
    *map_data = generate_map_data(&config);

    // Спавним новые тайлы.
    let assets = TileAssets::build(&mut meshes, &mut materials);
    spawn_tiles(&mut commands, &map_data, &config, &assets);
}
