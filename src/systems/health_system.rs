use bevy::prelude::*;
use crate::systems::*;
use crate::*;

pub const HEALTH_UI_HEIGHT : f32 = 300.0f32;
#[derive(Default)]
pub struct PlayerHealth {
    pub health: usize
}

impl PlayerHealth {
    pub fn new() -> PlayerHealth {
        PlayerHealth {
            health: 10
        }
    }
    
    pub fn take_damage(&mut self, amount: usize) -> bool{
        if amount > self.health {
            self.health = 0;
            return true;
        }
        else {
            self.health -= amount;
            return false;
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
    mut state: ResMut<State<GameState>>,
    mut health_query: Query<(&mut Transform, &PlayerHealthUI)>,
    mut players_query: Query<(&PlayerHealth, &mut PlayerState)>
) {
    let mut someone_died = false;
    for (mut transform, &health_ui) in health_query.iter_mut() {
        let (player_health, mut player_state) = players_query.get_mut(health_ui.entity.unwrap()).unwrap();
        transform.scale.x = player_health.health as f32 * 4.0f32;
        if player_health.health == 0 {
            player_state.set_player_state_to_transition(PlayerStateEnum::Death);
            someone_died = true;
        }
    }

    if someone_died  {
        state.set(GameState::Reset).unwrap();
    }
}