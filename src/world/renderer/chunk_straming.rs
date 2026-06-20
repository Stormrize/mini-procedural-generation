use bevy::{prelude::*, window::PrimaryWindow};
use std::time::Instant;
use crate::camera::MainCamera;
use crate::world::renderer::camera_math::{
    camera_chunk_bounds, camera_ground_point, chunk_coord_from_world_xz,
    finite_world_chunk_bounds, outside_bounds,
};
use crate::world::renderer::Rendered;
use crate::world::renderer::chunk_renderer::{
    despawn_chunk_meshes, spawn_chunk_meshes, TileAssets,
};
use crate::world::renderer::details_renderer::{despawn_models, spawn_models};
use crate::world::world::World;
use crate::world::world_config::WorldConfig;

/// Главная система, которая управляет жизненным циклом чанков вокруг камеры.
pub fn stream_chunks(
    mut commands: Commands,
    config: Res<WorldConfig>,
    mut world: ResMut<World>,
    mut rendered: ResMut<Rendered>,
    tile_assets: Res<TileAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_server: ResMut<AssetServer>,
    camera_q: Query<(&Transform, &Projection), With<MainCamera>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    // 1. Проверяем компоненты камеры и окна
    let Ok((camera_transform, projection)) = camera_q.single() else { return; };
    let Ok(window) = window_q.single() else { return; };

    // 2. Считаем точку фокуса и базовые границы загрузки
    let center = camera_ground_point(camera_transform);
    let center_chunk = chunk_coord_from_world_xz(center.x, center.z, &config);

    let (load_min, load_max) = calculate_load_area(camera_transform, projection, window, &config);
    let has_load_area = load_min.x <= load_max.x && load_min.y <= load_max.y;

    // 3. Выгружаем чанки, которые стали слишком далеки от камеры
    unload_distant_chunks(
        &mut commands,
        &mut meshes,
        &mut world,
        &mut rendered,
        &config,
        load_min,
        load_max,
        has_load_area,
    );

    if !has_load_area {
        return;
    }

    // 4. Собираем список чанков, которые нужно войти в зону видимости, и сортируем их по дистанции
    let (missing_chunks, missing_details) = get_sorted_missing_chunks(&rendered, load_min, load_max, center_chunk);

    // 5. Постепенно загружаем недостающие чанки (с ограничением по времени на кадр)
    load_missing_chunks(
        &mut commands,
        &mut meshes,
        &mut world,
        &mut rendered,
        &config,
        &tile_assets,
        missing_chunks,
    );

    load_missing_details(
        &mut commands,
        &mut asset_server,
        &config,
        missing_details,
        &mut rendered,
    );
}

// --- Вспомогательные функции (Приватные для этого модуля) ---

/// Вычисляет финальные границы чанков для загрузки с учетом ограничений карты
fn calculate_load_area(
    camera_transform: &Transform,
    projection: &Projection,
    window: &Window,
    config: &WorldConfig,
) -> (IVec2, IVec2) {
    let (mut min, mut max) = camera_chunk_bounds(camera_transform, projection, window, config);

    if let Some((world_min, world_max)) = finite_world_chunk_bounds(config) {
        min = min.max(world_min);
        max = max.min(world_max);
    }

    (min, max)
}

/// Определяет зону выгрузки и удаляет чанки, вышедшие за её пределы
fn unload_distant_chunks(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    world: &mut ResMut<World>,
    rendered: &mut ResMut<Rendered>,
    config: &WorldConfig,
    load_min: IVec2,
    load_max: IVec2,
    has_load_area: bool,
) {
    let (unload_min, unload_max) = if has_load_area {
        let unload_padding = IVec2::splat(config.unload_margin_chunks as i32);
        let mut min = load_min - unload_padding;
        let mut max = load_max + unload_padding;

        if let Some((world_min, world_max)) = finite_world_chunk_bounds(config) {
            min = min.max(world_min);
            max = max.min(world_max);
        }
        (min, max)
    } else {
        (IVec2::ONE, IVec2::ZERO) // Инвалидная зона, выгрузит вообще всё
    };

    let to_unload: Vec<IVec2> = rendered
        .chunks
        .keys()
        .copied()
        .filter(|coord| outside_bounds(*coord, unload_min, unload_max))
        .collect();



    for coord in to_unload {
        if let Some(chunk_meshes) = rendered.chunks.remove(&coord) {
            despawn_chunk_meshes(commands, meshes, chunk_meshes);
        }
        if let Some(model) = rendered.details.remove(&coord) {
            despawn_models(commands, model);
        }
        world.unload_chunk(coord);
    }
}

/// Находит нераспрендеренные координаты и сортирует их от центра к краям (Чебышёв + Манхэттен)
/// оптимизировать, не вызывать каждый кадр
fn get_sorted_missing_chunks(
    rendered: &Rendered,
    load_min: IVec2,
    load_max: IVec2,
    center_chunk: IVec2,
) -> (Vec<IVec2>, Vec<IVec2>) {
    let mut missing_chunks = Vec::new();
    let mut missing_details = Vec::new();

    for y in load_min.y..=load_max.y {
        for x in load_min.x..=load_max.x {
            let coord = IVec2::new(x, y);
            if !rendered.chunks.contains_key(&coord) {
                missing_chunks.push(coord);
            }
            if !rendered.details.contains_key(&coord) {
                missing_details.push(coord);
            }
        }
    }

    // Сортировка: сначала те, что ближе к камере
    missing_chunks.sort_by_key(|coord| {
        let delta = *coord - center_chunk;
        let chebyshev = delta.x.abs().max(delta.y.abs());
        let manhattan = delta.x.abs() + delta.y.abs();
        (chebyshev, manhattan)
    });

    (missing_chunks, missing_details)
}

/// Итерируется по пропущенным чанкам, генерирует их и создает меши, пока не упрется в лимит времени/количества
fn load_missing_chunks(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    world: &mut ResMut<World>,
    rendered: &mut ResMut<Rendered>,
    config: &WorldConfig,
    tile_assets: &TileAssets,
    missing_chunks: Vec<IVec2>,
) {
    let start_time = Instant::now();
    let max_loads = config.max_chunk_loads_per_frame as usize;

    for coord in missing_chunks.into_iter().take(max_loads) {
        // Защита от просадок FPS: если генерация заняла больше 5мс, прерываемся до следующего кадра
        if start_time.elapsed().as_millis() > 5 {
            break;
        }

        // На случай, если пока мы итерировались, что-то изменилось
        if rendered.chunks.contains_key(&coord) {
            continue;
        }

        // Генерация данных чанка (логика)
        let chunk = world.chunk_or_generate(coord, config);

        // Создание визуальной части (мешей)
        let chunk_meshes = spawn_chunk_meshes(commands, meshes, chunk, config, tile_assets);

        // Регистрируем готовый чанк
        rendered.chunks.insert(coord, chunk_meshes);
    }
}

fn load_missing_details(
    commands: &mut Commands,
    asset_server: &mut ResMut<AssetServer>,
    config: &WorldConfig,
    missing_details: Vec<IVec2>,
    rendered: &mut ResMut<Rendered>,
) {
    for coord in missing_details.into_iter() {

        // Создание деталей
        let models = spawn_models(commands, asset_server, config, coord);
        rendered.details.insert(coord, models);
    }

}
