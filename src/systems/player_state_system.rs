use crate::systems::*;
use crate::*;

#[derive(PartialEq, Copy, Clone, Debug, Hash, Reflect, Component)]
#[reflect(Hash)]
pub enum PlayerStateEnum {
    Idle,
    Run,
    Jump,
    HeavyAttack,
    LightAttack,
    Fall,
    TakeLightHit,
    TakeHeavyHit,
    Death,
    Dash,
}

#[derive(Copy, Clone, Reflect, Hash, Default)]
pub struct AbilityTimer {
    pub total_frames: usize,
    pub current_frame: usize,
    pub running: bool,
}

impl AbilityTimer {
    pub fn new(total_frames: usize) -> AbilityTimer {
        AbilityTimer {
            total_frames,
            current_frame: 0,
            running: false,
        }
    }

    pub fn tick(&mut self) {
        self.current_frame += 1;
        if self.current_frame == self.total_frames {
            self.running = false;
        }
    }

    pub fn running(&mut self) -> bool {
        self.running
    }

    pub fn start(&mut self) {
        self.current_frame = 0;
        self.running = true;
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.running = false;
    }
}

impl PlayerStateEnum {
    pub fn to_string(&self) -> String {
        match self {
            PlayerStateEnum::Idle => String::from("Idle"),
            PlayerStateEnum::Run => String::from("Run"),
            PlayerStateEnum::Jump => String::from("Jump"),
            PlayerStateEnum::HeavyAttack => String::from("HeavyAttack"),
            PlayerStateEnum::LightAttack => String::from("LightAttack"),
            PlayerStateEnum::Fall => String::from("Fall"),
            PlayerStateEnum::TakeLightHit => String::from("TakeHit"),
            PlayerStateEnum::TakeHeavyHit => String::from("TakeHit"),
            PlayerStateEnum::Death => String::from("Death"),
            PlayerStateEnum::Dash => String::from("Dash"),
        }
    }
}

#[derive(Default, Copy, Clone, Component)]
pub struct Player1;
#[derive(Default, Copy, Clone, Component)]
pub struct Player2;

impl Default for PlayerStateEnum {
    fn default() -> PlayerStateEnum {
        PlayerStateEnum::Idle
    }
}

#[derive(Default, Reflect, Clone, Component, Hash, Copy)]
pub struct PlayerState {
    pub player_id: usize,
    pub player_state: PlayerStateEnum,
    pub desired_player_state: PlayerStateEnum,
    pub current_sprite_index: usize,
    pub x_velocity: i32,
    pub y_velocity: i32,
    pub is_colliding: bool,
    pub state_is_dirty: bool,
    pub has_spawned_cloud: bool,
    pub has_dahsed: bool,
    pub dash_timer: AbilityTimer,
    pub light_attack_timer: AbilityTimer,
    pub heavy_attack_timer: AbilityTimer,
}

impl PlayerState {
    pub fn new(player_id: usize, player_state: PlayerStateEnum) -> PlayerState {
        PlayerState {
            player_id,
            player_state,
            desired_player_state: player_state,
            current_sprite_index: 0,
            x_velocity: 0,
            y_velocity: 0,
            is_colliding: false,
            state_is_dirty: true,
            has_spawned_cloud: false,
            has_dahsed: false,
            dash_timer: AbilityTimer::new(35),
            light_attack_timer: AbilityTimer::new(25),
            heavy_attack_timer: AbilityTimer::new(75),
        }
    }

    pub fn attempt_to_transition_state(&mut self) -> bool {
        let copy_of_initial_state = self.player_state.clone();
        match self.player_state {
            PlayerStateEnum::Idle => {
                self.player_state = self.desired_player_state;
            }
            PlayerStateEnum::Run => {
                self.player_state = self.desired_player_state;
            }
            PlayerStateEnum::Jump => {}
            PlayerStateEnum::HeavyAttack => {
                if self.desired_player_state == PlayerStateEnum::Jump {
                    self.player_state = self.desired_player_state;
                }
            }
            PlayerStateEnum::LightAttack => {
                if self.desired_player_state == PlayerStateEnum::Jump {
                    self.player_state = self.desired_player_state;
                }
            }
            PlayerStateEnum::Fall => {
                //For now, keep it in this, but techianlly a "Landed" state would be a valid transtion for this
            }
            PlayerStateEnum::TakeLightHit => {}
            PlayerStateEnum::TakeHeavyHit => {}
            PlayerStateEnum::Death => {}
            PlayerStateEnum::Dash => {}
        }
        return copy_of_initial_state != self.player_state;
    }

    pub fn reset_state(&mut self) {
        self.current_sprite_index = 0;
    }

    pub fn hard_reset(&mut self) {
        self.player_state = PlayerStateEnum::Idle;
        self.desired_player_state = PlayerStateEnum::Idle;
        self.current_sprite_index = 0;
        self.x_velocity = 0;
        self.y_velocity = 0;
        self.is_colliding = false;
        self.state_is_dirty = true;
        self.has_spawned_cloud = false;
        self.has_dahsed = false;
        self.light_attack_timer.reset();
        self.heavy_attack_timer.reset();
    }

    pub fn animation_finished(&mut self) -> PlayerStateEnum {
        match self.player_state {
            PlayerStateEnum::Idle => PlayerStateEnum::Idle,
            PlayerStateEnum::Run => PlayerStateEnum::Run,
            PlayerStateEnum::Jump => PlayerStateEnum::Jump,
            PlayerStateEnum::HeavyAttack => PlayerStateEnum::Idle,
            PlayerStateEnum::LightAttack => PlayerStateEnum::Idle,
            PlayerStateEnum::Fall => PlayerStateEnum::Fall,
            PlayerStateEnum::TakeLightHit => PlayerStateEnum::Idle,
            PlayerStateEnum::TakeHeavyHit => PlayerStateEnum::Idle,
            PlayerStateEnum::Death => PlayerStateEnum::Death,
            PlayerStateEnum::Dash => PlayerStateEnum::Idle,
        }
    }

    pub fn can_take_a_hit(&self) -> bool {
        return (self.player_state != PlayerStateEnum::TakeLightHit
            && self.desired_player_state != PlayerStateEnum::TakeLightHit)
            && (self.player_state != PlayerStateEnum::TakeHeavyHit
                && self.desired_player_state != PlayerStateEnum::TakeHeavyHit);
    }

    pub fn set_player_state_to_transition(&mut self, new_player_state: PlayerStateEnum) {
        self.desired_player_state = new_player_state;
        self.state_is_dirty = true;
    }

    pub fn tick_timers(&mut self) {
        if self.light_attack_timer.running() {
            self.light_attack_timer.tick();
        }
        if self.heavy_attack_timer.running() {
            self.heavy_attack_timer.tick();
        }
        if self.dash_timer.running() {
            self.dash_timer.tick();
        }
    }

    pub fn level_and_amount_damage(&self) -> Option<(usize, PlayerStateEnum)> {
        if self.player_state == PlayerStateEnum::LightAttack {
            return Some((2, PlayerStateEnum::TakeLightHit));
        }
        return Some((10, PlayerStateEnum::TakeHeavyHit));
    }
}
pub fn player_state_system(
    mut commands: Commands,
    inputs: Res<Vec<GameInput>>,
    local_id: Res<LocalId>,
    mut query: Query<(
        &mut TextureAtlasSprite,
        Entity,
        &mut PlayerState,
        &ScreenSideEnum,
        &Transform,
        &mut SpriteTimer,
    )>,
    res_test: Res<TextureAtlasDictionary>,
) {
    for (mut sprite, entity, mut player_state, &screen_side, &transform, mut sprite_timer) in
        query.iter_mut()
    {
        player_state.tick_timers();

        let input = InputEvents::from_input_vector(&inputs, player_state.player_id);

        if player_state.state_is_dirty == false {
            if input.left_right_axis != 0 {
                if player_state.player_state == PlayerStateEnum::Idle {
                    player_state.set_player_state_to_transition(PlayerStateEnum::Run);
                }
            } else {
                if player_state.player_state == PlayerStateEnum::Run {
                    player_state.set_player_state_to_transition(PlayerStateEnum::Idle);
                }
            }

            if input.jump_was_pressed == true {
                if player_state.player_state == PlayerStateEnum::Idle
                    || player_state.player_state == PlayerStateEnum::Run
                {
                    player_state.set_player_state_to_transition(PlayerStateEnum::Jump);
                }
            }

            if input.heavy_attack_was_pressed == true
                && player_state.heavy_attack_timer.running() == false
            {
                if player_state.player_state == PlayerStateEnum::Idle
                    || player_state.player_state == PlayerStateEnum::Run
                {
                    player_state.set_player_state_to_transition(PlayerStateEnum::HeavyAttack);
                }
            }

            if input.light_attack_was_pressed == true
                && player_state.light_attack_timer.running() == false
            {
                if player_state.player_state == PlayerStateEnum::Idle
                    || player_state.player_state == PlayerStateEnum::Run
                {
                    player_state.set_player_state_to_transition(PlayerStateEnum::LightAttack);
                }
            }

            if input.dash == true
                && input.left_right_axis != 0
                && player_state.dash_timer.running() == false
            {
                if player_state.player_state == PlayerStateEnum::Idle
                    || player_state.player_state == PlayerStateEnum::Run
                {
                    player_state.set_player_state_to_transition(PlayerStateEnum::Dash);
                }
            }
        }
        //There are a number of things we are do in the idle
        if player_state.player_state == PlayerStateEnum::Idle {
            if input.special_ability == true && player_state.has_spawned_cloud == false {
                //Lets spawn a cloud entity at this characters feet
                player_state.has_spawned_cloud = true;
                let mut new_transform;
                if player_state.player_id != local_id.id {
                    new_transform = Transform::from_translation(Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        transform.translation.z + 1.0f32,
                    ));
                } else {
                    new_transform = Transform::from_translation(Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        transform.translation.z - 1.0f32,
                    ));
                }
                new_transform.scale.x *= 1.5f32;
                new_transform.scale.y *= 1.5f32;
                commands
                    .spawn_bundle(SpriteBundle {
                        material: res_test.cloud_image.clone(),
                        transform: new_transform,
                        ..Default::default()
                    })
                    .insert(CloudComponent::new(player_state.player_id));
            }
        }

        if player_state.attempt_to_transition_state() || player_state.state_is_dirty {
            sprite.index = 0;
            player_state.current_sprite_index = 0;
            sprite_timer.reset();
            let next_animation;
            match player_state.desired_player_state {
                PlayerStateEnum::Idle => {
                    next_animation = "sprites/Idle.png";
                    player_state.x_velocity = 0;
                }
                PlayerStateEnum::Run => {
                    next_animation = "sprites/Run.png";
                    player_state.x_velocity = PLAYER_SPEED * input.left_right_axis as i32;
                }
                PlayerStateEnum::Jump => {
                    next_animation = "sprites/Jump.png";
                    player_state.y_velocity = 25;
                }
                PlayerStateEnum::HeavyAttack => {
                    next_animation = "sprites/HeavyAttack.png";
                    player_state.x_velocity = 0;
                    player_state.heavy_attack_timer.start();
                }
                PlayerStateEnum::LightAttack => {
                    next_animation = "sprites/LightAttack.png";
                    player_state.x_velocity = 0;
                    player_state.light_attack_timer.start();
                }
                PlayerStateEnum::Fall => {
                    next_animation = "sprites/Fall.png";
                }
                PlayerStateEnum::TakeLightHit => {
                    next_animation = "sprites/TakeHit.png";
                    player_state.x_velocity =
                        PLAYER_LIGHT_HIT_SPEED * screen_side.back_direction() as i32;
                }
                PlayerStateEnum::TakeHeavyHit => {
                    next_animation = "sprites/TakeHit.png";
                    player_state.x_velocity =
                        PLAYER_HEAVY_HIT_SPEED * screen_side.back_direction() as i32;
                }
                PlayerStateEnum::Death => {
                    next_animation = "sprites/Death.png";
                    player_state.x_velocity = 0;
                }
                PlayerStateEnum::Dash => {
                    next_animation = "sprites/Dash.png";
                    player_state.x_velocity = PLAYER_DASH_SPEED * input.left_right_axis as i32;
                    player_state.dash_timer.start();
                }
            }
            commands
                .entity(entity)
                .insert(res_test.animation_handles[next_animation].clone());
            player_state.player_state = player_state.desired_player_state;
        }

        player_state.state_is_dirty = false;
    }
}
