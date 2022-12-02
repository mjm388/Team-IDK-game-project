use bevy::{
	prelude::*,
};

use super::{Enemy, Player, EnemyLog};
use crate::combat::{EnemyType, CombatStats};

#[derive(Component)]
pub struct PlayerHealthBar;

#[derive(Component)]
pub struct EnemyHealthBar;

#[derive(Component)]
pub struct LogText;

pub fn spawn_enemy_sprite(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>,
	texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
	translation_val: Vec3,
	enemy_type: EnemyType,
){
	let stats = match enemy_type {
		EnemyType::Mob => CombatStats{
			health: 20,
			max_health: 20,
			tp: 10,
			max_tp: 10,
			token: 0,
			max_token: 3,
			guard: false,
			double: false,
			block: false,
			tp_cost_mult: 1,
			use_token: false,
		},
		EnemyType::Boss => CombatStats{
			health: 20,
			max_health: 20,
			tp: 10,
			max_tp: 10,
			token: 0,
			max_token: 3,
			guard: false,
			double: false,
			block: false,
			tp_cost_mult: 1,
			use_token: false,
		},
	};
	let enemy_handle = match enemy_type{
		EnemyType::Mob => asset_server.load("ghostEnemy.png"),
		EnemyType::Boss => asset_server.load("ghostBoss.png"),
	};
	let enemy_atlas = TextureAtlas::from_grid(enemy_handle, Vec2 {x:(300.), y: (500.)}, 1,1);
	let enemy_atlas_handle = texture_atlases.add(enemy_atlas);

	let enemy_health_text = format!("Health: {}/{}", stats.health, stats.max_health);
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::RED,
    };

    let _enemy_health_bar = commands
		.spawn_bundle(TextBundle::from_sections([
            	TextSection::new(
                	enemy_health_text,
                	text_style,
            	),
        	])
			.with_style(Style{
				position: UiRect{
					left: Val::Px(500.0),
					bottom: Val::Px(200.0),
					..default()
				},
				position_type: PositionType::Absolute,
				..default()
			}),
		)
		.insert(EnemyHealthBar)
        .id();

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

pub fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>, enemy_health: Query<Entity, With<EnemyHealthBar>>){
    for entity in enemy_query.iter(){
        commands.entity(entity).despawn_recursive();
    }
	for entity in enemy_health.iter(){
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
			health: 20,
			max_health: 20,
			tp: 10,
			max_tp: 10,
			token: 0,
			max_token: 3,
			guard: false,
			double: false,
			block: false,
			tp_cost_mult: 1,
			use_token: false,
	};
	let player_handle = asset_server.load("Player_Combat.png");
	let player_atlas = TextureAtlas::from_grid(player_handle, Vec2 {x:(300.), y: (500.)}, 1,1);
	let player_atlas_handle = texture_atlases.add(player_atlas);
	
	let health_text = format!("Health: {}/{}", stats.health, stats.max_health);
	let tp_text = format!("\nTP: {}/{}", stats.tp, stats.max_tp);
	let token_text = format!("\nToken: {}/{}\n\n", stats.token, stats.max_token);
	let log_text = format!("You Encountered An Enemy!");
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::GREEN,
    };
	let tp_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::BLUE,
    };

	let token_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::YELLOW,
    };

	let log_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::BLACK,
    };
    //let box_size = Vec2::new(200.0, 100.0);
    //let box_position = Vec2::new(-425., -250.0);

    let _health_bar = commands
		.spawn_bundle(TextBundle::from_sections([
            	TextSection::new(
                	health_text,
                	text_style,
            	),
				TextSection::new(
                	tp_text,
                	tp_text_style,
            	),
				TextSection::new(
					token_text, 
					token_text_style,
				),
				TextSection::new(
					log_text, 
					log_text_style,
				),
        	])
			.with_text_alignment(TextAlignment::TOP_CENTER)
			.with_style(Style{
				position: UiRect{
					left: Val::Px(100.0),
					bottom: Val::Px(100.0),
					..default()
				},
				position_type: PositionType::Absolute,
				..default()
			}),
		)
		.insert(PlayerHealthBar)
        .id();
	
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

pub fn despawn_player(
	mut commands: Commands, 
	player_query: Query<Entity, With<Player>>, 
	player_health: Query<Entity, With<PlayerHealthBar>>,
){
    for entity in player_query.iter(){
        commands.entity(entity).despawn_recursive();
    }
	for entity in player_health.iter(){
		commands.entity(entity).despawn_recursive();
	}
}

pub fn update_player_text(
	mut player_text_query: Query<&mut Text, With<PlayerHealthBar>>,
	player_query: Query<&CombatStats, With<Player>>,
){
	let player = player_query.single();
	for mut text in &mut player_text_query {
		text.sections[0].value = format!("Health: {}/{}", player.health, player.max_health);
		text.sections[1].value = format!("\nTP: {}/{}", player.tp, player.max_tp);
		text.sections[2].value = format!("\nToken: {}/{}\n\n", player.token, player.max_token);
	}
}
pub fn update_enemy_text(
	mut enemy_text_query: Query<&mut Text, With<EnemyHealthBar>>,
	enemy_query: Query<&CombatStats, With<Enemy>>,
){
	let enemy = enemy_query.single();
	for mut text in &mut enemy_text_query {
		text.sections[0].value = format!("Health: {}/{}", enemy.health, enemy.max_health);
	}
}

pub fn update_log_text(
	mut log_text_query: Query<&mut Text, With<PlayerHealthBar>>,
	enemy_log: Query<&EnemyLog>,
) {
	let log = enemy_log.single();
	if log.valid{
		for mut text in &mut log_text_query {
			text.sections[3].value = format!("Enemy {}",log.enemy_move);
		}
	} else {
		for mut text in &mut log_text_query {
			text.sections[3].value = format!("Resources not enough");
		}
	}
}