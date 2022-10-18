use bevy::prelude::*;

use crate::{
	GameState,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App){
        app
			.add_startup_system(setup_player)
			.add_system_set(SystemSet::on_update(GameState::Overworld)
				.with_system(move_player)
			)
			.add_system_set(SystemSet::on_enter(GameState::Overworld)
				.with_system(activate_player)
			)
			.add_system_set(SystemSet::on_exit(GameState::Overworld)
				.with_system(remove_player)
			);
    }
}

#[derive(Component)]
struct Player;

const PLAYER_SZ: f32 = 100.;
const PLAYER_SPEED: f32 = 100.;

fn setup_player(mut commands: Commands) {
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::CRIMSON,
				custom_size: Some(Vec2::splat(PLAYER_SZ)),
				..default()
			},
			visibility: Visibility {
				is_visible: false
			},
			..default()
		})
		.insert(Player);
}

fn activate_player(
	mut player: Query<&mut Visibility, With<Player>>,
){
	let mut player_vis = player.single_mut();
	player_vis.is_visible = true;
}

fn remove_player(
	mut player: Query<&mut Visibility, With<Player>>,
){
	let mut player_vis = player.single_mut();
	player_vis.is_visible = false;
}


fn move_player(
	input: Res<Input<KeyCode>>,
	mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
	mut game_state: ResMut<State<GameState>>,
){
	let mut player_transform = player.single_mut();

	let mut x_vel = 0.;
	let mut y_vel = 0.;

	if input.pressed(KeyCode::A) {
		x_vel -= PLAYER_SPEED;
	}

	if input.pressed(KeyCode::D) {
		x_vel += PLAYER_SPEED;
	}

	if input.pressed(KeyCode::W) {
		y_vel += PLAYER_SPEED;
	}

	if input.pressed(KeyCode::S) {
		y_vel -= PLAYER_SPEED;
	}

	player_transform.translation.x += x_vel * time.delta_seconds();
	player_transform.translation.y += y_vel * time.delta_seconds();
}
