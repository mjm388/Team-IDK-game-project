use bevy::{
	prelude::*,
	window::PresentMode,
};

pub const RESOLUTION: f32 = 16.0/9.0;

mod credits;
mod combat;
mod minimap;
mod movement;
mod map_gen;
mod room_renderer;
mod start_menu;


use credits::CreditsPlugin;
use combat::{CombatPlugin, CombatAgent, combat_ai::read_in, combat_ai::read_in2};
use minimap::MiniMapPlugin;
use movement::MovementPlugin;
use map_gen::RoomGenPlugin;
use room_renderer::RoomRendPlugin;
use start_menu::MainMenuPlugin;


#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum GameState{
	Overworld,
	Combat,
	Credits,
	Map,
	StartMenu,
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
			title: String::from("Game"),
			width: 1280.,
			height: 720.,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.add_state(GameState::StartMenu)//change the state from overworld to startstate to test the start menu
		//.add_state(GameState::Overworld)
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
		.run();

	}

	


fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
	
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
		if input.just_pressed(KeyCode::M) && game_state.current() != &GameState::Map{
			input.reset(KeyCode::M);
			game_state.set(GameState::Map).unwrap();
		}
		//removed map
		if input.just_pressed(KeyCode::M) && game_state.current() == &GameState::Map{
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
	}

}
