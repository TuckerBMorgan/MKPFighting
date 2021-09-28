use bevy::prelude::*;
use crate::systems::*;

pub const HEALTH_UI_HEIGHT : f32 = 300.0f32;
#[derive(Default)]
pub struct PlayerHealth {
    health: usize
}

impl PlayerHealth {
    pub fn new() -> PlayerHealth {
        PlayerHealth {
            health: 100
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct PlayerHealthUI {
    entity: Option<Entity>
}

impl PlayerHealthUI {
    pub fn new(entity: Entity) -> PlayerHealthUI {
        PlayerHealthUI {
            entity: Some(entity)
        }
    }
}

pub fn health_system_ui(
    mut health_query: Query<(&mut Transform, &PlayerHealthUI)>,
    mut players_query: Query<&PlayerHealth>
) {
    for (mut transform, &health_ui) in health_query.iter_mut() {
        let player_health = players_query.get(health_ui.entity.unwrap()).unwrap();
        //transform.scale.x = player_health.health as f32 * 4.0f32;
    }
}