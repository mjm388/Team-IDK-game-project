use bevy::{
	prelude::*,	
};
use std::convert::From;

use crate::{
	GameState,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin{
    fn build(&self, app: &mut App){
        app
		.add_system_set(SystemSet::on_update(GameState::Combat))
		.add_system_set(SystemSet::on_enter(GameState::Combat)
			.with_system(set_combat)
		)
		.add_system_set(SystemSet::on_exit(GameState::Combat));
    }
}


const BUTTON_NUM: u16 = 8;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Background;



fn set_combat(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,	
){
	//commands.spawn_bundle(Camera2dBundle::default());

	let player_handle = asset_server.load("Player_Combat.png");
	let player_atlas = TextureAtlas::from_grid(player_handle, Vec2 { x: (300.), y: (500.) }, 1, 1);
	let player_atlas_handle = texture_atlases.add(player_atlas);
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: player_atlas_handle.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform {
				translation: Vec3::new(-450., 100., 900.),
				..default()
			},
			..default()
		});

	let enemy_handle = asset_server.load("Enemy_Combat.png");
	let enemy_atlas = TextureAtlas::from_grid(enemy_handle, Vec2 { x: (300.), y: (500.) }, 1, 1);
	let enemy_atlas_handle = texture_atlases.add(enemy_atlas);
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: enemy_atlas_handle.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform {
				translation: Vec3::new(-50., 100., 900.),
				..default()
			},
			..default()
		});

	let button_handle = asset_server.load("Button_Combat.png");
	let button_atlas = TextureAtlas::from_grid(button_handle, Vec2 { x: (150.), y: (100.) }, 1, 1);
	let button_atlas_handle = texture_atlases.add(button_atlas);
	
	let mut i=0;
	while i < BUTTON_NUM {
		commands
			.spawn_bundle(SpriteSheetBundle {
				texture_atlas: button_atlas_handle.clone(),
				sprite: TextureAtlasSprite {
					index: 0,
					..default()
				},
				transform: Transform {
					translation: Vec3::new(250.+f32::from(200*(i%2)), 280.-f32::from(150*(i/2)), 900.),
					..default()
				},
				..default()
			});

		i += 1;
	}
}
