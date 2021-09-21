use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

use crate::systems::*;

#[derive(Copy, Clone)]
pub struct Collider {
    dim: Vec2
}


impl Collider {
    pub fn new(dim: Vec2) -> Collider {
        Collider {
            dim
        }
    }
}

pub fn collision_system(
    mut player_1_query: Query<(&Transform, &Collider, &Player1, &mut PlayerState), Without<Player2>>,
    mut player_2_query: Query<(&Transform, &Collider, &Player2, &mut PlayerState), Without<Player1>>) {
    

    for (&transform_1, &collider_1, &player_1, mut player_state_1) in player_1_query.iter_mut() {
        for (&transform_2, &collider_2, &player_2, mut player_state_2) in player_2_query.iter_mut() {
            let collision = collide(transform_1.translation, collider_1.dim, transform_2.translation, collider_2.dim);

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
                None => {
                    player_state_1.is_colliding = false;
                    player_state_2.is_colliding = false;
                }
                
            }
        }
    }




}
