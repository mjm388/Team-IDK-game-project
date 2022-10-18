use std::{i64::MAX, cmp::min_by};

use bevy::{
	prelude::*,
};

use crate::{
	GameState,
};

#[derive(Component)]
pub struct CombatStats {
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

pub struct CombatLog {
	pub player_damage: isize,
	pub enemy_damage: isize,
}

#[derive(Clone, Copy)]
pub enum EnemyType{
	Square,
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

pub struct CombatPlugin;

impl Plugin for CombatPlugin{
    fn build(&self, app: &mut App){
        app
		.add_system_set(SystemSet::on_update(GameState::Combat)
			.with_system(button_system)
			.with_system(combat_button_system2)
		)
		.add_system_set(SystemSet::on_enter(GameState::Combat)
			.with_system(set_combat)

		)
		.add_system_set(SystemSet::on_exit(GameState::Combat)
			.with_system(despawn_button)
			.with_system(despawn_enemy)
			.with_system(despawn_player)
		);
    }
}


const COMBAT_BUTTON: Color = Color::rgb(0.15, 0.15, 0.235);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Background;



fn set_combat(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,	
){
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
	let enemy_translation = Vec3::new(-50., 100., 900.);
	let enemy = EnemyType::Square;
	spawn_enemy_sprite(
		&mut commands,
		&asset_server,
		&mut texture_atlases,
		enemy_translation,
		enemy,
	);
	let player_translation = Vec3::new(-450., 100., 900.);
	spawn_player_sprite(
		&mut commands, 
		&asset_server, 
		&mut texture_atlases, 
		player_translation,
	);
	/*let enemy_handle = asset_server.load("Enemy_Combat.png");
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
		});*/
	
	//The code below sets up the button positions using the spawn function
	let mut left = Val::Px(850.0);
	let mut top = Val::Px(80.0);
	let mut combat_opt_txt = "Attack";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::Attack,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(1050.0);
	top = Val::Px(80.0);
	combat_opt_txt = "Charge";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::Charge,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(850.0);
	top = Val::Px(200.0);
	combat_opt_txt = "Recover";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::Recover,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(1050.0);
	top = Val::Px(200.0);
	combat_opt_txt = "Heal";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::Heal,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(850.0);
	top = Val::Px(320.0);
	combat_opt_txt = "Guard";
	spawn_combat_buttons(
	&mut commands,
        &asset_server,
        CombatOptions::Guard,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(1050.0);
	top = Val::Px(320.0);
	combat_opt_txt = "Anti-Mage";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::AntiMage,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(850.0);
	top = Val::Px(440.0);
	combat_opt_txt = "Double";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::Double,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(1050.0);
	top = Val::Px(440.0);
	combat_opt_txt = "Block";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::Block,
        left,
        top,
        combat_opt_txt,
	);
}

fn spawn_combat_buttons(
	commands: &mut Commands,
    asset_server: &Res<AssetServer>,
   	id: CombatOptions,
    left_val: Val,
    top_val: Val,
    text: &str,
){
	let _button_entity = 
	commands
		.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(100.0)),
				position: UiRect { 
					left: left_val,
					top: top_val, 
					..default()
				},
				position_type: PositionType::Absolute,
				justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: COMBAT_BUTTON.into(),
            ..default()
        })
		.with_children(|parent| {
			parent.spawn_bundle(TextBundle::from_section(
				text,
				TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: 40.0,
					color: Color::rgb(0.9, 0.9, 0.9),
				},
			));
		})
		.insert(id)
		.id();
}

fn despawn_button(mut commands: Commands, button_query: Query<Entity, With<CombatOptions>>){
    for button in button_query.iter(){
        commands.entity(button).despawn_recursive();
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
				*color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
				*color = COMBAT_BUTTON.into();
            }
            Interaction::None => {
				*color = COMBAT_BUTTON.into();
            }
        }
    }
}
//Can probably put this with the other button system
//This checks which button was clicked
fn combat_button_system2(
    query: Query<(&Interaction, &CombatOptions), (Changed<Interaction>, With<Button>)>,
	mut enemy_query: Query<&mut CombatStats, With<Enemy>>,
	mut player_query: Query<&mut CombatStats, Without<Enemy>>,
    //mut state: ResMut<State<GameState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Clicked{
			let mut log = CombatLog{
				player_damage:0,
				enemy_damage:0,
			};
			let mut player_stats = player_query.single_mut();
			let mut enemy_stats = enemy_query.single_mut();
			let mut valid = false;
            match button{
                CombatOptions::Attack => {
					//Will put into separate functions later
					println!("Attack");
					log.player_damage = if player_stats.double {10} else {5} ;
					valid = true;
					player_stats.double = false;
                }
                CombatOptions::Charge => {
					//Will put into separate functions later
					println!("Charge");
					if player_stats.tp >= 20*player_stats.tp_cost_mult {
						player_stats.tp -= 20*player_stats.tp_cost_mult;
						log.player_damage = if player_stats.double {60} else {30} ;
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!")
					}
                }
				CombatOptions::Recover => {
					println!("Recover");
					player_stats.tp = std::cmp::min(player_stats.tp+20, player_stats.max_tp);
					valid = true;
					player_stats.double = false;
                }
				CombatOptions::Heal => {
					println!("Heal");
					if player_stats.tp >= 10 {
						player_stats.tp -= 10;
						player_stats.health = std::cmp::min(player_stats.max_health, player_stats.health+20);
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!")
					}
                }
				CombatOptions::Guard => {
					println!("Guard");
					if player_stats.tp >= 30 {
						player_stats.tp -= 30;
						player_stats.guard = true;
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!")
					}
                }
				CombatOptions::AntiMage => {
					println!("AntiMage");
					if player_stats.tp >= 5*player_stats.tp_cost_mult {
						player_stats.tp -= 5*player_stats.tp_cost_mult;
						enemy_stats.tp = std::cmp::max(0, enemy_stats.tp-10);
						log.player_damage = if player_stats.double {10} else {5};
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!")
					}
                }
				CombatOptions::Double => {
					println!("Double");
					if player_stats.tp >= 5 {
						player_stats.tp -= 5;
						player_stats.double = true;
						player_stats.tp_cost_mult = 2;
						valid = true;
					} else {
						println!("TP Low!")
					}
                }
				CombatOptions::Block=> {
					println!("Block");
					if player_stats.tp >= 10 {
						player_stats.tp -= 10;
						player_stats.block = true;
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!")
					}
                }
            }

			// TODO: Implement Token manipulations
			if valid {
				if log.player_damage > log.enemy_damage {
					if enemy_stats.block { 
						enemy_stats.health -= log.player_damage/2;
					} else if enemy_stats.guard {
						player_stats.health -= log.player_damage*2;
					} else {
						enemy_stats.health -= log.player_damage - log.enemy_damage;
					}
				} else if log.enemy_damage > log.player_damage {
					if enemy_stats.block { 
						player_stats.health -= log.enemy_damage/2;
					} else if enemy_stats.guard {
						enemy_stats.health -= log.enemy_damage*2;
					} else {
						player_stats.health -= log.enemy_damage - log.player_damage;
					}
				}
				if player_stats.health <= 0 {
					println!("You Lose!")
				} else if enemy_stats.health <= 0 {
					println!("Victory!")
				}
				player_stats.block = false;
				player_stats.guard = false;
				enemy_stats.block = false;
				enemy_stats.guard = false;
				println!("Your health is {}", player_stats.health);
				println!("Enemy health is {}", enemy_stats.health);
			}
        }
    }
}

fn spawn_enemy_sprite(
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

fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>){
    for entity in enemy_query.iter(){
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_player_sprite(
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

fn despawn_player(mut commands: Commands, player_query: Query<Entity, Without<Enemy>>){
    for entity in player_query.iter(){
        commands.entity(entity).despawn_recursive();
    }
}