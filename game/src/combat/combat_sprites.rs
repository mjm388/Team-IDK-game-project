use bevy::{
	prelude::*, text::Text2dBounds,
};

use super::{Enemy, Player};
use crate::combat::{EnemyType, CombatStats};

#[derive(Component)]
pub struct PlayerHealthBar;

#[derive(Component)]
pub struct EnemyHealthBar;

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

	let enemy_health_text = format!("Health: {}/{}", stats.health, stats.max_health);
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::RED,
    };

    let enemy_health_bar = commands
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
	
	let health_text = format!("Health: {}/{}", stats.health, stats.max_health);
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::GREEN,
    };
    //let box_size = Vec2::new(200.0, 100.0);
    //let box_position = Vec2::new(-425., -250.0);

    let health_bar = commands
		.spawn_bundle(TextBundle::from_sections([
            	TextSection::new(
                	health_text,
                	text_style,
            	),
        	])
			.with_style(Style{
				position: UiRect{
					left: Val::Px(-500.0),
					bottom: Val::Px(200.0),
					..default()
				},
				..default()
			}),
		)
		.insert(PlayerHealthBar)
        .id();
	
	
	/*let health_bar = commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(health_text, text_style),
            text_2d_bounds: Text2dBounds {
                size: box_size,
            },
            transform: Transform::from_xyz(
                box_position.x - box_size.x / 2.0,
                box_position.y + box_size.y / 2.0,
                900.0,
            ),
			visibility: Visibility { is_visible: (true) },
            ..default()
        })
		.insert(PlayerHealthBar)
        .id();*/

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

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>, player_health: Query<Entity, With<PlayerHealthBar>>){
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