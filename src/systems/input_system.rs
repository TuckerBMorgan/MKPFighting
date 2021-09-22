use bevy::prelude::*;
pub const INPUT_SIZE: usize = std::mem::size_of::<InputEvents>();
use ggrs::{GameInput, PlayerHandle};

//A Distilation of the true input state
#[derive(Default)]
pub struct InputEvents {
    pub left_right_axis: i8,
    pub up_down_axis: i8,
    pub jump_was_pressed: bool,
    pub attack_1_was_pressed: bool
}

impl InputEvents {
    pub fn convert_input_events_into_vector(&self) -> Vec<u8> {
        let mut vector = vec![0 as u8; std::mem::size_of::<InputEvents>()];
        vector[0] = self.left_right_axis as u8;
    
        vector[1] = self.up_down_axis as u8;
    
        vector[2] = self.jump_was_pressed as u8;
    
        vector[3] = self.attack_1_was_pressed as u8;
        return vector;
    }    

    pub fn from_input_vector(input: &Res<Vec<GameInput>>, player_index: usize) -> InputEvents {
        InputEvents {
            left_right_axis: input[player_index].buffer[0] as i8,
            up_down_axis: input[player_index].buffer[1] as i8,
            jump_was_pressed: input[player_index].buffer[2] != 0,
            attack_1_was_pressed: input[player_index].buffer[3] != 0,
        }
    }
}


pub fn keyboard_input_system(_handle: In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>, mut input_events: ResMut<InputEvents>) -> Vec<u8> {

    if input_events.left_right_axis != 0 {
        if keyboard_input.pressed(KeyCode::A) == false && keyboard_input.pressed(KeyCode::D) == false {
            input_events.left_right_axis = 0;
        }
    }

    if keyboard_input.pressed(KeyCode::A) {
        input_events.left_right_axis = -1;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input_events.left_right_axis = 1;
    }

    if input_events.up_down_axis != 0 {
        if keyboard_input.pressed(KeyCode::W) == false && keyboard_input.pressed(KeyCode::S) == false {
            input_events.up_down_axis = 0;
        }
    }

    if keyboard_input.pressed(KeyCode::W) {
        input_events.up_down_axis = -1;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input_events.up_down_axis = 1;
    }

    input_events.jump_was_pressed = false;

    if keyboard_input.pressed(KeyCode::Space) {
        input_events.jump_was_pressed = true;
    }

    input_events.attack_1_was_pressed = false;
    if keyboard_input.pressed(KeyCode::Q) {
        input_events.attack_1_was_pressed = true;
    }
    return input_events.convert_input_events_into_vector();
}