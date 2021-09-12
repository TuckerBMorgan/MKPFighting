use bevy::prelude::*;


//A Distilation of the true input state
#[derive(Default)]
pub struct InputEvents {
    pub left_right_axis: f32,
    pub up_down_axis: f32,
    pub shoot_up_down: f32,
    pub shoot_left_right: f32,
    pub jump_was_pressed: bool,
    pub attack_1_was_pressed: bool
}

pub fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut input_events: ResMut<InputEvents>) {

    if input_events.left_right_axis != 0.0 {
        if keyboard_input.pressed(KeyCode::A) == false && keyboard_input.pressed(KeyCode::D) == false {
            input_events.left_right_axis = 0.0f32;
        }
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        input_events.left_right_axis = -1.0f32;
    }
    else if keyboard_input.just_pressed(KeyCode::D) {
        input_events.left_right_axis = 1.0f32;
    }

    if input_events.up_down_axis != 0.0 {
        if keyboard_input.pressed(KeyCode::W) == false && keyboard_input.pressed(KeyCode::S) == false {
            input_events.up_down_axis = 0.0f32;
        }
    }

    if keyboard_input.just_pressed(KeyCode::W) {
        input_events.up_down_axis = -1.0f32;
    }
    else if keyboard_input.just_pressed(KeyCode::S) {
        input_events.up_down_axis = 1.0f32;
    }


    if input_events.shoot_up_down != 0.0 {
        if keyboard_input.pressed(KeyCode::Up) == false && keyboard_input.pressed(KeyCode::Down) == false {
            input_events.shoot_up_down = 0.0f32;
        }
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        input_events.shoot_up_down = 1.0f32;
    }
    else if keyboard_input.just_pressed(KeyCode::Down) {
        input_events.shoot_up_down = -1.0f32;
    }


    if input_events.shoot_left_right != 0.0 {
        if keyboard_input.pressed(KeyCode::Left) == false && keyboard_input.pressed(KeyCode::Right) == false {
            input_events.shoot_left_right = 0.0f32;
        }
    }

    if keyboard_input.just_pressed(KeyCode::Left) {
        input_events.shoot_left_right = 1.0f32;
    }
    else if keyboard_input.just_pressed(KeyCode::Right) {
        input_events.shoot_left_right = -1.0f32;
    }

    input_events.jump_was_pressed = false;

    if keyboard_input.just_pressed(KeyCode::Space) {
        input_events.jump_was_pressed = true;
    }

    input_events.attack_1_was_pressed = false;
    if keyboard_input.just_pressed(KeyCode::Q) {
        input_events.attack_1_was_pressed = true;
    }

}