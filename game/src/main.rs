use bevy::{
	prelude::*,
	window::PresentMode,
};

pub const RESOLUTION: f32 = 16.0/9.0;

mod credits;
mod combat;
mod tilemap;
mod movement;

use credits::CreditsPlugin;
use combat::CombatPlugin;
use tilemap::TileMapPlugin;
use movement::MovementPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum GameState{
	Overworld,
	Combat,
	Credits,
}


fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from("Game"),
			width: 1280.,
			height: 720.,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.add_state(GameState::Overworld)
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(change_state)
		.add_plugin(TileMapPlugin)
		.add_plugin(CreditsPlugin)
		.add_plugin(MovementPlugin)
		.add_plugin(CombatPlugin)
		.run();

	}


fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
	let camera = Camera2dBundle{
		..default()
	};
	commands.spawn_bundle(camera);
}

fn change_state(
	mut input: ResMut<Input<KeyCode>>,
	mut game_state: ResMut<State<GameState>>,
){
	if input.just_pressed(KeyCode::Z) {
		game_state.set(GameState::Combat).unwrap();
		 input.reset(KeyCode::Z);
	}

	if input.just_pressed(KeyCode::X) {
		game_state.set(GameState::Overworld).unwrap();
		 input.reset(KeyCode::X);
	}

	if input.just_pressed(KeyCode::C) {
		game_state.set(GameState::Credits).unwrap();
		 input.reset(KeyCode::C);
	}

}
