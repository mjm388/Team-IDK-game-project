/*
    This is meant to be used outside of Bevy, unless someone knows how to implement it.
    This is a separate file that should be run on its own, with the mdp, strategy, and lib.rs files that
    Harry put.
    TODO: Update the Combat State and account for player turns
*/
use std::collections::HashMap;
use Test2::mdp::{Agent,State};
use Test2::strategy::explore::RandomExplore;
use Test2::strategy::learn::QLearning;
use Test2::strategy::terminate::GivenIteration;
use Test2::AgentTrainer;

#[derive(PartialEq, Eq, Hash, Clone)]
struct CombatState{
    pub player_health: i32,
    pub player_max_health: i32,
	pub player_tp: i32,
	pub player_max_tp: i32,
	pub player_token: i32,
	pub player_max_token: i32,
	pub player_guard: bool,
	pub player_double: bool,
	pub player_block: bool,
	pub player_tp_cost_mult: i32,
    pub player_use_token: bool,
    pub enemy_health: i32,
    pub enemy_max_health: i32,
	pub enemy_tp: i32,
	pub enemy_max_tp: i32,
	pub enemy_token: i32,
	pub enemy_max_token: i32,
	pub enemy_guard: bool,
	pub enemy_double: bool,
	pub enemy_block: bool,
	pub enemy_tp_cost_mult: i32,
    pub enemy_use_token: bool,
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
pub struct CombatLog {
	pub player_damage: i32,
	pub enemy_damage: i32,
}

impl State for CombatState{
    type Act = CombatOptions;

    fn reward(&self) -> f64{
        //TODO: Put correct reward
        let d = 1.0;
        d
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
            enemy_damage: 0,
        };
        match action{
            &CombatOptions::Attack =>{
                log.player_damage = if self.state.player_double {10} else {5};
                self.state.player_double = false;
            },
            &CombatOptions::Charge =>{
                if self.state.player_tp >= 20*self.state.player_tp_cost_mult{
                    self.state.player_tp -= 20*self.state.player_tp;
                    log.player_damage = if self.state.player_double{60} else {30};
                    self.state.player_double = false;
                }
            },
            &CombatOptions::Recover =>{
                self.state.player_tp = std::cmp::min(self.state.player_tp+20, self.state.player_max_tp);
                self.state.player_double = false;
            },
            &CombatOptions::Heal =>{
                if self.state.player_tp >= 10{
                    self.state.player_tp -= 10;
                    self.state.player_health = std::cmp::min(self.state.player_health, self.state.player_max_health);
                    self.state.player_double = false;
                }
            },
            &CombatOptions::Guard =>{
                if self.state.player_tp >= 30{
                    self.state.player_tp -= 30;
                    self.state.player_guard = true;
                    self.state.player_double = false;
                }
            },
            &CombatOptions::AntiMage =>{
                if self.state.player_tp >= 5*self.state.player_tp_cost_mult{
                    self.state.player_tp -= 5*self.state.player_tp_cost_mult;
                    self.state.enemy_tp = std::cmp::max(0, self.state.enemy_tp - 10);
                    log.player_damage = if self.state.player_double {10} else {5};
                    self.state.player_double = false;
                }
            },
            &CombatOptions::Double =>{
                if self.state.player_tp >= 5{
                    self.state.player_tp -= 5;
                    self.state.player_double = true;
                    self.state.player_tp_cost_mult = 2;
                }
            },
            &CombatOptions::Block =>{
                if self.state.player_tp >= 10{
                    self.state.player_tp -= 10;
                    self.state.player_block = true;
                    self.state.player_double;
                }
            },
            &CombatOptions::Unleash =>{
                match self.state.player_token{
                    1 => {
                        if self.state.player_tp <= self.state.player_max_tp - 10{
                            self.state.player_tp += 10;
                        }else{
                            self.state.player_tp = self.state.player_max_tp;
                        }
                        log.player_damage += 10;
                        self.state.player_double = false;
                        self.state.player_token = 0;
                        self.state.player_use_token = true;
                    }
                    2 => {
                        if self.state.player_tp <= self.state.player_max_tp - 20{
                            self.state.player_tp += 20;
                        }else{
                            self.state.player_tp = self.state.player_max_tp;
                        }
                        if self.state.player_health <= self.state.player_max_health-20{
                            self.state.player_health += 20;
                        }else{
                            self.state.player_health = self.state.player_max_health;
                        }
                        self.state.player_double = false;
                        self.state.player_token = 0;
                        self.state.player_use_token = true;
                    }
                    3 =>{
                        log.player_damage += 30;
                        self.state.player_double = false;
                        self.state.player_token = 0;
                        self.state.player_use_token = true;
                    }
                    4 => {
                        if self.state.player_tp <= self.state.player_max_tp - 40{
                            self.state.player_tp += 30;
                        }else{
                            self.state.player_tp = self.state.player_max_tp;
                        }
                        if self.state.player_health <= self.state.player_max_health - 40{
                            self.state.player_health += 40;
                        }else{
                            self.state.player_health = self.state.player_max_health;
                        }
                        self.state.player_double = false;
                        self.state.player_token = 0;
                        self.state.player_use_token = true;
                    }
                    5 => {
                        self.state.player_health = self.state.player_max_health;
                        log.player_damage += 50;
                        self.state.player_double = false;
                        self.state.player_token = 0;
                        self.state.player_use_token = true;
                    }
                    _ => println!("Something went wrong!")
                }
            },
        }
        self.state = CombatState{
            ..self.state.clone()
        };
    }
}



/*fn main() {
   TODO: Start training the AI
    
}*/
