use std::collections::HashMap;
use std::path::Path;
use std::fs;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide};
use serde::{Deserialize, Serialize};
use crate::systems::*;


#[derive(Default, Serialize, Deserialize)]
pub struct ColliderSetComponent {
    pub colliders: HashMap<String, Vec<Vec<Collider>>>
}

impl ColliderSetComponent {
    pub fn from_file(path: &Path) -> ColliderSetComponent {
        let file_contents = fs::read_to_string(path).unwrap();
        let deserialized: ColliderSetComponent = serde_json::from_str(&file_contents).unwrap();
        return deserialized;
    }

    pub fn fake_one() -> ColliderSetComponent {
        let collider = Collider::new(Vec3::new(1.0, 2.0, 3.0), Vec2::new(0.0, -10.0), ColliderType::HitBox);
        let inner = vec![collider];
        let outer = vec![inner];
        let mut maps = HashMap::new();

        maps.insert(String::from("Idle"), outer);
        ColliderSetComponent {
            colliders: maps
        }
    }
}


#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum ColliderType {
    HitBox,
    HurtBox
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Collider {
    pub offset: Vec3,
    pub dimension: Vec2,
    pub collider_type: ColliderType
}


impl Collider {
    pub fn new(offset: Vec3, dimension: Vec2, collider_type: ColliderType) -> Collider {
        Collider {
            offset,
            dimension,
            collider_type
        }
    }
}

pub fn collision_system(
    collider_boxes: Res<ColliderSetComponent>,
    mut player_1_query: Query<(&Transform, &Player1, &mut PlayerState, &mut PlayerHealth, &ScreenSideEnum), Without<Player2>>,
    mut player_2_query: Query<(&Transform, &Player2, &mut PlayerState, &mut PlayerHealth, &ScreenSideEnum), Without<Player1>>) {        
    
    for (&transform_1, &_player_1, mut player_state_1, mut _health_1, &player_1_side) in player_1_query.iter_mut() {
        for (&transform_2, &_player_2, mut player_state_2, mut _health_2, &player_2_side) in player_2_query.iter_mut() {
            
            player_state_1.is_colliding = false;
            player_state_2.is_colliding = false;

            let p1_colliders = &collider_boxes.colliders[&player_state_1.player_state.to_string()][player_state_1.current_sprite_index];
            let p2_colliders = &collider_boxes.colliders[&player_state_2.player_state.to_string()][player_state_2.current_sprite_index];
            let mut player_1_should_inverse = 1.0f32;
            match player_1_side {
                ScreenSideEnum::Right => {
                    player_1_should_inverse = -1.0f32;
                },
                _ => {}
            }

            let mut player_2_should_inverse = 1.0f32;
            match player_2_side {
                ScreenSideEnum::Right => {
                    player_2_should_inverse = -1.0f32;
                },
                _ => {}
            }


            for collider_1 in p1_colliders {
                for collider_2 in p2_colliders {

                    let mut collider_1_offset = collider_1.offset.clone();
                    collider_1_offset.x = collider_1_offset.x * player_1_should_inverse;
                    let mut collider_2_offset = collider_2.offset.clone();
                    collider_2_offset.x = collider_2_offset.x * player_2_should_inverse;

                    let collision = collide(transform_1.translation + collider_1_offset, collider_1.dimension, transform_2.translation + collider_2_offset, collider_2.dimension);

                    match collision {
                        Some(_) => {
                            player_state_1.is_colliding = true;
                            player_state_2.is_colliding = true;
        
                            
                            match player_state_1.player_state {
                                PlayerStateEnum::Idle => {
                                    player_state_1.x_velocity = PLAYER_SPEED * player_1_side.back_direction();
                                },
                                PlayerStateEnum::Attack1 => {
        
                                },
                                _ => {}
                            }
                            match player_state_2.player_state {
                                PlayerStateEnum::Idle => {
                                    player_state_2.x_velocity = PLAYER_SPEED * player_2_side.back_direction();
                                },
                                PlayerStateEnum::Attack1 => {
        
                                },
                                _ => {}
                            }
                        },
                        None => {}
                    }
                }
            }
        }
    }
}
