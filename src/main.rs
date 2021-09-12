use bevy::prelude::*;

use bevy_ggrs::{GGRSApp, GGRSPlugin, RollbackIdProvider};
use ggrs::{GameInput, P2PSession, P2PSpectatorSession, PlayerHandle, SyncTestSession, PlayerType};
use std::net::SocketAddr;
use structopt::StructOpt;

use std::collections::HashMap;

mod systems;
use crate::systems::*;

#[derive(Default)]
pub struct TextureAtlasDictionary {
    animation_handles: HashMap<String, Handle<TextureAtlas>>
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(InputEvents::default())
        .insert_resource(TextureAtlasDictionary::default())
        .add_startup_system(setup)
        .add_system(player_state_system)
        .add_system(keyboard_input_system)
        .add_system(player_movement_system)
        .run();
}


fn load_sprite_atlas_into_texture_dictionary(
    animation_name: String, 
    asset_server: &Res<AssetServer>, 
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    mut texture_atlas_handles: &mut ResMut<TextureAtlasDictionary>,
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut texture_atlas_handles: ResMut<TextureAtlasDictionary>,
) {
    load_sprite_atlas_into_texture_dictionary(String::from("sprites/Idle.png"), &asset_server, &mut texture_atlases, &mut texture_atlas_handles, 200.0, 200.0, 8);
    load_sprite_atlas_into_texture_dictionary(String::from("sprites/Run.png"), &asset_server, &mut texture_atlases, &mut texture_atlas_handles, 200.0, 200.0, 8);
    load_sprite_atlas_into_texture_dictionary(String::from("sprites/Jump.png"), &asset_server, &mut texture_atlases, &mut texture_atlas_handles, 200.0, 200.0, 2);
    load_sprite_atlas_into_texture_dictionary(String::from("sprites/Attack1.png"), &asset_server, &mut texture_atlases, &mut texture_atlas_handles, 200.0, 200.0, 6);
    load_sprite_atlas_into_texture_dictionary(String::from("sprites/Fall.png"), &asset_server, &mut texture_atlases, &mut texture_atlas_handles, 200.0, 200.0, 2);
    
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let mut p1_transform = Transform::from_translation(Vec3::new(-100.0, 0.0, 0.0));
    p1_transform.scale.x = 2.0;
    p1_transform.scale.y = 2.0;
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handles.animation_handles["sprites/Idle.png"].clone(),
            transform:p1_transform,
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true)).insert(PlayerState::new(PlayerStateEnum::Idle, ScreenSideEnum::Left));

    let mut p2_transform = Transform::from_translation(Vec3::new(100.0, 0.0, 0.0));
    p2_transform.scale.x = 2.0;
    p2_transform.scale.y = 2.0;
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handles.animation_handles["sprites/Idle.png"].clone(),
            transform: p2_transform,
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true)).insert(PlayerState::new(PlayerStateEnum::Idle, ScreenSideEnum::Right));        
}


