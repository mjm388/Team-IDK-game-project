use bevy::{
	prelude::*,
	window::PresentMode,
};



use std::fs::File;
use std::io;
use std::fs;
use std::path::Path;

pub const RESOLUTION: f32 = 16.0/9.0;

mod credits;
mod combat;
mod minimap;
mod movement;
mod map_gen;
mod room_renderer;
mod start_menu;
mod tutorial;
mod loss;
mod victory;


use credits::CreditsPlugin;
use combat::{CombatPlugin, CombatAgent, combat_ai::read_in, combat_ai::read_in2,};
use minimap::MiniMapPlugin;
use movement::MovementPlugin;
use map_gen::RoomGenPlugin;
use room_renderer::RoomRendPlugin;
use start_menu::MainMenuPlugin;
use tutorial::TutorialPlugin;
use loss::LossPlugin;
use victory::VictoryPlugin;
use room_renderer::RoomWasCreated;
//use room_renderer::DecorWasCreated;

#[derive(SystemLabel)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum GameState{
	Overworld,
	Combat,
	Credits,
	Map,
	StartMenu,
	Tutorial,
	Loss,
	Victory,
}

#[derive(Component)]
pub struct BossTrigger{
	pub boss_trigger: bool,
}

#[derive(Component)]
struct Camera;


fn main() {
	App::new()
	.insert_resource(WindowDescriptor {
		title: String::from("Luigo's Haunted House Tour"),
		width: 1280.,
		height: 720.,
		present_mode: PresentMode::Fifo,
		..default()
		})
		.insert_resource(RoomWasCreated(false))
		//.insert_resource(DecorWasCreated(false))
		.add_state(GameState::StartMenu)
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(change_state)
		.add_plugin(MainMenuPlugin)
		.add_plugin(RoomGenPlugin)
		.add_plugin(RoomRendPlugin)
		.add_plugin(CreditsPlugin)
		.add_plugin(MovementPlugin)
		.add_plugin(MiniMapPlugin)
		.add_plugin(CombatPlugin)
		.add_plugin(TutorialPlugin)
		.add_plugin(LossPlugin)
		.add_plugin(VictoryPlugin)
		.run();

	}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
	if !(Path::new("final_boss_agent.json").exists() && Path::new("final_mob_agent.json").exists()) {
		let zipfile = File::open("final_ai2.zip").unwrap();
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
		}
	}
	commands.spawn_bundle(Camera2dBundle{
		transform: Transform {
			translation: Vec3::new(-360., 0., 100.),
			..default()
		},
		..default()
	}).insert(Camera);
	let boss_flag = BossTrigger{
		boss_trigger: false,
	};
	commands.spawn()
	.insert(boss_flag);
	
	let qtable = CombatAgent{q: read_in().expect("not correct"), q2: read_in2().expect("not correct")};	
	commands.spawn()
	.insert(qtable);
}

fn change_state(
	mut input: ResMut<Input<KeyCode>>,
	mut game_state: ResMut<State<GameState>>,
){
	if game_state.current() != &GameState::Credits{
		//switch to combat
		if input.just_pressed(KeyCode::X) && game_state.current() == &GameState::Overworld{
			input.reset(KeyCode::X);
			game_state.set(GameState::Combat).unwrap();
		}
		//switch to overworld
		if input.just_pressed(KeyCode::X) && game_state.current() == &GameState::Combat{
			input.reset(KeyCode::X);
			game_state.set(GameState::Overworld).unwrap();
		}
		//roll credits
		if input.just_pressed(KeyCode::C) && game_state.current() != &GameState::Combat{
			input.reset(KeyCode::C);
			game_state.set(GameState::Credits).unwrap();
		}
		//display map
		if input.just_pressed(KeyCode::M) && game_state.current() != &GameState::Map && game_state.current() != &GameState::Combat{
			input.reset(KeyCode::M);
			game_state.set(GameState::Map).unwrap();
		}
		//removed map
		if input.just_pressed(KeyCode::M) && game_state.current() == &GameState::Map && game_state.current() != &GameState::Combat{
			input.reset(KeyCode::M);
			game_state.set(GameState::Overworld).unwrap();
		}
		/*if input.just_pressed(KeyCode::G) && game_state.current() != &GameState::StartMenu{
			input.reset(KeyCode::G);
			game_state.set(GameState::StartMenu).unwrap();
		}*/
		if input.just_pressed(KeyCode::G) && game_state.current() == &GameState::StartMenu{
			input.reset(KeyCode::G);
			game_state.set(GameState::Overworld).unwrap();
		} 
		if input.just_pressed(KeyCode::I) && game_state.current() == &GameState::StartMenu{
			input.reset(KeyCode::I);
			game_state.set(GameState::Tutorial).unwrap();
		} 
		if input.just_pressed(KeyCode::I) && game_state.current() == &GameState::Tutorial{
			input.reset(KeyCode::I);
			game_state.set(GameState::StartMenu).unwrap();
		} 
	}

}
