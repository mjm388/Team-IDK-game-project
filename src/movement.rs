use bevy::{
	prelude::*,
	sprite::collide_aabb::collide,utils::hashbrown::HashMap,
	//time::FixedTimestep,
};
use rand::Rng;
use crate::{
	GameState,
	BossTrigger,
	room_renderer::{TILE_SIZE, TileCollider, KeyObject, DoorTile, ViewShed}, 
	minimap::M_TILE_SIZE,
};
pub struct MovementPlugin;
impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App){
        app
			.add_startup_system(setup_player)
			.add_startup_system(initialize_key)
			.add_startup_system(init_dt)
			//.add_system_set(SystemSet::on_update(GameState::Overworld)
			//	.with_run_criteria(FixedTimestep::step(2.0 as f64))
			//	.with_system(random_encounter)
			//)
			.add_system_set(SystemSet::on_update(GameState::Overworld)
				.label(GameState::Overworld)
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
			.add_system_set(SystemSet::on_exit(GameState::Combat)
				.with_system(restart_dt)
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
pub struct DistanceTraveled {
	pub distance: f32,
}

impl DistanceTraveled {
	fn new() -> DistanceTraveled {
		DistanceTraveled{
			distance: 0.,
		}
	}
}

fn init_dt(mut commands: Commands){
	commands.spawn().insert(DistanceTraveled::new());
}

fn restart_dt(mut dist: Query<&mut DistanceTraveled,With<DistanceTraveled>>){
	let mut d = dist.single_mut();
	d.distance = 0.;
}

#[derive(Component)]
pub struct HoldingKey {
    pub held: bool,
}
impl HoldingKey {
	fn new(held: bool) -> HoldingKey {
		HoldingKey {
			held,
		}
	}
}

const PLAYER_SZ: f32 = 0.5;
const PLAYER_SPEED: f32 = 6.;

fn initialize_key(
	mut commands: Commands,
) {
	commands.spawn().insert(HoldingKey::new(false));
}

fn setup_player(
	mut commands: Commands,
) {
	// normal size player
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::rgb(0.012,0.475,0.016),
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
		.insert(ViewShed{range:215.0, viewed_tiles: HashMap::<Entity,Color>::new()})
		.insert(OverworldPlayer);

	// mini player
	commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::LIME_GREEN,
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

fn random_encounter(
	mut game_state: ResMut<State<GameState>>,
) {
	if game_state.current() == &GameState::Overworld{
		let chance = 10;	
		let mut rng = rand::thread_rng();
		let attack = rng.gen_range::<i32,_>(1..chance);

		if attack == 1 {
			game_state.set(GameState::Combat).unwrap();
		}
	}
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
	cam_transform.translation = Vec3::new(0., 0., 999.);
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
	mut _commands: Commands,
	input: Res<Input<KeyCode>>,
	mut player: Query<&mut Transform, (With<OverworldPlayer>, Without<MiniPlayer>, Without<TileCollider>)>,
	mut m_player: Query<&mut Transform, (With<MiniPlayer>, Without<OverworldPlayer>, Without<TileCollider>)>,
    time: Res<Time>,
	//windows: Res<Windows>,
	collision_tiles: Query<&Transform, (With<TileCollider>, Without<OverworldPlayer>, Without<MiniPlayer>)>,
	mut key_objects: Query<&mut Transform, (With<KeyObject>, Without<OverworldPlayer>,  Without<MiniPlayer>, Without<TileCollider>)>,
	door_objects: Query<&Transform, (With<DoorTile>, Without<OverworldPlayer>,  Without<MiniPlayer>, Without<TileCollider>, Without<KeyObject>)>,
	//fog_tiles: Query<(&Transform, Entity), (With<Fog>, Without<OverworldPlayer>,  Without<MiniPlayer>, Without<TileCollider>, Without<KeyObject>, Without<DoorTile>)>,
	mut holding: Query<&mut HoldingKey>,
	mut game_state: ResMut<State<GameState>>,
	mut boss_flag: Query<&mut BossTrigger>,
	mut dis: Query<&mut DistanceTraveled,With<DistanceTraveled>>,
){
	//let window = windows.get_primary().unwrap();
	let mut player_transform = player.single_mut();
	let mut m_player_transform = m_player.single_mut();
	let mut key_transform = key_objects.single_mut();
	let mut holding_transform = holding.single_mut();
	let mut boss_fight = boss_flag.single_mut();

	let mut x_vel = 0.;
	let mut y_vel = 0.;

	let player_move = PLAYER_SPEED * time.delta_seconds();

	let starting_pos = Vec3::new(
		player_transform.translation.x,
		player_transform.translation.y,
		player_transform.translation.z,
	);

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

	let potential_pos = Vec3::new (
		player_transform.translation.x + x_vel * TILE_SIZE,
		player_transform.translation.y + y_vel * TILE_SIZE,
		player_transform.translation.z,
	);

	//fog_collide(&player_transform.translation, &fog_tiles, commands);

	let target_x = player_transform.translation + Vec3::new(x_vel,0.,0.) * TILE_SIZE;
	if collision_check(target_x, &collision_tiles)
	{
		player_transform.translation = target_x;
		m_player_transform.translation.x += x_vel * M_TILE_SIZE;
	}

	let target_y = player_transform.translation + Vec3::new(0.,y_vel,0.) * TILE_SIZE;
	if collision_check(target_y, &collision_tiles)
	{
		player_transform.translation = target_y;
		m_player_transform.translation.y += y_vel * M_TILE_SIZE;
	}

	if holding_transform.held == false {
		let collision = collide (
			player_transform.translation,
			Vec2::splat(PLAYER_SZ*TILE_SIZE*0.9),
			key_transform.translation,
			Vec2::splat(TILE_SIZE),
		);
		if collision.is_some() {
			info!("Key is picked up");
			holding_transform.held = true;
		}
	}
	if holding_transform.held == true {
		key_transform.translation = player_transform.translation;
		if door_collide(player_transform.translation, &door_objects) {
			for _door in door_objects.iter() {
				info!("Collided with the door while holding key");
				boss_fight.boss_trigger = true;
				if game_state.current() == &GameState::Overworld{
					game_state.set(GameState::Combat).unwrap();
				}
			}
		}
	}

	if starting_pos.eq(&potential_pos) {
		return;
	}

	let d_x = (starting_pos.x - player_transform.translation.x).abs();
	let d_y = (starting_pos.y - player_transform.translation.y).abs();

	let mut d = dis.single_mut();
	d.distance += (d_x.powf(2.) + d_y.powf(2.)).sqrt();

	//println!("{}",d.distance);

	if !starting_pos.eq(&player_transform.translation) && d.distance >= 800. {
		random_encounter(game_state);
		restart_dt(dis);
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

/*fn fog_collide(
	player: &Vec3,
	fog_tiles: &Query<(&Transform, Entity), (With<Fog>, Without<OverworldPlayer>,  Without<MiniPlayer>, Without<TileCollider>, Without<KeyObject>, Without<DoorTile>)>,
	mut commands: Commands,
) {
	for (fog, s) in fog_tiles.iter() {
		let collision = collide (
			*player,
			Vec2::splat(PLAYER_SZ*TILE_SIZE*2.0),
			fog.translation,
			Vec2::splat(TILE_SIZE * 6.0),
		);
		if collision.is_some() {
			commands.entity(s).despawn_recursive();
		}
	}
}*/


fn door_collide(
	player: Vec3,
	doors: &Query<&Transform, (With<DoorTile>, Without<OverworldPlayer>,  Without<MiniPlayer>, Without<TileCollider>, Without<KeyObject>)>,
) -> bool {
	for door in doors.iter() {
		let collision = collide (
			player,
			Vec2::splat(PLAYER_SZ*TILE_SIZE*0.9),
			door.translation,
			Vec2::splat(TILE_SIZE * 1.5),
		);
		if collision.is_some() {
			return true;
		}
	}
	false
}
