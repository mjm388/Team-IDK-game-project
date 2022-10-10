use bevy::{
	prelude::*,
	window::PresentMode,
};

mod credits;
use credits::CreditsPlugin;

mod tilemap;
use tilemap::TileMapPlugin;

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from("Hello World!"),
			width: 1280.,
			height: 720.,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_plugin(TileMapPlugin)
		.add_plugin(CreditsPlugin)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(Camera2dBundle::default());
}
