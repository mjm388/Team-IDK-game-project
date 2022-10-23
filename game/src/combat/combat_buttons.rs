use bevy::{
	prelude::*,
};

use super::{CombatOptions, CombatStats, Enemy, CombatLog};

const COMBAT_BUTTON: Color = Color::rgb(0.15, 0.15, 0.235);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);


pub fn spawn_combat_buttons(
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

pub fn despawn_button(mut commands: Commands, button_query: Query<Entity, With<CombatOptions>>){
    for button in button_query.iter(){
        commands.entity(button).despawn_recursive();
    }
}

pub fn button_system(
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
pub fn combat_button_system2(
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