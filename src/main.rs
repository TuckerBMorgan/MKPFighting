use std::net::SocketAddr;
use structopt::StructOpt;
use std::path::Path;

use bevy::{prelude::*};
use bevy_ggrs::{Rollback, RollbackIdProvider, GGRSApp, GGRSPlugin};
use ggrs::{GameInput, PlayerType, P2PSession};


use std::collections::HashMap;

mod systems;
use crate::systems::*;


#[derive(Default)]
pub struct TextureAtlasDictionary {
    pub animation_handles: HashMap<String, Handle<TextureAtlas>>,
    pub debug_hit_box_texture: Handle<ColorMaterial>,
    pub debug_hurt_box_texture: Handle<ColorMaterial>
}



const FPS: u32 = 60;
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // read cmd line arguments
    let opt = Opt::from_args();
    let num_players = opt.players.len();
    assert!(num_players > 0);

    let mut p2p_sess = P2PSession::new(2, INPUT_SIZE, opt.local_port)?;
    p2p_sess.set_sparse_saving(true)?;
    p2p_sess.set_fps(FPS).expect("Invalid fps");

    let collider_both = Path::new("./assets/hitboxes/character_1.json");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GGRSPlugin)
        .insert_resource(opt)
        .insert_resource(ColliderSetComponent::from_file(&collider_both))
        .insert_resource(InputEvents::default())
        .insert_resource(TextureAtlasDictionary::default())
        .insert_resource(ShouldRenderHitBoxes::default())
        .add_startup_system(start_p2p_session)
        .add_startup_system(setup)
        .add_startup_system(hit_box_setup_system)
        .register_rollback_type::<Transform>()
        .register_rollback_type::<PlayerState>()
        .with_input_system(keyboard_input_system.system())
        .add_rollback_system(player_movement_system)
        .add_rollback_system(player_state_system)
        .with_p2p_session(p2p_sess)
        .add_system(sprite_timers)
        .add_system(collision_system)
        .add_system(screen_side_system)
        .add_system(hit_box_setup_system)
        .run();
    Ok(())
    
}


fn sprite_timers(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>, Entity, &mut PlayerState, &ScreenSideEnum)>,
    res_test: Res<TextureAtlasDictionary>
) {
    for (mut timer, mut sprite, texture_atlas_handle, entity, mut player_state, &screen_side) in query.iter_mut() {
        timer.tick(time.delta());
        let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
        match screen_side {
            ScreenSideEnum::Left => {
                sprite.flip_x = false;
            },
            ScreenSideEnum::Right => {
                sprite.flip_x = true;
            }
        }

        if timer.finished() {
            let next = ((player_state.current_sprite_index as usize + 1) % texture_atlas.textures.len()) as u32;
            //As we start it at 0, we should let the system know "we have finished playing a full animation cycle, who wants next"
            if next == 0 {
                let desired_state = player_state.animation_finished();
                player_state.player_state = desired_state;
                sprite.index = 0;
                player_state.current_sprite_index = 0;
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
            player_state.current_sprite_index = next as usize;
        }
    }
}

// structopt will read command line parameters for u
#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    local_port: u16,
    #[structopt(short, long)]
    players: Vec<String>,
    #[structopt(short, long)]
    spectators: Vec<SocketAddr>,
}

fn start_p2p_session(mut p2p_sess: ResMut<P2PSession>, opt: Res<Opt>) {
    let mut local_handle = 0;
    let num_players = p2p_sess.num_players() as usize;

    // add players
    for (i, player_addr) in opt.players.iter().enumerate() {
        // local player
        if player_addr == "localhost" {
            p2p_sess.add_player(PlayerType::Local, i).unwrap();
            local_handle = i;
        } else {
            // remote players
            let remote_addr: SocketAddr =
                player_addr.parse().expect("Invalid remote player address");
            p2p_sess
                .add_player(PlayerType::Remote(remote_addr), i)
                .unwrap();
        }
    }

    // optionally, add spectators
    for (i, spec_addr) in opt.spectators.iter().enumerate() {
        p2p_sess
            .add_player(PlayerType::Spectator(*spec_addr), num_players + i)
            .unwrap();
    }

    // set input delay for the local player
    p2p_sess.set_frame_delay(2, local_handle).unwrap();

    // set default expected update frequency (affects synchronization timings between players)
    p2p_sess.set_fps(FPS).expect("Invalid fps");

    // start the GGRS session
    p2p_sess.start_session().unwrap();
}

fn load_sprite_atlas_into_texture_dictionary(
    animation_name: String, 
    asset_server: &Res<AssetServer>, 
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    texture_atlas_handles: &mut ResMut<TextureAtlasDictionary>,
    width: f32,
    height: f32,
    number_of_images: usize
) {
    let texture_handle = asset_server.load(animation_name.as_str());
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(width, height), number_of_images, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    texture_atlas_handles.animation_handles.insert(animation_name, texture_atlas_handle);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rip: ResMut<RollbackIdProvider>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_handles: ResMut<TextureAtlasDictionary>,
    p2p_session: Option<Res<P2PSession>>,
) {
    load_sprite_atlas_into_texture_dictionary(String::from("sprites/Idle.png"), &asset_server, &mut texture_atlases, &mut texture_atlas_handles, 200.0, 200.0, 8);
    load_sprite_atlas_into_texture_dictionary(String::from("sprites/Run.png"), &asset_server, &mut texture_atlases, &mut texture_atlas_handles, 200.0, 200.0, 8);
    load_sprite_atlas_into_texture_dictionary(String::from("sprites/Jump.png"), &asset_server, &mut texture_atlases, &mut texture_atlas_handles, 200.0, 200.0, 2);
    load_sprite_atlas_into_texture_dictionary(String::from("sprites/Attack1.png"), &asset_server, &mut texture_atlases, &mut texture_atlas_handles, 200.0, 200.0, 6);
    load_sprite_atlas_into_texture_dictionary(String::from("sprites/Fall.png"), &asset_server, &mut texture_atlases, &mut texture_atlas_handles, 200.0, 200.0, 2);

    let num_players = p2p_session
        .map(|s| s.num_players()).expect("No GGRS session found");
    
    //Spawn the background image, simply fire and forget
    let background_texture_handle = asset_server.load("sprites/background_bar.png");
    let mut background_transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    background_transform.scale.x = 0.8;
    background_transform.scale.y = 0.8;

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(background_texture_handle.into()),
        transform: background_transform,
        ..Default::default()
    });

    //Spawn each player
    for i in 0..num_players {
        if i == 0 {
            commands.spawn_bundle(OrthographicCameraBundle::new_2d());
            let mut p1_transform = Transform::from_translation(Vec3::new(-100.0 + (200.0 * i as f32), FLOOR_HEIGHT, 0.0));
            p1_transform.scale.x = 2.0;
            p1_transform.scale.y = 2.0;
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handles.animation_handles["sprites/Idle.png"].clone(),
                    transform:p1_transform,
                    ..Default::default()
                })
                .insert(Timer::from_seconds(0.1, true))
                .insert(PlayerState::new(i as usize, PlayerStateEnum::Idle))
                .insert(Rollback::new(rip.next_id()))
                .insert(Player1::default())
                .insert(PlayerHealth::default())
                .insert(ScreenSideEnum::Left);
        }
        else {
            commands.spawn_bundle(OrthographicCameraBundle::new_2d());
            let mut p1_transform = Transform::from_translation(Vec3::new(-100.0 + (200.0 * i as f32), FLOOR_HEIGHT, 0.0));
            p1_transform.scale.x = 2.0;
            p1_transform.scale.y = 2.0;

            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handles.animation_handles["sprites/Idle.png"].clone(),
                    transform:p1_transform,
                    ..Default::default()
                })
                .insert(Timer::from_seconds(0.1, true))
                .insert(PlayerState::new(i as usize, PlayerStateEnum::Idle))
                .insert(Rollback::new(rip.next_id()))
                .insert(Player2::default())
                .insert(PlayerHealth::default())
                .insert(ScreenSideEnum::Right);
        }
    }
}