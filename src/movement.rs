use bevy::{
	prelude::*,
	sprite::collide_aabb::collide,
};

use crate::{
	GameState,
	room_renderer::{TILE_SIZE, TileCollider, KeyObject}, 
	minimap::M_TILE_SIZE,
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
				.with_system(put_back_camera)
			)
			.add_system_set(SystemSet::on_exit(GameState::Overworld)
				.with_system(remove_player)
				.with_system(adjust_camera)
			)
			.add_system_set(SystemSet::on_enter(GameState::Map)
				.with_system(activate_m_player)
			)
			.add_system_set(SystemSet::on_exit(GameState::Map)
				.with_system(remove_m_player)
			);
    }
}

#[derive(Component)]
pub struct OverworldPlayer;

#[derive(Component)]
pub struct MiniPlayer;

#[derive(Component)]
pub struct HoldingKey;

const PLAYER_SZ: f32 = 0.5;
const PLAYER_SPEED: f32 = 6.;

fn setup_player(
	mut commands: Commands,
) {
	// normal size player
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::CRIMSON,
				custom_size: Some(Vec2::splat(PLAYER_SZ * TILE_SIZE)),
				..default()
			},
			transform: Transform {
				translation: Vec3::new(0., 0., 100.),
				..default()
			},
			visibility: Visibility {
				is_visible: false
			},
			..default()
		})
		.insert(OverworldPlayer);

	// mini player
	commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(2. * M_TILE_SIZE, 2. * M_TILE_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 100.),
                ..default()
            },
			visibility: Visibility {
				is_visible: false
			},
            ..default()
        })
        .insert(MiniPlayer);
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

fn activate_m_player(
	mut player: Query<&mut Visibility, With<MiniPlayer>>,
){
	let mut player_vis = player.single_mut();
	player_vis.is_visible = true;
}

fn remove_m_player(
	mut player: Query<&mut Visibility, With<MiniPlayer>>,
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

fn adjust_camera (	// Stores current position of camera
	mut camera: Query<&mut Transform, (With<Camera>,Without<OverworldPlayer>)>
){
	let mut cam_transform = camera.single_mut();
	cam_transform.translation = Vec3::new(0., 0., 999.)
}

fn put_back_camera (	// Resets camera position back to player
	player: Query<&Transform, With<OverworldPlayer>>,
	mut camera: Query<&mut Transform, (With<Camera>,Without<OverworldPlayer>)>
){
	let player_transform = player.single();
	let mut cam_transform = camera.single_mut();
	cam_transform.translation = player_transform.translation;
}

fn move_player(
	input: Res<Input<KeyCode>>,
	mut player: Query<&mut Transform, (With<OverworldPlayer>, Without<MiniPlayer>, Without<TileCollider>)>,
	mut m_player: Query<&mut Transform, (With<MiniPlayer>, Without<OverworldPlayer>, Without<TileCollider>)>,
    time: Res<Time>,
	//windows: Res<Windows>,
	collision_tiles: Query<&Transform, (With<TileCollider>, Without<OverworldPlayer>, Without<MiniPlayer>)>,
	key_objects: Query<&Transform, (With<KeyObject>, Without<OverworldPlayer>,  Without<MiniPlayer>)>,
){
	//let window = windows.get_primary().unwrap();
	let mut player_transform = player.single_mut();
	let mut m_player_transform = m_player.single_mut();

	let mut x_vel = 0.;
	let mut y_vel = 0.;

	let player_move = PLAYER_SPEED * time.delta_seconds();

	if input.pressed(KeyCode::A) {
		x_vel -= player_move;
	}

	if input.pressed(KeyCode::D) {
		x_vel += player_move;
	}

	if input.pressed(KeyCode::W) {
		y_vel += player_move;
	}

	if input.pressed(KeyCode::S) {
		y_vel -= player_move;
	}

	if (x_vel.abs() + y_vel.abs()) > player_move {
		x_vel *= 0.70710678118;
		y_vel *= 0.70710678118;
	}

	let new_pos = Vec3::new (
		player_transform.translation.x + x_vel * TILE_SIZE,
		player_transform.translation.y + y_vel * TILE_SIZE,
		player_transform.translation.z,
	);
	if collision_check(new_pos, &collision_tiles)
	{
		if key_pickup(new_pos, &key_objects) {
			for _key in key_objects.iter() {
				//key.translation = new_pos;
				info!("Key is picked up");
			}
		}
		player_transform.translation = new_pos;
		m_player_transform.translation = Vec3::new(
			m_player_transform.translation.x + x_vel * M_TILE_SIZE, 
			m_player_transform.translation.y + y_vel * M_TILE_SIZE, 
			m_player_transform.translation.z,
		)
	}
}

fn collision_check(
	target_player_pos: Vec3,
	collision_tile: &Query<&Transform, (With<TileCollider>, Without<OverworldPlayer>,  Without<MiniPlayer>)>,
) -> bool {
	for obs_transform in collision_tile.iter() {
		let collision = collide (
			target_player_pos,
			Vec2::splat(PLAYER_SZ*TILE_SIZE*0.9),
			obs_transform.translation,
			Vec2::splat(TILE_SIZE),
		);
		if collision.is_some() {
			return false;
		}
	}
	true
}

fn key_pickup(
	player: Vec3,
	keys: &Query<&Transform, (With<KeyObject>, Without<OverworldPlayer>,  Without<MiniPlayer>)>,
) -> bool {
	for key in keys.iter() {
		let collision = collide (
			player,
			Vec2::splat(PLAYER_SZ*TILE_SIZE*0.9),
			key.translation,
			Vec2::splat(TILE_SIZE / 1.5),
		);
		if collision.is_some() {
			return true;
		}
	}
	false
}
