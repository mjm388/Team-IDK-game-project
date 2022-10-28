use bevy::{
	prelude::*,
};
use rand::Rng;

use crate::{
	GameState,
};

#[derive(Component)]
pub struct TrainingCombatStats {
    pub health: isize,
    pub max_health: isize,
	pub tp: isize,
	pub max_tp: isize,
	pub token: isize,
	pub max_token: isize,
	pub guard: bool,
	pub double: bool,
	pub block: bool,
	pub tp_cost_mult: isize,
}

#[derive(Component)]
pub struct TrainingCombatLog {
	pub ai1_damage: isize,
	pub ai2_damage: isize,
}

#[derive(Component, PartialEq, Clone, Copy)]
pub enum CombatOptions{
	Attack,
	Charge,
	Recover,
	Heal,
	Guard,
	AntiMage,
	Double,
	Block,
}

pub struct TrainingPlugin;

impl Plugin for TrainingPlugin{
    fn build(&self, app: &mut App){
        app
		.add_system_set(SystemSet::on_update(GameState::Training)
			.with_system(training_combat_system)
			.with_system(update_ai1_text)
			.with_system(update_ai2_text)
			//.with_system(update_enemy_text)
		)
		.add_system_set(SystemSet::on_enter(GameState::Training)
			.with_system(set_training_env)
		)
		.add_system_set(SystemSet::on_exit(GameState::Training)
			.with_system(despawn_training_background)
			.with_system(despawn_ai)
		);
    }
}

#[derive(Component)]
pub struct TrainingAI1;

#[derive(Component)]
pub struct TrainingAI2;

#[derive(Component)]
pub struct Background;

fn spawn_training_background(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,    
){
    let background_handle = asset_server.load("Background_Combat.png");
    let background_atlas = TextureAtlas::from_grid(background_handle, Vec2 {x:(1280.), y: (720.)}, 1,1);
    let background_atlas_handle = texture_atlases.add(background_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: background_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
			transform: Transform {
				translation: Vec3::new(0., 0., 500.),
				..default()
			},
            ..default()
        })
        .insert(Background);
}

fn set_training_env(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,	
){
	spawn_training_background(&mut commands, &asset_server, &mut texture_atlases);
	spawn_ai1(
		&mut commands,
		&asset_server,
		&mut texture_atlases,
	);
	spawn_ai2(
		&mut commands, 
		&asset_server, 
		&mut texture_atlases, 
	);
}

pub fn spawn_ai1(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>,
	texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
){
	let stats = TrainingCombatStats{
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
		};
	let ai1_health_text = format!("Health: {}/{}", stats.health, stats.max_health);
	let ai1_tp_text = format!("\nTP: {}/{}", stats.tp, stats.max_tp);
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::BLUE,
    };
	let tp_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::GREEN,
    };
    let _enemy_health_bar = commands
		.spawn_bundle(TextBundle::from_sections([
            	TextSection::new(
                	ai1_health_text,
                	text_style,
            	),
				TextSection::new(
                	ai1_tp_text,
                	tp_text_style,
            	),
        	])
			.with_style(Style{
				position: UiRect{
					left: Val::Px(100.0),
					bottom: Val::Px(200.0),
					..default()
				},
				position_type: PositionType::Absolute,
				..default()
			}),
		)
		.insert(TrainingAI1)
		.insert(stats)
		.id();
}
//We can really just combine this function with spawn_ai1
pub fn spawn_ai2(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>,
	texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
){
	let stats = TrainingCombatStats{
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
		};
	let ai2_health_text = format!("Health: {}/{}", stats.health, stats.max_health);
	let ai2_tp_text = format!("\nTP: {}/{}", stats.tp, stats.max_tp);
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::RED,
    };
	let tp_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::GREEN,
    };

    let _ai2_health_bar = commands
		.spawn_bundle(TextBundle::from_sections([
            	TextSection::new(
                	ai2_health_text,
                	text_style,
            	),
				TextSection::new(
                	ai2_tp_text,
                	tp_text_style,
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
		.insert(TrainingAI2)
		.insert(stats)
		.id();
}

pub fn training_combat_system(
	mut ai1_query: Query<&mut TrainingCombatStats, With<TrainingAI1>>,
	mut ai2_query: Query<&mut TrainingCombatStats, Without<TrainingAI1>>,
    //mut state: ResMut<State<GameState>>,
) {
	let mut log = TrainingCombatLog{
		ai1_damage:0,
		ai2_damage:0,
	};
	let mut ai1_stats = ai1_query.single_mut();
	let mut ai2_stats = ai2_query.single_mut();

	// TODO: Implement Token manipulations
	// TODO: 
	let mut rng = rand::thread_rng();
	let mut random_num = rng.gen_range(1..9);
	let mut valid_ai_move = false;
	let mut valid = true;

	while !valid_ai_move{
		match random_num{
			1 =>{
				println!("Enemy Attacks");
				log.ai1_damage = if ai1_stats.double {10} else {5} ;
				valid_ai_move = true;
				ai1_stats.double = false;
			}
			2 =>{
				if ai1_stats.tp >= 20*ai1_stats.tp_cost_mult {
					println!("Enemy Charges");
					ai1_stats.tp -= 20*ai1_stats.tp_cost_mult;
					log.ai1_damage = if ai1_stats.double {60} else {30};
					valid_ai_move = true;
					ai1_stats.double = false;
				}else{
					valid_ai_move = false;
				}
			}
			3 =>{
				println!("Enemy Recovers");
				ai1_stats.tp = std::cmp::min(ai1_stats.tp+20, ai1_stats.max_tp);
				valid_ai_move = true;
				ai1_stats.double = false;
			}
			4 =>{
				if ai1_stats.tp >= 10 {
					println!("Enemy Heals");
					ai1_stats.tp -= 10;
					ai1_stats.health = std::cmp::min(ai1_stats.max_health, ai1_stats.health+20);
					valid_ai_move = true;
					ai1_stats.double = false;
				} else {
					valid_ai_move = false;
				}
			}
			5 =>{
				if ai1_stats.tp >= 30 {
					println!("Enemy Guards");
					ai1_stats.tp -= 30;
					ai1_stats.guard = true;
					valid_ai_move = true;
					ai1_stats.double = false;
				} else {
					valid_ai_move = false;
				}
			}
			6 =>{
				if ai1_stats.tp >= 5*ai1_stats.tp_cost_mult {
					println!("Enemy AntiMage");
					ai1_stats.tp -= 5*ai1_stats.tp_cost_mult;
					ai2_stats.tp = std::cmp::max(0, ai2_stats.tp-10);
					log.ai1_damage = if ai1_stats.double {10} else {5};
					valid_ai_move = true;
					ai1_stats.double = false;
				} else {
					valid_ai_move = false;
				}
			}
			7 =>{
				if ai1_stats.tp >= 5 {
					println!("Enemy Double");
					ai1_stats.tp -= 5;
					ai1_stats.double = true;
					ai1_stats.tp_cost_mult = 2;
					valid_ai_move = true;
				} else {
					valid_ai_move = false;
				}
			}
			8 =>{
				if ai1_stats.tp >= 10 {
					println!("Enemy Block");
					ai1_stats.tp -= 10;
					ai1_stats.block = true;
					valid_ai_move = true;
					ai1_stats.double = false;
				} else {
					valid_ai_move = false;
				}
			}
			_ =>{
				panic!("Shouldn't happen");
			}
		}
		if !valid_ai_move{
			random_num = rng.gen_range(1..9);
		}
	}	

	if valid {
		if log.ai1_damage > log.ai2_damage {
			if ai2_stats.block { 
				ai2_stats.health -= log.ai1_damage/2;
			} else if ai2_stats.guard {
				ai1_stats.health -= log.ai1_damage*2;
			} else {
				ai2_stats.health -= log.ai1_damage - log.ai2_damage;
			}
		} else if log.ai2_damage > log.ai1_damage {
			if ai1_stats.block { 
				ai1_stats.health -= log.ai2_damage/2;
			} else if ai1_stats.guard {
				ai2_stats.health -= log.ai2_damage*2;
			} else {
				ai1_stats.health -= log.ai2_damage - log.ai1_damage;
			}
		}
			if ai1_stats.health <= 0 {
				println!("You Lose!")
			} else if ai2_stats.health <= 0 {
				println!("Victory!")
			}
			ai1_stats.block = false;
			ai1_stats.guard = false;
			ai2_stats.block = false;
			ai2_stats.guard = false;
			println!("ai1 health is {}", ai1_stats.health);
			println!("ai2 health is {}", ai2_stats.health);
	}
}

pub fn update_ai1_text(
	mut ai1_text_query: Query<&mut Text, With<TrainingAI1>>,
	ai1_query: Query<&TrainingCombatStats, With<TrainingAI1>>,
){
	let ai1 = ai1_query.single();
	for mut text in &mut ai1_text_query {
		text.sections[0].value = format!("Health: {}/{}", ai1.health, ai1.max_health);
		text.sections[1].value = format!("\nTP: {}/{}", ai1.tp, ai1.max_tp);
	}
}

pub fn update_ai2_text(
	mut ai2_text_query: Query<&mut Text, With<TrainingAI2>>,
	ai2_query: Query<&TrainingCombatStats, With<TrainingAI2>>,
){
	let ai2 = ai2_query.single();
	for mut text2 in &mut ai2_text_query {
		text2.sections[0].value = format!("Health: {}/{}", ai2.health, ai2.max_health);
		text2.sections[1].value = format!("\nTP: {}/{}", ai2.tp, ai2.max_tp);
	}
}

pub fn despawn_ai(mut commands: Commands, ai1_query: Query<Entity, With<TrainingAI1>>, ai2_query: Query<Entity, With<TrainingAI2>>){
    for entity in ai1_query.iter(){
        commands.entity(entity).despawn_recursive();
    }
	for entity in ai2_query.iter(){
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_training_background(
	mut commands: Commands, background_query: Query<Entity, With<Background>>
){
	for entity in background_query.iter(){
        commands.entity(entity).despawn_recursive();
    }
}
