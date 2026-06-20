use bevy::prelude::*;
use crate::world::world_config::WorldConfig;

#[derive(Message)] // Обычно в Bevy используется Event, а не Message
pub struct RegenerateWorldEvent;

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<WorldConfig>,
    mut events: MessageWriter<RegenerateWorldEvent>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        config.seed = config.seed.wrapping_add(1);
        events.write(RegenerateWorldEvent);
        info!("Новый seed: {}", config.seed);
    }

    if keyboard.just_pressed(KeyCode::Space) {
        config.seed = config
            .seed
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        events.write(RegenerateWorldEvent);
        info!("Случайный seed: {}", config.seed);
    }
}