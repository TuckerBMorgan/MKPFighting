use bevy::prelude::*;

#[derive(Default, Component)]
pub struct CloudComponent {
    #[allow(dead_code)]
    player_id: usize,
}

impl CloudComponent {
    pub fn new(player_id: usize) -> CloudComponent {
        CloudComponent { player_id }
    }
}
