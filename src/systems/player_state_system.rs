
use bevy::prelude::*;
use crate::systems::*;
use crate::*;


#[derive(PartialEq, Copy, Clone, Debug, Hash, Reflect)]
#[reflect(Hash)]
pub enum PlayerStateEnum {
    Idle,
    Run,
    Jump,
    Attack1,
    Fall,
    TakeHit,
    Death
}

impl PlayerStateEnum {
    pub fn to_string(&self) -> String {
        match self {
            PlayerStateEnum::Idle => {
                String::from("Idle")
            },
            PlayerStateEnum::Run => {
                String::from("Run")
            },
            PlayerStateEnum::Jump => {
                String::from("Jump")
            },
            PlayerStateEnum::Attack1 => {
                String::from("Attack1")
            },
            PlayerStateEnum::Fall => {
                String::from("Fall")
            },
            PlayerStateEnum::TakeHit => {
                String::from("TakeHit")
            },
            PlayerStateEnum::Death => {
                String::from("Death")
            }
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct Player1;
#[derive(Default, Copy, Clone)]
pub struct Player2;

impl Default for PlayerStateEnum {
    fn default() -> PlayerStateEnum {
        PlayerStateEnum::Idle
    }
}

pub struct WinningPlayer {}
pub struct LosingPlayer {}

#[derive(Default, Reflect, Copy, Clone)]
pub struct PlayerState {
    pub player_id: usize,
    pub player_state: PlayerStateEnum,
    pub desired_player_state: PlayerStateEnum,
    pub current_sprite_index: usize,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub is_colliding: bool,
    pub state_is_dirty: bool
}

impl PlayerState {

    pub fn new(player_id: usize, player_state: PlayerStateEnum) -> PlayerState {
        PlayerState {
            player_id,
            player_state,
            desired_player_state: player_state,
            current_sprite_index: 0,
            x_velocity: 0.0f32,
            y_velocity: 0.0f32,
            is_colliding: false,
            state_is_dirty: true
        }
    }

    pub fn attempt_to_transition_state(&mut self) -> bool {
        let copy_of_initial_state = self.player_state.clone();
        match self.player_state {
            PlayerStateEnum::Idle => {
                self.player_state = self.desired_player_state;
            },
            PlayerStateEnum::Run => {
                self.player_state = self.desired_player_state;
            },
            PlayerStateEnum::Jump => {
            },
            PlayerStateEnum::Attack1 => {
                if self.desired_player_state == PlayerStateEnum::Jump {
                    self.player_state = self.desired_player_state;
                }
            },
            PlayerStateEnum::Fall => {
                //For now, keep it in this, but techianlly a "Landed" state would be a valid transtion for this
            },
            PlayerStateEnum::TakeHit => {

            },
            PlayerStateEnum::Death => {
                
            }
        }
        return copy_of_initial_state != self.player_state;
    } 
    
    pub fn reset_state(&mut self) {
        self.current_sprite_index = 0;
    }

    pub fn animation_finished(&mut self) -> PlayerStateEnum {
        match self.player_state {
            PlayerStateEnum::Idle => {
                PlayerStateEnum::Idle
            },
            PlayerStateEnum::Run => {
                PlayerStateEnum::Run
            }
            PlayerStateEnum::Jump => {
                PlayerStateEnum::Jump
            },
            PlayerStateEnum::Attack1 => {
                PlayerStateEnum::Idle
            },
            PlayerStateEnum::Fall => {
                PlayerStateEnum::Fall
            },
            PlayerStateEnum::TakeHit => {
                PlayerStateEnum::Idle
            },
            PlayerStateEnum::Death => {
                PlayerStateEnum::Death
            }
        }
    }

    pub fn can_take_a_hit(&self) -> bool {
        return self.player_state != PlayerStateEnum::TakeHit && self.desired_player_state != PlayerStateEnum::TakeHit;
    }
    
    pub fn set_player_state_to_transition(&mut self, new_player_state: PlayerStateEnum) {
        self.desired_player_state = new_player_state;
        self.state_is_dirty = true;
    }
}
pub fn player_state_system(
    mut commands: Commands,
    inputs: Res<Vec<GameInput>>,
    mut query: Query<(&mut TextureAtlasSprite, Entity, &mut PlayerState, &ScreenSideEnum)>,
    res_test: Res<TextureAtlasDictionary>
) {
    for (mut sprite, entity, mut player_state, &screen_side) in query.iter_mut() {
        let input = InputEvents::from_input_vector(&inputs, player_state.player_id);
        if player_state.state_is_dirty == false {

        
            if input.left_right_axis != 0 {
                if player_state.player_state == PlayerStateEnum::Idle {
                    player_state.set_player_state_to_transition(PlayerStateEnum::Run);
                }
            }
            else {
                if player_state.player_state == PlayerStateEnum::Run {
                    player_state.set_player_state_to_transition(PlayerStateEnum::Idle);
                }
            }
                    
            if input.jump_was_pressed == true {
                if player_state.player_state == PlayerStateEnum::Idle || player_state.player_state == PlayerStateEnum::Run {
                    player_state.set_player_state_to_transition(PlayerStateEnum::Jump);
                }
            }
        
            if input.attack_1_was_pressed == true {
                if player_state.player_state == PlayerStateEnum::Idle || player_state.player_state == PlayerStateEnum::Run {
                    player_state.set_player_state_to_transition(PlayerStateEnum::Attack1);
                }
            }
        }


        if player_state.attempt_to_transition_state() || player_state.state_is_dirty {
            sprite.index = 0;
            player_state.current_sprite_index = 0;
            let next_animation;
            match player_state.desired_player_state {
                PlayerStateEnum::Idle => {
                    next_animation = "sprites/Idle.png";
                    player_state.x_velocity = 0.0;
                },
                PlayerStateEnum::Run => {
                    next_animation = "sprites/Run.png";
                    player_state.x_velocity = PLAYER_SPEED * input.left_right_axis as f32;
                },
                PlayerStateEnum::Jump => {
                    next_animation = "sprites/Jump.png";
                    player_state.y_velocity = 25.0f32;
                },
                PlayerStateEnum::Attack1 => {
                    next_animation = "sprites/Attack1.png";
                    player_state.x_velocity = 0.0;
                }
                PlayerStateEnum::Fall => {
                    next_animation = "sprites/Fall.png";
                },
                PlayerStateEnum::TakeHit => {
                    next_animation = "sprites/TakeHit.png";
                    player_state.x_velocity = PLAYER_SPEED * 1.5f32 * screen_side.back_direction();
                },
                PlayerStateEnum::Death => {
                    next_animation = "sprites/Death.png";
                    player_state.x_velocity = 0.0;
                }
            }
            commands.entity(entity).insert(res_test.animation_handles[next_animation].clone());
            player_state.player_state = player_state.desired_player_state;
        }
        player_state.state_is_dirty = false;
    }
}