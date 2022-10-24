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
        .add_startup_system(generate_rooms)
        .add_system_set(SystemSet::on_update(GameState::Overworld)
		)
		.add_system_set(SystemSet::on_enter(GameState::Overworld)
		)
		.add_system_set(SystemSet::on_exit(GameState::Overworld)
        );
    }
}

#[derive(Component)]
pub struct Room {
    pub size: Vec2,
}
impl Room {
	fn new(size: Vec2) -> Room {
		Room {
			size,
		}
	}
}

fn generate_rooms(mut commands: Commands) {
    let mut rng = rand::thread_rng();

        // Create bounds on where to put in window
        let x_bound = 100.;  
        let y_bound = 100.;

        // Create bounds on size of room
        let size_lower_bound = 6.;
        let size_upper_bound = 15.;  

        let num_of_rooms = 20;
        
        let mut coords = Vec::new();
        let mut sizes = Vec::new();

    let mut i = 0;
    loop {
        if i >= num_of_rooms {
            break;
        }
        // Randomly generate location of the room
        let coord = Vec3::new(rng.gen_range(-x_bound..x_bound), rng.gen_range(-y_bound..y_bound), 1.0);

        // Randomly generate dimensions of the room
        let size = Vec2::new(rng.gen_range(size_lower_bound..size_upper_bound), rng.gen_range(size_lower_bound..size_upper_bound));

        // Check if this room overlaps another
        if !overlap(&coord, &size, &coords, &sizes) {
            coords.push(coord.clone());
            sizes.push(size.clone());
            //println!("Room {}: coord: {:?}  size:{}", i, &coord, &size);
            commands.spawn()
                .insert(Room::new(size))
                .insert(Transform::from_translation(coord));
            i += 1;
        }
    }
    
}

fn overlap(
	room_pos: &Vec3,
    room_length: &Vec2,
	pos_list: &Vec<Vec3>,
    size_list: &Vec<Vec2>,
) -> bool {
    for i in 0..size_list.len() {
        let overlap = collide (
        	*room_pos,
        	*room_length,
        	pos_list[i],
           	size_list[i], 
        );
        if overlap.is_some() {
            return true;
        }
    }
    false
}
    