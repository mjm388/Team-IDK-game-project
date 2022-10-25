use bevy::{
	prelude::*,
};

use super::{Enemy, Player};
use crate::combat::{EnemyType, CombatStats};

pub fn spawn_enemy_sprite(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>,
	texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
	translation_val: Vec3,
	enemy_type: EnemyType,
){
	let stats = match enemy_type {
		EnemyType::Square => CombatStats{
			health: 50,
			max_health: 50,
			tp: 50,
			max_tp: 50,
			token: 2,
			max_token: 5,
			guard: false,
			block: false,
			double: false,
			tp_cost_mult: 1,
		},
	};
	let enemy_handle = match enemy_type{
		EnemyType::Square => asset_server.load("Enemy_Combat.png"),
	};
	let enemy_atlas = TextureAtlas::from_grid(enemy_handle, Vec2 {x:(300.), y: (500.)}, 1,1);
	let enemy_atlas_handle = texture_atlases.add(enemy_atlas);
	let _enemy_sprite = commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: enemy_atlas_handle.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform {
				translation: translation_val,
				..default()
			},
			..default()
		})
		.insert(Enemy)
		.insert(stats)
		.id();
}

pub fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>){
    for entity in enemy_query.iter(){
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_player_sprite(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>,
	texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
	translation_val: Vec3,
){
	let stats = CombatStats{
			health: 100,
			max_health: 100,
			tp: 50,
			max_tp: 50,
			token: 2,
			max_token: 5,
			guard: false,
			double: false,
			block: false,
			tp_cost_mult: 1,
	};
	let player_handle = asset_server.load("Player_Combat.png");
	let player_atlas = TextureAtlas::from_grid(player_handle, Vec2 {x:(300.), y: (500.)}, 1,1);
	let player_atlas_handle = texture_atlases.add(player_atlas);
	let _player_sprite = commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: player_atlas_handle.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform {
				translation: translation_val,
				..default()
			},
			..default()
		})
		.insert(Player)
		.insert(stats)
		.id();
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>){
    for entity in player_query.iter(){
        commands.entity(entity).despawn_recursive();
    }
}