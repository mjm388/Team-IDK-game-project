use std::collections::HashMap;
/*
    TODO: Use Serde to store AI
*/

use std::fmt::{Display, Formatter};
use std::{fs::File, io::BufReader};

use game::mdp::{Agent,State};
use game::strategy::explore::RandomExplore;
use game::strategy::learn::QLearning;
use game::strategy::terminate::GivenIteration;
use game::{AgentTrainer, AgentT};
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy)]
struct CombatState{
    player_health: i32,
    player_max_health: i32,
	player_tp: i32,
	player_max_tp: i32,
	player_token: i32,
	player_max_token: i32,
	player_double: bool,
    enemy_health: i32,
    enemy_max_health: i32,
	enemy_tp: i32,
	enemy_max_tp: i32,
	enemy_token: i32,
	enemy_max_token: i32,
	enemy_double: bool,
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy)]
enum CombatOptions{
    Attack,
	Charge,
	Recover,
	Heal,
	Guard,
	AntiMage,
	Double,
	Block,
    Unleash,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct CombatLog {
	player_damage: i32,
    player_block: bool,
    player_guard: bool,
    player_double: bool,
    player_move: i32,
    player_use_token: bool,
    player_tp_change: i32,
    player_hp_change: i32,
    enemy_damage: i32,
    enemy_block: bool,
    enemy_guard: bool,
    enemy_double: bool,
    enemy_use_token: bool,
    enemy_tp_change: i32,
    enemy_hp_change: i32,
    valid: bool,
}

impl State for CombatState{
    type Act = CombatOptions;

    fn reward(&self) -> f64{
        //TODO: Put correct reward
        let mut p_health = self.player_health;
        let mut e_health = self.enemy_health;
        if self.player_health <= 0{
            p_health = 0;
        }
        if self.enemy_health <= 0{
            e_health = 0;
        }
        let d = (self.player_max_health-p_health)*3-(self.enemy_max_health-e_health)*3
            + if self.enemy_token==3 {50} else {0}
            + if p_health==0 {500} else {0}
            + if p_health<5 {50} else {0}
            - if self.player_token==3 {50} else {0}
            - if e_health==0 {500} else {0};
        d.into()
    }

   fn action_set(&self) -> Vec<CombatOptions>{
    vec![
        CombatOptions::Attack,
        CombatOptions::Charge,
        CombatOptions::Recover,
        CombatOptions::Heal,
        CombatOptions::Guard,
        CombatOptions::AntiMage,
        CombatOptions::Double,
        CombatOptions::Block,
        CombatOptions::Unleash,
    ]
   }

fn random_action(&self) -> Self::Act {
        let actions = self.action_set();
        let action = rand::random::<usize>() % actions.len();
        actions[action].clone()
    }
}

struct AIAgent{
    state: CombatState,
    q: HashMap<String, HashMap<String, isize>>,
}

impl Agent<CombatState>for AIAgent{
    fn current_state(&self) -> &CombatState{
        &self.state
    }

    fn act(&mut self, action: &CombatOptions) -> f64{
        let _initial_state = CombatState {
            player_health: 20,
            player_max_health: 20,
            player_tp: 10,
            player_max_tp: 10,
            player_token: 0,
            player_max_token: 3,
            player_double: false,
            enemy_health: 20,
            enemy_max_health: 20,
            enemy_tp: 10,
            enemy_max_tp: 10,
            enemy_token: 0,
            enemy_max_token: 3,
            enemy_double: false,
        };
        let mut log = CombatLog{
            player_damage: 0,
            player_block: false,
            player_guard: false,
            player_double: false,
            player_move: 0,
            player_use_token: false,
            player_tp_change: 0,
            player_hp_change: 0,
            enemy_damage: 0,
            enemy_block: false,
            enemy_guard: false,
            enemy_double: false,
            enemy_use_token: false,
            enemy_tp_change: 0,
            enemy_hp_change: 0,
            valid: false,
        };
        
        let q = &self.q;
        let first_key = format!("{},{},{},{},{},{},{},{}", 
				self.state.enemy_health, self.state.enemy_tp, self.state.enemy_token, self.state.enemy_double, self.state.player_health, self.state.player_tp, self.state.player_token, self.state.player_double);

		let mut temp_table = HashMap::new();

		if self.state.player_token>2 {
			temp_table.insert("Unleash".to_string(), 0);
		} else if self.state.player_tp > if self.state.player_double {8} else {4} {
			temp_table.insert("Charge".to_string(), 0);
		} else if self.state.player_tp > 5 {
			temp_table.insert("Block".to_string(), 0);
		} else if self.state.player_tp <2 {
			temp_table.insert("Recover".to_string(), 0);
		} else {
			temp_table.insert("Attack".to_string(), 0);
		}
		let inner_table = q.get(&first_key).unwrap_or(&temp_table);
		let max_value = inner_table.values().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&-100000);
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
        
        match max_move.as_str(){
            "Attack" =>{
                log.player_damage = if self.state.player_double {2} else {1};
                log.player_move = 1;
            }
            "Charge" =>{
                log.player_tp_change -= if self.state.player_double {8} else {4};
                log.player_damage = if self.state.player_double {6} else {3};   
                log.player_move = 2; 
            }
            "Recover" =>{
                log.player_tp_change += 4;
                log.player_move = 3;
            }
            "Heal" =>{
                log.player_tp_change += -2;
                log.player_hp_change += 3;
                log.player_move = 4;
            }
            "Guard" =>{
                log.player_tp_change += -6;
                log.player_guard = true;
                log.player_move = 5;
            }
            "AntiMage" =>{
                log.player_tp_change += if self.state.player_double {-2} else {-1};
                //log.enemy_tp_change += if self.state.player_double {-2} else {-1};
                log.enemy_tp_change += -1;
                log.player_damage += if self.state.player_double {2} else {1};    
                log.player_move = 6;            
            }
            "Double" =>{
                log.player_tp_change += -2;
                log.player_double = true;
                log.player_move = 7;
            }
            "Block" =>{
                log.player_tp_change += -2;
                log.player_block = true;
                log.player_move = 8;
            }
            "Unleash" => {
                match self.state.player_token {
                    1 => {
                        log.player_damage = 2;
                        log.player_tp_change += 1;
                        log.player_use_token = true;
                        log.player_move = 9;
                    }
                    2 => {
                        log.player_damage = 6;
                        log.player_tp_change += 1;
                        log.enemy_tp_change += -1;
                        log.player_use_token = true;
                        log.player_move = 10;
                    }
                    3 => {
                        log.player_damage = 10;
                        log.player_hp_change = 20;
                        log.player_use_token = true;
                        log.player_move = 11;
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
        // randomly assumes the player's move
        /*let mut rng = rand::thread_rng();
        let mut player_move = rng.gen_range(1..=9);
        let mut valid_move = false;
        // the following does not change the state yet but only records combat log
        while !valid_move {
            match player_move {
                // attack
                1 => {
                    log.player_damage = if self.state.player_double {2} else {1} ;
                    log.player_move = 1;
                    valid_move = true;
                }
                
                // charge
                2 => {
                    if self.state.player_tp >= if self.state.player_double {8} else {4} {
                        log.player_tp_change += if self.state.player_double {-8} else {-4};
                        log.player_damage = if self.state.player_double {6} else {3} ;
                        log.player_move = 2;
                        valid_move = true;
                    }
                }

                // recover
                3 => {
                    log.player_tp_change += 4;
                    log.player_move = 3;
                    valid_move = true;
                }

                // heal
                4 => {
                    if self.state.player_tp >= 2 {
                        log.player_tp_change += -2;
                        log.player_hp_change += 3;
                        log.player_move = 4;
                        valid_move = true;
                    }
                }

                // guard
                5 => {
                    if self.state.player_tp >= 6 {
                        log.player_tp_change += -6;
                        log.player_guard = true;
                        log.player_move = 5;
                        valid_move = true;
                    }
                }

                // anti-mage
                6 => {
                    if self.state.player_tp >= if self.state.player_double {2} else {1} {
                        log.player_tp_change += if self.state.player_double {-2} else {-1};
                        log.player_damage = if self.state.player_double {2} else {1} ;
                        log.enemy_tp_change += if self.state.player_double {-4} else {-2};
                        log.player_move = 6;
                        valid_move = true;
                    }
                }

                // double
                7 => {
                    if self.state.player_tp >= 1 {
                        log.player_tp_change += -1;
                        log.player_double = true;
                        log.player_move = 7;
                        valid_move = true;
                    }
                }

                // block
                8 => {
                    if self.state.player_tp >= 2 {
                        log.player_tp_change += -2;
                        log.player_block = true;
                        log.player_move = 8;
                        valid_move = true;
                    }
                }

                // unleash
                9 => {
                    if self.state.player_token != 0 {
                        match self.state.player_token {
                            1 => {
                                log.player_damage = 2;
                                log.player_tp_change += 1;
                                log.player_use_token = true;
                                log.player_move = 9;
                                valid_move = true;
                            }

                            2 => {
                                log.player_damage = 6;
                                log.player_tp_change += 1;
                                log.enemy_tp_change += -1;
                                log.player_use_token = true;
                                log.player_move = 10;
                                valid_move = true;
                            }

                            3 => {
                                log.player_damage = 10;
                                log.player_hp_change = 20;
                                log.player_use_token = true;
                                log.player_move = 11;
                                valid_move = true;
                            }

                            _ => {
                                println!("token error");
                                valid_move = false;
                            }
                        }
                    }
                }

                _ => {
                    valid_move = false;
                }
            }

            if !valid_move {
                player_move = rng.gen_range(1..9);
            }
        }*/

        match action {
            &CombatOptions::Attack =>{
                log.enemy_damage = if self.state.enemy_double {2} else {1};
                log.valid = true;
            },
            &CombatOptions::Charge =>{
                if self.state.enemy_tp >= if self.state.enemy_double {8} else {4}{
                    log.enemy_tp_change += if self.state.enemy_double {-8} else {-4};
                    log.enemy_damage = if self.state.enemy_double {6} else {3};
                    log.valid = true;
                }
            },
            &CombatOptions::Recover =>{
                log.enemy_tp_change += 4;
                log.valid = true;
            },
            &CombatOptions::Heal =>{
                if self.state.enemy_tp >= 2 {
                    log.enemy_tp_change += -2;
                    log.enemy_hp_change += 3;
                    log.valid = true;
                }
            },
            &CombatOptions::Guard =>{
                if self.state.enemy_tp >= 6{
                    log.enemy_tp_change += -6;
                    log.enemy_guard = true;
                    log.valid = true;
                }
            },
            &CombatOptions::AntiMage =>{
                if self.state.enemy_tp >= if self.state.enemy_double {2} else {1} {
                    log.enemy_tp_change += if self.state.enemy_double {-2} else {-1};
                    //log.player_tp_change += if self.state.enemy_double {-2} else {-1};
                    log.player_tp_change += -1;
                    log.enemy_damage = if self.state.enemy_double {2} else {1} ;
                    log.valid = true;
                }
            },
            &CombatOptions::Double =>{
                if self.state.enemy_tp >= 2 {
                    log.enemy_tp_change += -2;
                    log.enemy_double = true;
                    log.valid = true;
                }
            },
            &CombatOptions::Block =>{
                if self.state.enemy_tp >= 2 {
                    log.enemy_tp_change += -2;
                    log.enemy_block = true;
                    log.valid = true;
                }
            },
            &CombatOptions::Unleash =>{
                if self.state.enemy_token != 0 {
                    match self.state.enemy_token {
                        1 => {
                            log.enemy_damage = 2;
                            log.enemy_tp_change += 1;
                            log.enemy_use_token = true;
                            log.valid = true;
                        }

                        2 => {
                            log.enemy_damage = 6;
                            log.enemy_tp_change += 1;
                            log.player_tp_change += -1;
                            log.enemy_use_token = true;
                            log.valid = true;
                        }

                        3 => {
                            log.enemy_damage = 10;
                            log.enemy_hp_change = 20;
                            log.enemy_use_token = true;
                            log.valid = true;
                        }

                        _ => {
                            println!("Token Error");
                        }
                    }
                }
            },
        }
        // apply state changes
        if log.valid {
            if log.enemy_damage < log.player_damage {
                if log.enemy_block { 
                    self.state = CombatState {
                        player_health: std::cmp::min(self.state.player_health + log.player_hp_change, self.state.player_max_health),
                        player_tp: std::cmp::max(std::cmp::min(self.state.player_tp + log.player_tp_change, self.state.player_max_tp), 0),
                        player_token: if log.player_use_token {0} else {self.state.player_token},
                        player_double: false,
                        enemy_health: self.state.enemy_health - log.player_damage/2,
                        enemy_tp: std::cmp::max(std::cmp::min(self.state.enemy_tp + log.enemy_tp_change, self.state.enemy_max_tp), 0),
                        enemy_token: std::cmp::min(self.state.enemy_max_token, self.state.enemy_token + 1),
                        enemy_double: false,
                        ..self.state.clone()
                    };
                } else if log.enemy_guard {
                    self.state = CombatState {
                        player_health: std::cmp::min(self.state.player_health - 2*log.player_damage + log.player_hp_change, self.state.player_max_health),
                        player_tp: std::cmp::max(std::cmp::min(self.state.player_tp + log.player_tp_change, self.state.player_max_tp), 0),
                        player_token: if log.player_use_token {0} else {self.state.player_token},
                        player_double: false,
                        enemy_token: std::cmp::min(self.state.enemy_max_token, self.state.enemy_token + 1),
                        enemy_tp: std::cmp::max(std::cmp::min(self.state.enemy_tp + log.enemy_tp_change, self.state.enemy_max_tp), 0),
                        enemy_double: false,
                        ..self.state.clone()
                    };
                } else {
                    self.state = CombatState {
                        player_health: std::cmp::min(self.state.player_health + log.player_hp_change, self.state.player_max_health),
                        player_tp: std::cmp::max(std::cmp::min(self.state.player_tp + log.player_tp_change, self.state.player_max_tp), 0),
                        player_token: if log.player_use_token {0} else {std::cmp::min(self.state.player_token + 1, self.state.player_max_token)},
                        player_double: false,
                        enemy_health: std::cmp::min(self.state.enemy_health - log.player_damage + log.enemy_damage + log.enemy_hp_change, self.state.enemy_max_health),
                        enemy_tp: std::cmp::max(std::cmp::min(self.state.enemy_tp + log.enemy_tp_change, self.state.enemy_max_tp), 0),
                        enemy_token: if log.enemy_use_token {0} else {self.state.enemy_token},
                        enemy_double: log.enemy_double,
                        ..self.state.clone()
                    };
                }
            } else if log.enemy_damage > log.player_damage {
                if log.player_block { 
                    self.state = CombatState {
                        player_health: self.state.player_health - log.enemy_damage/2,
                        player_tp: std::cmp::max(std::cmp::min(self.state.player_tp + log.player_tp_change, self.state.player_max_tp), 0),
                        player_token: std::cmp::min(self.state.player_max_token, self.state.player_token + 1),
                        player_double: false,
                        enemy_health: std::cmp::min(self.state.enemy_health + log.enemy_hp_change, self.state.enemy_max_health),
                        enemy_tp: std::cmp::max(std::cmp::min(self.state.enemy_tp + log.enemy_tp_change, self.state.enemy_max_tp), 0),
                        enemy_token: if log.enemy_use_token {0} else {self.state.enemy_token},
                        enemy_double: false,
                        ..self.state.clone()
                    };
                } else if log.player_guard {
                    self.state = CombatState {
                        player_tp: std::cmp::max(std::cmp::min(self.state.player_tp + log.player_tp_change, self.state.player_max_tp), 0),
                        player_double: false,
                        player_token: std::cmp::min(self.state.player_max_token, self.state.player_token + 1),
                        enemy_health: std::cmp::min(self.state.enemy_health - 2*log.enemy_damage + log.enemy_hp_change, self.state.enemy_max_health),
                        enemy_tp: std::cmp::max(std::cmp::min(self.state.enemy_tp + log.enemy_tp_change, self.state.enemy_max_tp), 0),
                        enemy_token: if log.enemy_use_token {0} else {self.state.enemy_token},
                        enemy_double: false,
                        ..self.state.clone()
                    };
                } else {
                    self.state = CombatState {
                        player_health: std::cmp::min(self.state.player_health + log.player_damage - log.enemy_damage + log.player_hp_change, self.state.player_max_health),
                        player_tp: std::cmp::max(std::cmp::min(self.state.player_tp + log.player_tp_change, self.state.player_max_tp), 0),
                        player_token: if log.player_use_token {0} else {self.state.player_token},
                        player_double: log.player_double,
                        enemy_health: std::cmp::min(self.state.enemy_health + log.enemy_hp_change, self.state.enemy_max_health),
                        enemy_tp: std::cmp::max(std::cmp::min(self.state.enemy_tp + log.enemy_tp_change, self.state.enemy_max_tp), 0),
                        enemy_token: if log.enemy_use_token {0} else {std::cmp::min(self.state.enemy_token + 1, self.state.enemy_max_token)},
                        enemy_double: false,
                        ..self.state.clone()
                    };
                }
            } else {
                self.state = CombatState {
                    player_health: std::cmp::min(self.state.player_health + log.player_hp_change, self.state.player_max_health),
                    player_tp: std::cmp::max(std::cmp::min(self.state.player_tp + log.player_tp_change, self.state.player_max_tp), 0),
                    player_double: log.player_double,
                    enemy_health: std::cmp::min(self.state.enemy_health + log.enemy_hp_change, self.state.enemy_max_health),
                    enemy_tp: std::cmp::max(std::cmp::min(self.state.enemy_tp + log.enemy_tp_change, self.state.enemy_max_tp), 0),
                    enemy_double: log.enemy_double,
                    ..self.state.clone()
                };
            }
        }
        let reward = self.state.reward();
        if self.state.player_health <= 0 || self.state.enemy_health <= 0 {
            self.state = _initial_state;
        }
        reward
    }

    fn random_act(&mut self) -> <CombatState as State>::Act {
        let mut valid = false;
        let action = self.current_state().random_action();
        match action {
            CombatOptions::Attack =>{
                valid = true;
            },
            CombatOptions::Charge =>{
                if self.state.enemy_tp >= if self.state.enemy_double {8} else {4}{
                    valid = true;
                }
            },
            CombatOptions::Recover =>{
                valid = true;
            },
            CombatOptions::Heal =>{
                if self.state.enemy_tp >= 2 {
                    valid = true;
                }
            },
            CombatOptions::Guard =>{
                if self.state.enemy_tp >= 6{
                    valid = true;
                }
            },
            CombatOptions::AntiMage =>{
                if self.state.enemy_tp >= if self.state.enemy_double {2} else {1} {
                    valid = true;
                }
            },
            CombatOptions::Double =>{
                if self.state.enemy_tp >= 2 {
                    valid = true;
                }
            },
            CombatOptions::Block =>{
                if self.state.enemy_tp >= 2 {
                    valid = true;
                }
            },
            CombatOptions::Unleash =>{
                if self.state.enemy_token != 0 {
                    valid = true;
                }
            },
        }
        if valid {action} else {self.random_act()}
    }
}

fn read_in() -> std::io::Result<HashMap<String, HashMap<String, isize>>>{
    let f = File::open("temp_boss.json")?;
    let file = BufReader::new(f);
    let ai_state = serde_json::from_reader(file)?;
    Ok(ai_state)
}

fn main() -> Result<(), Result<(), serde_json::Error>>{
    /*Unzips from a zipfile
    let zipfile = File::open("agent.zip").unwrap();
	let mut archive = zip::ZipArchive::new(zipfile).unwrap();
	//Loops through all files in the zip for extraction
	for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
		//Gives permisions for the files extracted
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }*/
    let sparring_agent = read_in().expect("not correct");

    let initial_state = CombatState {
        player_health: 20,
        player_max_health: 20,
        player_tp: 10,
        player_max_tp: 10,
        player_token: 0,
        player_max_token: 3,
        player_double: false,
        enemy_health: 20,
        enemy_max_health: 20,
        enemy_tp: 10,
        enemy_max_tp: 10,
        enemy_token: 0,
        enemy_max_token: 3,
        enemy_double: false,
    };
    let mut trainer = AgentTrainer::new();
    let mut agent = AIAgent {
        state: initial_state.clone(),
        q: sparring_agent,
    };
    trainer.train(
        &mut agent,
        &QLearning::new(0.2, 0.01),
        &mut GivenIteration::new(700000000),
        &RandomExplore::new(),
        &initial_state,
    );

    let mut tr = AgentT::new();

    for k in trainer.q.keys() {
        for h1 in trainer.q.get(k){
            for k2 in h1.keys() {
                let v = *h1.get(k2).unwrap();
                tr.q.entry(*k)
                .or_insert_with(HashMap::new)
                .insert(*k2, v as isize);
            }
        }
    }

    let writer = File::create("fixed_agent_strong3.json").unwrap();
    
    let _ww = serde_json::to_writer(writer, &tr); 
    return Ok(())
}

impl Display for CombatState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{},{},{},{},{},{},{},{}", 
        self.player_health, self.player_tp, self.player_token, self.player_double,
        self.enemy_health, self.enemy_tp, self.enemy_token, self.enemy_double)
    }
}

impl Display for CombatOptions {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_owned())
    }
}