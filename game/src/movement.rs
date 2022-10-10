use bevy::prelude::*;


pub struct MovementPlugin;

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App){
        app
            .add_startup_system(setup_player)
            .add_system(move_player);
    }
}

#[derive(Component)]
struct Player;

const PLAYER_SZ: f32 = 100.;

fn setup_player(mut commands: Commands) {
	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::SEA_GREEN,
				custom_size: Some(Vec2::splat(PLAYER_SZ)),
				..default()
			},
			..default()
		})
		.insert(Player);
}


fn move_player(
	input: Res<Input<KeyCode>>,
	mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
){
	let mut player_transform = player.single_mut();

	let mut x_vel = 0.;
	let mut y_vel = 0.;

	if input.pressed(KeyCode::A) {
		x_vel -= 5.;
	}

	if input.pressed(KeyCode::D) {
		x_vel += 5.;
	}

	if input.pressed(KeyCode::W) {
		y_vel += 5.;
	}

	if input.pressed(KeyCode::S) {
		y_vel -= 5.;
	}

	player_transform.translation.x += x_vel * time.delta_seconds();
	player_transform.translation.y += y_vel * time.delta_seconds();
}
