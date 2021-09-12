
use bevy::prelude::*;
use crate::systems::*;
use crate::*;


#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PlayerStateEnum {
    Idle,
    Run,
    Jump,
    Attack1,
    Fall
}

impl Default for PlayerStateEnum {
    fn default() -> PlayerStateEnum {
        PlayerStateEnum::Idle
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ScreenSideEnum {
    Left,
    Right
}

impl Default for ScreenSideEnum {
    fn default() -> ScreenSideEnum {
        ScreenSideEnum::Left
    }
}

#[derive(Default)]
pub struct PlayerState {
    pub player_state: PlayerStateEnum,
    pub screen_side: ScreenSideEnum
}

impl PlayerState {

    pub fn new(player_state: PlayerStateEnum, screen_side: ScreenSideEnum) -> PlayerState {
        PlayerState {
            player_state,
            screen_side
        }
    }

    pub fn attempt_to_transition_state(&mut self, desired_state: PlayerStateEnum) -> bool {
        let copy_of_initial_state = self.player_state.clone();
        match self.player_state {
            PlayerStateEnum::Idle => {
                self.player_state = desired_state;
            },
            PlayerStateEnum::Run => {
                self.player_state = desired_state;
            },
            PlayerStateEnum::Jump => {
                //If I want to add an air dash, it would be here
            },
            PlayerStateEnum::Attack1 => {
                if desired_state == PlayerStateEnum::Jump {
                    self.player_state = desired_state;
                }
            },
            PlayerStateEnum::Fall => {
                //For now, keep it in this, but techianlly a "Landed" state would be a valid transtion for this
            }
        }
        return copy_of_initial_state != self.player_state;
    }

    pub fn animation_finished(&mut self) -> PlayerStateEnum{
        match self.player_state {
            PlayerStateEnum::Idle => {
                PlayerStateEnum::Idle
            },
            PlayerStateEnum::Run => {
                PlayerStateEnum::Run
            }
            PlayerStateEnum::Jump => {
                PlayerStateEnum::Fall
            },
            PlayerStateEnum::Attack1 => {
                PlayerStateEnum::Idle
            },
            PlayerStateEnum::Fall => {
                PlayerStateEnum::Fall
            }
        }
    }
}

pub fn player_state_system(
    mut commands: Commands,
    input: Res<InputEvents>,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>, Entity, &mut PlayerState)>,
    res_test: Res<TextureAtlasDictionary>
) {
    for (mut timer, mut sprite, texture_atlas_handle, entity, mut player_state) in query.iter_mut() {
        match player_state.screen_side {
            ScreenSideEnum::Left => {
                sprite.flip_x = false;
            },
            ScreenSideEnum::Right => {
                sprite.flip_x = true;
            }
        }
        let mut desired_state = player_state.player_state.clone();
        if input.left_right_axis != 0.0f32 {
            desired_state = PlayerStateEnum::Run;
        }
        else {
            desired_state = PlayerStateEnum::Idle;
        }

        if input.jump_was_pressed == true {
            desired_state = PlayerStateEnum::Jump;
        }

        if input.attack_1_was_pressed == true {
            desired_state = PlayerStateEnum::Attack1;
        }


        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        timer.tick(time.delta());
        if timer.finished() {
            let next = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            //As we start it at 0, we should let the system know "we have finished playing a full animation cycle, who wants next"
            if next == 0 {
                desired_state = player_state.animation_finished();
                player_state.player_state = desired_state;
                sprite.index = 0;
                match player_state.player_state {
                    PlayerStateEnum::Idle => {
                        commands.entity(entity).remove::<Handle<TextureAtlas>>();
                        commands.entity(entity).insert(res_test.animation_handles["sprites/Idle.png"].clone());
                    },
                    PlayerStateEnum::Run => {
                        commands.entity(entity).remove::<Handle<TextureAtlas>>();
                        commands.entity(entity).insert(res_test.animation_handles["sprites/Run.png"].clone());
                    },
                    PlayerStateEnum::Jump => {
                        commands.entity(entity).remove::<Handle<TextureAtlas>>();
                        commands.entity(entity).insert(res_test.animation_handles["sprites/Jump.png"].clone());
                    },
                    PlayerStateEnum::Attack1 => {
                        commands.entity(entity).remove::<Handle<TextureAtlas>>();
                        commands.entity(entity).insert(res_test.animation_handles["sprites/Attack1.png"].clone());
                    }
                    PlayerStateEnum::Fall => {
                        commands.entity(entity).remove::<Handle<TextureAtlas>>();
                        commands.entity(entity).insert(res_test.animation_handles["sprites/Fall.png"].clone());
                    }
                }
                continue;
            }
            sprite.index = next;
        }

        if player_state.attempt_to_transition_state(desired_state) {
            sprite.index = 0;
            match player_state.player_state {
                PlayerStateEnum::Idle => {
                    commands.entity(entity).remove::<Handle<TextureAtlas>>();
                    commands.entity(entity).insert(res_test.animation_handles["sprites/Idle.png"].clone());
                },
                PlayerStateEnum::Run => {
                    commands.entity(entity).remove::<Handle<TextureAtlas>>();
                    commands.entity(entity).insert(res_test.animation_handles["sprites/Run.png"].clone());
                },
                PlayerStateEnum::Jump => {
                    commands.entity(entity).remove::<Handle<TextureAtlas>>();
                    commands.entity(entity).insert(res_test.animation_handles["sprites/Jump.png"].clone());
                },
                PlayerStateEnum::Attack1 => {
                    commands.entity(entity).remove::<Handle<TextureAtlas>>();
                    commands.entity(entity).insert(res_test.animation_handles["sprites/Attack1.png"].clone());
                }
                PlayerStateEnum::Fall => {
                    commands.entity(entity).remove::<Handle<TextureAtlas>>();
                    commands.entity(entity).insert(res_test.animation_handles["sprites/Fall.png"].clone());
                }
            }
        }
    }
}