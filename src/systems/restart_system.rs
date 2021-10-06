use bevy::prelude::*;
use crate::systems::*;
use crate::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct RestartSystem;

#[derive(Default, Copy, Clone)]
pub struct UpperBlind {

}

#[derive(Default, Copy, Clone)]
pub struct LowerBlind {

}

pub fn restart_system(
    mut state: ResMut<State<GameState>>,
    command: Commands,
    mut upper_blind_query: Query<(&UpperBlind, &mut Transform), Without<(LowerBlind)>>,
    mut lower_blind_query: Query<(&LowerBlind, &mut Transform), Without<(UpperBlind)>>
) {

    let mut slides_in_place = 0;
    for (_up, mut transform) in lower_blind_query.iter_mut() {
        transform.translation.y -= 1.0f32;
        if transform.translation.y <= 0.0f32 {
            transform.translation.y = 0.0f32;
            slides_in_place += 1;
        }
    }

    for (_lp, mut transform) in upper_blind_query.iter_mut() {
        transform.translation.y += 1.0f32;
        if transform.translation.y >= 0.0f32 {
            transform.translation.y = 0.0f32;
            slides_in_place += 1;
        }
    }

    if slides_in_place == 2 {
        state.set(GameState::Fighting).unwrap();
    }

}