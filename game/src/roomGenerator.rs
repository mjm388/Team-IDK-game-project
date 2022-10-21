use rand::Rng;
use bevy::{
	prelude::*,
	sprite::collide_aabb::collide,
};

use crate::{
	GameState,
};

pub struct RoomGenPlugin;

impl Plugin for RoomGenPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(generateRooms)
        .add_system_set(SystemSet::on_update(GameState::Overworld)
		)
		.add_system_set(SystemSet::on_enter(GameState::Overworld)
		)
		.add_system_set(SystemSet::on_exit(GameState::Overworld)
        );
    }
}

pub const TILE_SIZE: f32 = 3.;

#[derive(Component)]
struct Room {
    coord: Vec3,
    size: Vec2,
}

impl Room {
    fn new(coord: Vec3, size: Vec2) -> Room {
        Room {
            coord,
            size,
        }
    }
}


fn generateRooms(mut commands: Commands) {
    let mut rng = rand::thread_rng();

        // Create bounds on where to put in window
        let x_bound = 50. * TILE_SIZE;  
        let y_bound = 30. * TILE_SIZE;

        // Create bounds on size of room
        let size_lower_bound = 6;       
        let size_upper_bound = 15;     

        // windows: Res<Windows>
        // let window = windows.get_primary().unwrap();

    for i in 0..10 {
        // Randomly generate location of the room
        let coord = Vec3::new(rng.gen_range(-x_bound..x_bound), rng.gen_range(-y_bound..y_bound), 1);

        // Randomly generate dimensions of the room
        let size = Vec2::new(rng.gen_range(size_lower_bound..size_upper_bound), rng.gen_range(size_lower_bound..size_upper_bound));

        // Check if this room overlaps another
        if (!overlap(&coord, &size)) {
            commands
                .spawn()
                .insert(Room(coord, size));
            print!("Added room");
        }
        else {
            i = i - 1;
        }
    }
}

fn overlap(
	room_pos: Vec3,
    room_length: Vec2,
	existing_rooms: Query<&Room>,
) -> bool {
	for each_room in existing_rooms.iter() {
		let overlap = collide (
			room_pos,
			room_length,
			each_room.coord,
			each_room.size, 
		);
		if overlap.is_some() {
			return false;
		}
	}
	true
}
    