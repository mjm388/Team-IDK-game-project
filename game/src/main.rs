use bevy::{
	prelude::*,
	window::PresentMode,
};

pub const RESOLUTION: f32 = 16.0/9.0;

mod credits;
use credits::CreditsPlugin;
mod combat;
use combat::CombatPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum GameState{
	Overworld,
	Combat,
	Credits,
}

mod tilemap;
use tilemap::TileMapPlugin;

mod movement;
use movement::MovementPlugin;

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
