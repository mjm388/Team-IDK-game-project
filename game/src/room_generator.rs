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
#[derive(Component)]
pub struct PlayerStartRoom;

// Create bounds on where to put in window
const X_BOUND: f32 = 50.;  
const Y_BOUND: f32 = 50.;

// Create bounds on size of room
const SIZE_LOWER_BOUND: f32 = 3.; //7
const SIZE_UPPER_BOUND: f32 = 7.;  //15

const NUM_OF_ROOMS: i32 = 15;
const ROOM_BUFFER_SPACE: f32 = 10.;

fn generate_rooms(mut commands: Commands) {
    let mut rng = rand::thread_rng();
        
    let mut coords = Vec::new();
    let mut sizes = Vec::new();

    // prevent the room from spawning at origin ???
    coords.push(Vec3::new(0.,0.,1.0));
    sizes.push(Vec2::splat(1.));

    let spawnroom = rng.gen_range(0..NUM_OF_ROOMS);

    let mut i = 0;
    loop {
        if i >= NUM_OF_ROOMS {
            break;
        }
        // Randomly generate location of the room
        let coord = Vec3::new(
            rng.gen_range::<f32,_>(-X_BOUND..X_BOUND).floor(),
            rng.gen_range::<f32,_>(-Y_BOUND..Y_BOUND).floor(), 
            1.0,
        );

        // Randomly generate dimensions of the room
        let size = Vec2::new(
            rng.gen_range::<f32,_>(SIZE_LOWER_BOUND..SIZE_UPPER_BOUND).floor()*2.+1., 
            rng.gen_range::<f32,_>(SIZE_LOWER_BOUND..SIZE_UPPER_BOUND).floor()*2.+1.,
        );

        // Check if this room overlaps another
        if !overlap(&coord, &size, &coords, &sizes) {
            coords.push(coord.clone());
            sizes.push(size.clone());
            //println!("Room {}: coord: {:?}  size:{}", i, &coord, &size);
            let id = commands.spawn()
                .insert(Room::new(size))
                .insert(Transform::from_translation(coord))
                .id();
            if i == spawnroom {
                commands.entity(id).insert(PlayerStartRoom);
            }
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
           	Vec2::new(
                size_list[i].x+ROOM_BUFFER_SPACE, 
                size_list[i].y+ROOM_BUFFER_SPACE,
            ),
        );
        if overlap.is_some() {
            return true;
        }
    }
    false
}
    