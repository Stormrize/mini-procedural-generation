use crate::terrain::world_config::WorldConfig;
use bevy::prelude::*;
use noise::Perlin;
use crate::terrain::generation::heightmap::PerlinPlaceholder;

#[derive(Message)] // Обычно в Bevy используется Event, а не Message
pub struct RegenerateWorldEvent;

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<WorldConfig>,
    mut events: MessageWriter<RegenerateWorldEvent>,
    mut perlin: ResMut<PerlinPlaceholder>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        config.seed = config.seed.wrapping_add(1);
        perlin.perlin_main = Perlin::new(config.seed);
        perlin.perlin_detail = Perlin::new(config.seed.wrapping_add(1337));
        events.write(RegenerateWorldEvent);
        info!("Новый seed: {}", config.seed);
    }

    if keyboard.just_pressed(KeyCode::Space) {
        config.seed = config
            .seed
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        events.write(RegenerateWorldEvent);
        perlin.perlin_main = Perlin::new(config.seed);
        perlin.perlin_detail = Perlin::new(config.seed.wrapping_add(1337));
        info!("Случайный seed: {}", config.seed);
    }
}