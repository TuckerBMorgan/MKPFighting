use crate::*;

#[derive(Copy, Clone, Default, Component)]
pub struct RoundTimer {
    total_round_timer: usize,
    round_counter: usize,
}

impl RoundTimer {
    pub fn new(round_time_in_seconds: usize) -> RoundTimer {
        RoundTimer {
            round_counter: round_time_in_seconds * 60,
            total_round_timer: round_time_in_seconds * 60,
        }
    }

    pub fn reset(&mut self) {
        self.round_counter = self.total_round_timer;
    }
}

pub fn round_timer_system(
    mut state: ResMut<State<GameState>>,
    mut timer_query: Query<(&mut RoundTimer, &mut Text)>,
) {
    for (mut round_timer, mut text) in timer_query.iter_mut() {
        round_timer.round_counter -= 1;
        text.sections[0].value = format!("{}", round_timer.round_counter / 60);
        if round_timer.round_counter == 0 {
            state.set(GameState::Reset).unwrap();
        }
    }
}
