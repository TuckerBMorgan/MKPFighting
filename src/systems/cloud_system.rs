use bevy::prelude::*;
use crate::systems::*;

#[derive(Default, Component)]
pub struct CloudComponent {
    player_id: usize
}

impl CloudComponent {
    pub fn new(player_id: usize) -> CloudComponent {
        CloudComponent {
            player_id
        }
    }
}

pub fn cloud_system(
    mut cloud_component: Query<(&mut Transform, &CloudComponent)>,
) {
    for (mut transform, _cc) in cloud_component.iter_mut() {
        
    }
}