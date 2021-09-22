use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;
use std::fs;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::systems::*;


#[derive(Default, Serialize, Deserialize)]
pub struct ColliderSetComponent {
    colliders: HashMap<String, Vec<Vec<Collider>>>
}

impl ColliderSetComponent {
    pub fn from_file(path: &Path) -> ColliderSetComponent {
        let file_contents = fs::read_to_string(path).unwrap();
        let deserialized: ColliderSetComponent = serde_json::from_str(&file_contents).unwrap();
        return deserialized;
    }

    pub fn fake_one() -> ColliderSetComponent {
        let collider = Collider::new(Vec3::new(1.0, 2.0, 3.0), Vec2::new(0.0, -10.0));
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
pub struct Collider {
    offset: Vec3,
    dimension: Vec2
}


impl Collider {
    pub fn new(offset: Vec3, dimension: Vec2) -> Collider {
        Collider {
            offset,
            dimension
        }
    }
}

pub fn collision_system(
    collider_boxes: Res<ColliderSetComponent>,
    mut player_1_query: Query<(&Transform, &Player1, &mut PlayerState), Without<Player2>>,
    mut player_2_query: Query<(&Transform, &Player2, &mut PlayerState), Without<Player1>>) {
    
        
    for (&transform_1, &_player_1, mut player_state_1) in player_1_query.iter_mut() {
        for (&transform_2, &_player_2, mut player_state_2) in player_2_query.iter_mut() {
            
            player_state_1.is_colliding = false;
            player_state_2.is_colliding = false;

            let p1_colliders = &collider_boxes.colliders[&player_state_1.player_state.to_string()][player_state_1.current_sprite_index];
            let p2_colliders = &collider_boxes.colliders[&player_state_2.player_state.to_string()][player_state_2.current_sprite_index];
            for collider_1 in p1_colliders {
                for collider_2 in p2_colliders {
                    let collision = collide(transform_1.translation + collider_1.offset, collider_1.dimension, transform_2.translation + collider_2.offset, collider_2.dimension);

                    match collision {
                        Some(_) => {
                            player_state_1.is_colliding = true;
                            player_state_2.is_colliding = true;
        
                            
                            match player_state_1.player_state {
                                PlayerStateEnum::Idle => {
                                    player_state_1.x_velocity = -1.0f32;
                                },
                                PlayerStateEnum::Attack1 => {
        
                                },
                                _ => {}
                            }
                            match player_state_2.player_state {
                                PlayerStateEnum::Idle => {
                                    player_state_2.x_velocity = 1.0f32;
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
