use bevy::prelude::*;
use ggrs::{GameInput};
use crate::systems::*;

pub fn player_movement_system(
    inputs: Res<Vec<GameInput>>,
    mut query: Query<(&mut Timer, &mut Transform, &PlayerState)>,    
) {
    for (mut timer, mut transform, player_state) in query.iter_mut() {
        let input = InputEvents::from_input_vector(&inputs, player_state.player_id);
        match player_state.player_state {
            PlayerStateEnum::Run => {
                transform.translation += Vec3::new(1.0, 0.0, 0.0) * input.left_right_axis as f32;
            },
            _ => {

            }
        }
    }
}