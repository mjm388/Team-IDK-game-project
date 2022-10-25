use bevy::{
	prelude::*,
	sprite::collide_aabb::collide,
};

use crate::{
	GameState,
	room_renderer::{TILE_SIZE, TileCollider},
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App){
        app
			.add_startup_system(setup_player)
			.add_system_set(SystemSet::on_update(GameState::Overworld)
				.with_system(move_player)
				.with_system(move_camera)
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
struct OverworldPlayer;

const PLAYER_SZ: f32 = 30.;
const PLAYER_SPEED: f32 = 220.;

fn setup_player(mut commands: Commands) {
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::CRIMSON,
				custom_size: Some(Vec2::splat(PLAYER_SZ)),
				..default()
			},

			transform: Transform {
				translation: Vec3::new(-360., 0., 100.),
				..default()
			},
			visibility: Visibility {
				is_visible: false

			},
			..default()
		})
		.insert(OverworldPlayer);
}


fn collision_check(
	target_player_pos: Vec3,
	collision_tile: &Query<&Transform, (With<TileCollider>, Without<OverworldPlayer>)>,
) -> bool {
	for obs_transform in collision_tile.iter() {
		let collision = collide (
			target_player_pos,
			Vec2::splat(PLAYER_SZ*0.9),
			obs_transform.translation,
			Vec2::splat(TILE_SIZE),
		);
		if collision.is_some() {
			return false;
		}
	}
	true
}

fn activate_player(
	mut player: Query<&mut Visibility, With<OverworldPlayer>>,
){
	let mut player_vis = player.single_mut();
	player_vis.is_visible = true;
}

fn remove_player(
	mut player: Query<&mut Visibility, With<OverworldPlayer>>,
){
	let mut player_vis = player.single_mut();
	player_vis.is_visible = false;

}

fn move_camera(
	player: Query<&Transform, With<OverworldPlayer>>,
	mut camera: Query<&mut Transform, (With<Camera>,Without<OverworldPlayer>)>
){
	let player_transform = player.single();
	let mut cam_transform = camera.single_mut();
	cam_transform.translation.x = player_transform.translation.x;
	cam_transform.translation.y = player_transform.translation.y;
}


fn move_player(
	input: Res<Input<KeyCode>>,
	mut player: Query<&mut Transform, With<OverworldPlayer>>,
    time: Res<Time>,
	//mut game_state: ResMut<State<GameState>>,
	windows: Res<Windows>,
	collision_tiles: Query<&Transform, (With<TileCollider>, Without<OverworldPlayer>)>,
){
	let window = windows.get_primary().unwrap();
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

	let new_pos = Vec3::new (
		player_transform.translation.x + x_vel * time.delta_seconds(),
		player_transform.translation.y + y_vel * time.delta_seconds(),
		player_transform.translation.z,
	);
	// needs fix when map is bigger than screen
	if collision_check(new_pos, &collision_tiles)
		&& new_pos.x.abs() <= (window.width()/2.- PLAYER_SZ/2.)
		&& new_pos.y.abs() <= (window.height()/2.- PLAYER_SZ/2.)
	{
		player_transform.translation = new_pos;
	}


}
