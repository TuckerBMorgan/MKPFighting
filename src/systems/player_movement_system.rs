use bevy::prelude::*;
use crate::systems::*;

pub fn player_movement_system(
    input: Res<InputEvents>,
    mut query: Query<(&mut Timer, &mut Transform, &PlayerState)>,    
) {
    for (mut timer, mut transform, player_state) in query.iter_mut() {
        match player_state.player_state {
            PlayerStateEnum::Run => {
                transform.translation += Vec3::new(1.0, 0.0, 0.0) * input.left_right_axis;
            },
            _ => {

            }
        }
    }
}