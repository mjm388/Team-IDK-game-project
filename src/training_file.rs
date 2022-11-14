/*
    This is meant to be used outside of Bevy, unless someone knows how to implement it.
    This is a separate file that should be run on its own, with the mdp, strategy, and lib.rs files that
    Harry put.
    TODO: Update the Combat State and account for player turns
*/
use std::collections::HashMap;
use game::mdp::{Agent,State};
use game::strategy::explore::RandomExplore;
use game::strategy::learn::QLearning;
use game::strategy::terminate::GivenIteration;
use game::AgentTrainer;
use rand::Rng;

#[derive(PartialEq, Eq, Hash, Clone)]
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

#[derive(PartialEq, Eq, Hash, Clone)]
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
        let d = (self.player_max_health-self.player_health)*15-(self.enemy_max_health-self.enemy_health)*5+(self.enemy_token-self.player_token)-(self.enemy_max_tp-self.enemy_tp);
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
}

struct AIAgent{
    state: CombatState,
}

impl Agent<CombatState>for AIAgent{
    fn current_state(&self) -> &CombatState{
        &self.state
    }

    fn act(&mut self, action: &CombatOptions){
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
        
        // randomly assumes the player's move
        let mut rng = rand::thread_rng();
        let mut player_move = rng.gen_range(1..9);
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
                    if self.state.player_tp >= if self.state.player_double {4} else {2} {
                        log.player_tp_change += if self.state.player_double {-4} else {-2};
                        log.player_hp_change = if self.state.player_double {-4} else {-2};
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
                                log.player_hp_change = 50;
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
        }

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
                if self.state.enemy_tp >= if self.state.enemy_double {4} else {2} {
                    log.enemy_tp_change += if self.state.enemy_double {-4} else {-2};
                    log.enemy_hp_change = if self.state.enemy_double {-4} else {-2};
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
                    log.enemy_damage = if self.state.enemy_double {2} else {1} ;
                    log.player_tp_change += if self.state.player_double {-4} else {-2};
                    log.valid = true;
                }
            },
            &CombatOptions::Double =>{
                if self.state.enemy_tp >= 1 {
                    log.enemy_tp_change += -1;
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
                        player_health: self.state.player_health - 2*log.player_damage,
                        player_tp: std::cmp::max(std::cmp::min(self.state.player_tp + log.player_tp_change, self.state.player_max_tp), 0),
                        player_token: if log.player_use_token {0} else {self.state.player_token},
                        player_double: false,
                        enemy_tp: std::cmp::max(std::cmp::min(self.state.enemy_tp + log.enemy_tp_change, self.state.enemy_max_tp), 0),
                        enemy_double: false,
                        ..self.state.clone()
                    };
                } else {
                    self.state = CombatState {
                        player_tp: std::cmp::max(std::cmp::min(self.state.player_tp + log.player_tp_change, self.state.player_max_tp), 0),
                        player_token: if log.player_use_token {0} else {std::cmp::min(self.state.player_token + 1, self.state.player_max_token)},
                        player_double: false,
                        enemy_health: self.state.enemy_health - log.player_damage - log.enemy_damage,
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
                        enemy_tp: std::cmp::max(std::cmp::min(self.state.enemy_tp + log.enemy_tp_change, self.state.enemy_max_tp), 0),
                        enemy_token: if log.enemy_use_token {0} else {self.state.enemy_token},
                        enemy_double: false,
                        ..self.state.clone()
                    };
                } else if log.player_guard {
                    self.state = CombatState {
                        player_tp: std::cmp::max(std::cmp::min(self.state.player_tp + log.player_tp_change, self.state.player_max_tp), 0),
                        player_double: false,
                        enemy_health: self.state.enemy_health - 2*log.enemy_damage,
                        enemy_tp: std::cmp::max(std::cmp::min(self.state.enemy_tp + log.enemy_tp_change, self.state.enemy_max_tp), 0),
                        enemy_token: if log.enemy_use_token {0} else {self.state.enemy_token},
                        enemy_double: false,
                        ..self.state.clone()
                    };
                } else {
                    self.state = CombatState {
                        player_health: self.state.player_health - log.enemy_damage - log.player_damage,
                        player_tp: std::cmp::max(std::cmp::min(self.state.player_tp + log.player_tp_change, self.state.player_max_tp), 0),
                        player_token: if log.player_use_token {0} else {self.state.player_token},
                        player_double: log.player_double,
                        enemy_tp: std::cmp::max(std::cmp::min(self.state.enemy_tp + log.enemy_tp_change, self.state.enemy_max_tp), 0),
                        enemy_token: if log.enemy_use_token {0} else {std::cmp::min(self.state.enemy_token + 1, self.state.enemy_max_token)},
                        enemy_double: false,
                        ..self.state.clone()
                    };
                }
            } else {
                self.state = CombatState {
                    player_tp: std::cmp::max(std::cmp::min(self.state.player_tp + log.player_tp_change, self.state.player_max_tp), 0),
                    player_double: log.player_double,
                    enemy_tp: std::cmp::max(std::cmp::min(self.state.enemy_tp + log.enemy_tp_change, self.state.enemy_max_tp), 0),
                    enemy_double: log.enemy_double,
                    ..self.state.clone()
                };
            }
        }
    }

    fn random_act(&mut self) -> <CombatState as State>::Act {
        let action = self.current_state().random_action();
        self.act(&action);
        action
    }
}



fn main() {
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
    };
    trainer.train(
        &mut agent,
        &QLearning::new(0.2, 0.01, 2.),
        &mut GivenIteration::new(100000000),
        &RandomExplore::new(),
    );
    println!("ss");
}
