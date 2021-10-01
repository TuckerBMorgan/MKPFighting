use bevy::prelude::*;
use ggrs::{GameInput};
use crate::systems::*;

const GRAVITY : f32 = 1.00f32;
pub const FLOOR_HEIGHT : f32 = -250.0f32;
pub const PLAYER_SPEED : f32 = 5.0f32;

pub fn player_movement_system(
    inputs: Res<Vec<GameInput>>,
    mut query: Query<(&mut Transform, &mut PlayerState)>,    
) {
    for (mut transform, mut player_state) in query.iter_mut() {
        let input = InputEvents::from_input_vector(&inputs, player_state.player_id);
        transform.translation += Vec3::new(player_state.x_velocity, player_state.y_velocity, 0.0);// *  as f32;

        match player_state.player_state {
            PlayerStateEnum::Run => {
            },
            PlayerStateEnum::Jump => {
                player_state.y_velocity -= GRAVITY;
                if player_state.y_velocity < 0.0f32 {
                    player_state.set_player_state_to_transition(PlayerStateEnum::Fall);
                }
            },
            PlayerStateEnum::Fall => {
                player_state.y_velocity -= GRAVITY;
                if transform.translation.y < FLOOR_HEIGHT {
                    player_state.set_player_state_to_transition(PlayerStateEnum::Idle);
                    player_state.y_velocity = 0.0f32;
                    transform.translation.y = FLOOR_HEIGHT;
                }
            },
            PlayerStateEnum::Idle => {
                if player_state.is_colliding == false {
                    player_state.x_velocity = 0.0f32;
                }
            },
            PlayerStateEnum::Death => {
                if transform.translation.y < FLOOR_HEIGHT {
                    player_state.y_velocity = 0.0f32;
                    transform.translation.y = FLOOR_HEIGHT;
                }
            }
            _ => {}
        }
    }
}