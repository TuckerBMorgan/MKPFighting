
use crate::systems::*;
use crate::*;
use bevy::prelude::*;

#[derive(Copy, Clone, Default, Component)]
pub struct RoundTimer {
    round_counter: usize
}

impl RoundTimer {
    pub fn new(round_time_in_seconds: usize) -> RoundTimer {
        RoundTimer {
            round_counter: round_time * 60
        }
    }
}

pub fn round_timer(
    timer_query: Query<(&mut RoundTimer, &mut Text)>
) {
    for (mut round_timer, mut text) in timer_query.iter_mut() {
        round_timer.round_counter -= 1;
        text.text = round_counter.to_string();
    }
}