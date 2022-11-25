use bevy::{
	prelude::*,
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
	pub use_token: bool,
}

#[derive(Component)]
pub struct CombatLog {
	pub player_damage: isize,
	pub enemy_damage: isize,
	pub player_tp_change: isize,
	pub player_health_change: isize,
	pub enemy_tp_change: isize,
	pub enemy_health_change: isize,
}