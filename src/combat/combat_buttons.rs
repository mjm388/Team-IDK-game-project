use std::collections::HashMap;

use bevy::{
	prelude::*,
};
use rand::Rng;

use crate::{BossTrigger};
use super::{CombatOptions, CombatStats, Enemy, Player, CombatLog, CombatAgent, EnemyLog};


const COMBAT_BUTTON: Color = Color::rgb(0.15, 0.15, 0.235);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn spawn_combat_buttons(
	commands: &mut Commands,
    asset_server: &Res<AssetServer>,
   	id: CombatOptions,
    left_val: Val,
    top_val: Val,
    text: &str,
	button_size: Size<Val>,
){
	let _button_entity = 
	commands
		.spawn_bundle(ButtonBundle {
            style: Style {
                size: button_size,
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

pub fn despawn_button(mut commands: Commands, button_query: Query<Entity, With<CombatOptions>>){
    for button in button_query.iter(){
        commands.entity(button).despawn_recursive();
    }
}

pub fn button_system(
    mut interaction_query: Query<(&Interaction, &mut UiColor),(Changed<Interaction>, With<Button>)>,
	button_query: Query<(&Interaction, &CombatOptions, &Children), (Changed<Interaction>, With<Button>)>,
	mut text_query: Query<&mut Text>,
	player_query: Query<&CombatStats, With<Player>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
				*color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
				*color = COMBAT_BUTTON.into();
				for (_interaction, button, children) in button_query.iter(){
					let mut text = text_query.get_mut(children[0]).unwrap();
					match button{
						CombatOptions::Attack => {
							text.sections[0].style.font_size = 20.0;
							text.sections[0].value = "Does 1 dmg".to_string();
						}
						CombatOptions::Charge => {
							text.sections[0].style.font_size = 20.0;
							text.sections[0].value = "Does 3 dmg,\nbut costs 4 TP".to_string();
						}
						CombatOptions::Recover => {
							text.sections[0].style.font_size = 20.0;
							text.sections[0].value = "Recover 4 TP".to_string();
						}
						CombatOptions::Heal => {
							text.sections[0].style.font_size = 20.0;
							text.sections[0].value = "Heal 3 HP,\n costs 2 TP".to_string();
						}
						CombatOptions::Guard => {
							text.sections[0].style.font_size = 20.0;
							text.sections[0].value = "Invincible, reflect\n 2x dmg back,\ncosts 6 TP".to_string();
						}
						CombatOptions::AntiMage => {
							text.sections[0].style.font_size = 20.0;
							text.sections[0].value = "Does 1 dmg and\nsubtracts\n2TP from enemy,\ncosts 1 TP".to_string();
						}
						CombatOptions::Double => {
							text.sections[0].style.font_size = 20.0;
							text.sections[0].value = "Double dmg on\nnext turn and 2x\n increase TP cost,\ncosts 1 TP".to_string();
						}
						CombatOptions::Block => {
							text.sections[0].style.font_size = 20.0;
							text.sections[0].value = "0.5x dmg taken\nprevent token\ngeneration,\ncosts 2TP".to_string();
						}
						CombatOptions::Unleash => {
							let player_stats = player_query.single();
							match player_stats.token{
								1 => {
									text.sections[0].style.font_size = 20.0;
									text.sections[0].value = "Does 2 dmg,\nreceive 1 TP\nuses all tokens".to_string();
								}
								2 => {
									text.sections[0].style.font_size = 20.0;
									text.sections[0].value = "Does 6 dmg,\ntake 1 TP\nfrom enemy\nuses all tokens".to_string();
								}
								3 => {
									text.sections[0].style.font_size = 20.0;
									text.sections[0].value = "Does 10 dmg,\nrecover full TP\nuses all tokens".to_string();
								}
								_ => {
									text.sections[0].style.font_size = 20.0;
									text.sections[0].value = "No tokens!".to_string();
								}
							}
						}
					}
				}
            }
            Interaction::None => {
				*color = COMBAT_BUTTON.into();
				for (_interaction, button, children) in button_query.iter(){
					let mut text = text_query.get_mut(children[0]).unwrap();
					match button{
						CombatOptions::Attack => {
							text.sections[0].style.font_size = 30.0;
							text.sections[0].value = "Attack".to_string();
						}
						CombatOptions::Charge => {
							text.sections[0].style.font_size = 30.0;
							text.sections[0].value = "Charge".to_string();
						}
						CombatOptions::Recover => {
							text.sections[0].style.font_size = 30.0;
							text.sections[0].value = "Recover".to_string();
						}
						CombatOptions::Heal => {
							text.sections[0].style.font_size = 30.0;
							text.sections[0].value = "Heal".to_string();
						}
						CombatOptions::Guard => {
							text.sections[0].style.font_size = 30.0;
							text.sections[0].value = "Guard".to_string();
						}
						CombatOptions::AntiMage => {
							text.sections[0].style.font_size = 29.0;
							text.sections[0].value = "AntiMage".to_string();
						}
						CombatOptions::Double => {
							text.sections[0].style.font_size = 30.0;
							text.sections[0].value = "Double".to_string();
						}
						CombatOptions::Block => {
							text.sections[0].style.font_size = 30.0;
							text.sections[0].value = "Block".to_string();
						}
						CombatOptions::Unleash => {
							text.sections[0].style.font_size = 30.0;
							text.sections[0].value = "Unleash".to_string();
						}
					}
				}
            }
        }
    }
}
//This checks which button was clicked
pub fn combat_button_system2(
    query: Query<(&Interaction, &CombatOptions), (Changed<Interaction>, With<Button>)>,
	mut enemy_query: Query<&mut CombatStats, With<Enemy>>,
	mut player_query: Query<&mut CombatStats, Without<Enemy>>,
	mut qtable: Query<&mut CombatAgent>,
	mut boss_flag: Query<&mut BossTrigger>,
	mut enemy_log: Query<&mut EnemyLog>
    //mut state: ResMut<State<GameState>>,
) {
	let boss_fight = boss_flag.single_mut();
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Clicked{
			let mut log = CombatLog{
				player_damage:0,
				enemy_damage:0,
				player_tp_change: 0,
				player_health_change:0,
				enemy_tp_change: 0,
				enemy_health_change: 0,
			};
			let mut player_stats = player_query.single_mut();
			let mut enemy_stats = enemy_query.single_mut();
			let mut valid = false;
			let table = qtable.single_mut();
			let q = if boss_fight.boss_trigger{
				&table.q
			}else{
				&table.q2
			};
			let first_key = format!("{},{},{},{},{},{},{},{}", 
				player_stats.health, player_stats.tp, player_stats.token, player_stats.double, enemy_stats.health, enemy_stats.tp, enemy_stats.token, enemy_stats.double);

			let mut temp_table = HashMap::new();
			if enemy_stats.token>2 {
				temp_table.insert("Unleash".to_string(), 0);
			} else if enemy_stats.tp > if enemy_stats.double {8} else {4} {
				temp_table.insert("Charge".to_string(), 0);
			} else if enemy_stats.tp > 5 {
				temp_table.insert("Block".to_string(), 0);
			} else if enemy_stats.tp <2 {
				temp_table.insert("Recover".to_string(), 0);
			} else {
				temp_table.insert("Attack".to_string(), 0);
			}
			let inner_table = q.get(&first_key).unwrap_or(&temp_table);
			let max_value = 
				inner_table.values().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&-100000);
			let mut max_move = String::new();
			let mut second_move = String::new();
			let mut second_max = -100000;
			let mut max_found = false;
			if !inner_table.eq(&temp_table) && inner_table.keys().len()>1{
				for key in inner_table.keys() {
					if !max_found && inner_table.get(key).unwrap() == max_value {
						max_move = key.to_string();
						max_found = true;
					} else if inner_table.get(key).unwrap() > &second_max {
						second_max = *inner_table.get(key).unwrap();
						second_move = key.to_string();
					}
				};
			} else if !inner_table.eq(&temp_table) {
				for key in inner_table.keys() {
					if !max_found && inner_table.get(key).unwrap() == max_value {
						max_move = key.to_string();
						max_found = true;
					}
				};
				for second_key in temp_table.keys() {
					second_move = second_key.to_string();
				}
			} else {
				for key in inner_table.keys() {
					max_move = key.to_string();
					second_move = key.to_string();
				};
			}

			let mut rng = rand::thread_rng();
        	let player_move = rng.gen_range(1..=5);
			if player_move == 5 {
				max_move = second_move;
			}

			let mut log_text = enemy_log.single_mut();
			log_text.enemy_move = max_move.clone();

            match button{
                CombatOptions::Attack => {
					log.player_damage = if player_stats.double {2} else {1} ;
					valid = true;
					player_stats.double = false;
                }
                CombatOptions::Charge => {
					if player_stats.tp >= if player_stats.double {8} else {4} {
						log.player_tp_change -= if player_stats.double {8} else {4};
						log.player_damage = if player_stats.double {6} else {3} ;
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!");
						log_text.valid = false;
					}
                }
				CombatOptions::Recover => {
					log.player_tp_change += 4;
					valid = true;
					player_stats.double = false;
                }
				CombatOptions::Heal => {
					if player_stats.tp >= 2 {
						log.player_tp_change -= 2;
						log.player_health_change += 3;
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!");
						log_text.valid = false;
					}
                }
				CombatOptions::Guard => {
					if player_stats.tp >= 6 {
						log.player_tp_change -= 6;
						player_stats.guard = true;
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!");
						log_text.valid = false;
					}
                }
				CombatOptions::AntiMage => {
					if player_stats.tp >= if player_stats.double {2} else {1} {
						log.player_tp_change -= if player_stats.double {2} else {1};
						log.enemy_tp_change -= 2;
						log.player_damage = if player_stats.double {2} else {1};
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!");
						log_text.valid = false;
					}
                }
				CombatOptions::Double => {
					if player_stats.tp >= 1 {
						log.player_tp_change -= 1;
						player_stats.double = true;
						player_stats.tp_cost_mult = 2;
						valid = true;
					} else {
						println!("TP Low!");
						log_text.valid = false;
					}
                }
				CombatOptions::Block=> {
					if player_stats.tp >= 2 {
						log.player_tp_change -= 2;
						player_stats.block = true;
						valid = true;
						player_stats.double = false;
					} else {
						println!("TP Low!");
						log_text.valid = false;
					}
                }
                CombatOptions::Unleash => {
					match player_stats.token{
						1 => {
							log.player_damage = 2;
							log.player_tp_change += 1;
							player_stats.use_token = true;
							valid = true;
						}

						2 => {
							log.player_damage = 6;
							log.player_tp_change += 1;
							log.enemy_tp_change -= 1;
							player_stats.use_token = true;
							valid = true;
						}

						3 => {
							log.player_damage = 10;
							log.player_health_change += 20;
							player_stats.use_token = true;
							valid = true;
						}

						// this line is to avoid compile error
						_ => {log_text.valid = false;}
					}
				},
            }

			if valid{
				log_text.valid = true;
				// match ai move
				match max_move.as_str(){
					"Attack" =>{
						println!("Enemy Attacks");
						log.enemy_damage = if enemy_stats.double {2} else {1} ;
						enemy_stats.double = false;
					}
					"Charge" =>{
						println!("Enemy Charges");
						log.enemy_tp_change -= if enemy_stats.double {8} else {4};
						log.enemy_damage = if enemy_stats.double {6} else {3};
						enemy_stats.double = false;
					}
					"Recover" =>{
						println!("Enemy Recovers");
						log.enemy_tp_change += 4;
						enemy_stats.double = false;
					}
					"Heal" =>{
						println!("Enemy Heals");
						log.enemy_tp_change -= 2;
						log.enemy_health_change += 3;
						enemy_stats.double = false;
					}
					"Guard" =>{
						println!("Enemy Guards");
						log.enemy_tp_change -= 6;
						enemy_stats.guard = true;
						enemy_stats.double = false;
					}
					"AntiMage" =>{
						println!("Enemy AntiMage");
						log.enemy_tp_change -= if enemy_stats.double {2} else {1};
						log.player_tp_change -= if enemy_stats.double {4} else {2};
						log.enemy_damage += if enemy_stats.double {2} else {1};
						enemy_stats.double = false;

					}
					"Double" =>{
						println!("Enemy Double");
						log.enemy_tp_change -= 1;
						enemy_stats.double = true;
					}
					"Block" =>{
						println!("Enemy Block");
						log.enemy_tp_change -= 2;
						enemy_stats.block = true;
						enemy_stats.double = false;
					}
					"Unleash" => {
						match enemy_stats.token {
							1 => {
								println!("Enemy Unleash 1");
								log.enemy_damage = 2;
								log.enemy_tp_change += 1;
								enemy_stats.use_token = true;
							}
	
							2 => {
								println!("Enemy Unleash 2");
								log.enemy_damage = 6;
								log.enemy_tp_change += 1;
								log.player_tp_change += -1;
								enemy_stats.use_token = true;
							}
	
							3 => {
								println!("Enemy Unleash 3");
								log.enemy_damage = 10;
								log.enemy_health_change += 20;
								enemy_stats.use_token = true;
							}
	
							_ => {
								println!("Token Error");
							}
						}
					}
					_ =>{
						panic!("Shouldn't happen");
					}
				}
				
				// log calculation
				if log.player_damage > log.enemy_damage {
					if enemy_stats.block { 
						enemy_stats.health -= log.player_damage/2;
						enemy_stats.token = std::cmp::min(enemy_stats.max_token, enemy_stats.token+1);
					} else if enemy_stats.guard {
						player_stats.health -= log.player_damage*2;
						if enemy_stats.token < enemy_stats.max_token {
							enemy_stats.token += 1;
						}
					} else {
						enemy_stats.health -= log.player_damage - log.enemy_damage;
						if !player_stats.use_token {
							if player_stats.token < player_stats.max_token {
								player_stats.token += 1;
							}
						}
					}
				} else if log.enemy_damage > log.player_damage {
					if player_stats.block { 
						player_stats.health -= log.enemy_damage/2;
						player_stats.token = std::cmp::min(player_stats.max_token, player_stats.token+1);
					} else if player_stats.guard {
						enemy_stats.health -= log.enemy_damage*2;
						if player_stats.token < player_stats.max_token {
							player_stats.token += 1;
						}
					} else {
						player_stats.health -= log.enemy_damage - log.player_damage;
						if !enemy_stats.use_token {
							if enemy_stats.token < enemy_stats.max_token {
								enemy_stats.token += 1;
							}
						}
					}
				}
				player_stats.health = std::cmp::max(0, std::cmp::min(player_stats.max_health, player_stats.health + log.player_health_change));
				player_stats.tp = std::cmp::max(0, std::cmp::min(player_stats.max_tp, player_stats.tp + log.player_tp_change));
				enemy_stats.health = std::cmp::max(0, std::cmp::min(enemy_stats.max_health, enemy_stats.health + log.enemy_health_change));
				enemy_stats.tp = std::cmp::max(0, std::cmp::min(enemy_stats.max_tp, enemy_stats.tp + log.enemy_tp_change));
				if player_stats.use_token {
					player_stats.token = 0;
				}
				if enemy_stats.use_token {
					enemy_stats.token = 0;
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
				player_stats.use_token = false;
				enemy_stats.use_token = false;
			}
        }
    }
}