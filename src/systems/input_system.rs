use bevy::prelude::*;
pub const INPUT_SIZE: usize = std::mem::size_of::<InputEvents>();
use ggrs::{GameInput, PlayerHandle};

//A Distilation of the true input state
#[derive(Default, Copy, Clone, Reflect, Hash)]
pub struct InputEvents {
    pub left_right_axis: i8,
    pub up_down_axis: i8,
    pub jump_was_pressed: bool,
    pub heavy_attack_was_pressed: bool,
    pub light_attack_was_pressed: bool,
    pub medium_attack_was_pressed: bool,
    pub special_ability: bool,
    pub dash: bool,
}

impl InputEvents {
    pub fn convert_input_events_into_vector(&self) -> Vec<u8> {
        let mut vector = vec![0 as u8; std::mem::size_of::<InputEvents>()];
        vector[0] = self.left_right_axis as u8;
        vector[1] = self.up_down_axis as u8;
        vector[2] = self.jump_was_pressed as u8;
        vector[3] = self.heavy_attack_was_pressed as u8;
        vector[4] = self.medium_attack_was_pressed as u8;
        vector[5] = self.light_attack_was_pressed as u8;
        vector[6] = self.special_ability as u8;
        vector[7] = self.dash as u8;
        return vector;
    }

    pub fn from_input_vector(input: &Res<Vec<GameInput>>, player_index: usize) -> InputEvents {
        InputEvents {
            left_right_axis: input[player_index].buffer[0] as i8,
            up_down_axis: input[player_index].buffer[1] as i8,
            jump_was_pressed: input[player_index].buffer[2] != 0,
            heavy_attack_was_pressed: input[player_index].buffer[3] != 0,
            medium_attack_was_pressed: input[player_index].buffer[4] != 0,
            light_attack_was_pressed: input[player_index].buffer[5] != 0,
            special_ability: input[player_index].buffer[6] != 0,
            dash: input[player_index].buffer[7] != 0,
        }
    }
}

pub fn keyboard_input_system(
    _handle: In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>,
    mut input_events: ResMut<InputEvents>,
) -> Vec<u8> {
    if keyboard_input.pressed(KeyCode::Left) == false && keyboard_input.pressed(KeyCode::Right) == false {
        input_events.left_right_axis = 0;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        input_events.left_right_axis = -1;
    } else if keyboard_input.pressed(KeyCode::Right) {
        input_events.left_right_axis = 1;
    }

    if keyboard_input.pressed(KeyCode::Left) && keyboard_input.pressed(KeyCode::Right) {
        input_events.left_right_axis = 0;
    }

    if keyboard_input.pressed(KeyCode::Up) == false && keyboard_input.pressed(KeyCode::Down) == false {
        input_events.up_down_axis = 0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        input_events.up_down_axis = -1;
    } else if keyboard_input.pressed(KeyCode::Down) {
        input_events.up_down_axis = 1;
    }

    input_events.jump_was_pressed = false;
    if keyboard_input.pressed(KeyCode::Up) {
        input_events.jump_was_pressed = true;
    }

    input_events.light_attack_was_pressed = false;
    if keyboard_input.pressed(KeyCode::Q) {
        input_events.light_attack_was_pressed = true;
    }

    input_events.medium_attack_was_pressed = false;
    if keyboard_input.pressed(KeyCode::W) {
        input_events.medium_attack_was_pressed = true;
    }

    input_events.heavy_attack_was_pressed = false;
    if keyboard_input.pressed(KeyCode::E) {
        input_events.heavy_attack_was_pressed = true;
    }

    input_events.special_ability = false;
    if keyboard_input.pressed(KeyCode::R) {
        input_events.special_ability = true;
    }

    input_events.dash = false;
    if keyboard_input.pressed(KeyCode::Space) {
        input_events.dash = true;
    }

    return input_events.convert_input_events_into_vector();
}
