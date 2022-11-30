use rand::Rng;
use bevy::{
	prelude::*,
	sprite::collide_aabb::collide,
};

use super::Room;

// Create bounds on where to put in window
pub const X_BOUND: f32 = 50.;
pub const Y_BOUND: f32 = 50.;

// Create bounds on size of room
const SIZE_LOWER_BOUND: f32 = 3.;  //7
pub const SIZE_UPPER_BOUND: f32 = 7.;  //15

pub const NUM_OF_ROOMS: i32 = 15;
const ROOM_BUFFER_SPACE: f32 = 10.;

// pub const X_BOUND: f32 = 15.;
// pub const Y_BOUND: f32 = 15.;
//const ROOM_BUFFER_SPACE: f32 = 5.;
// pub const SIZE_UPPER_BOUND: f32 = 5.;  //11

pub fn room_generator(commands: &mut Commands) -> (Vec<Vec2>, Vec<Vec2>) {
    let mut rng = rand::thread_rng();

    let mut centers = Vec::new();
    let mut sizes = Vec::new();

    let mut i = 0;
    loop {
        if i >= NUM_OF_ROOMS {
            break;
        }
        // Randomly generate location of the room
        let center = match i {
            0 => Vec3::new(
                0.,
                0.,
                1.,
            ),
            _ => Vec3::new(
                rng.gen_range::<f32,_>(-X_BOUND..X_BOUND).floor(),
                rng.gen_range::<f32,_>(-Y_BOUND..Y_BOUND).floor(),
                1.,
            ),
        };

        // Randomly generate dimensions of the room
        let size = Vec2::new(
            rng.gen_range::<f32,_>(SIZE_LOWER_BOUND..SIZE_UPPER_BOUND).floor()*2.+1.,
            rng.gen_range::<f32,_>(SIZE_LOWER_BOUND..SIZE_UPPER_BOUND).floor()*2.+1.,
        );

        // Check if this room overlaps another
        if !overlap(&center, &size, &centers, &sizes) {
            centers.push(center.clone());
            sizes.push(size.clone());
            //println!("Room {}: coord: {:?}  size:{}", i, &coord, &size);
            commands.spawn()
                .insert(Room::new(size,i, center));
                //.insert(Transform::from_translation(center));
            i += 1;
        }
    }
    // to Vec2 for delauay and bresenhams
    return (centers.iter().map(|c| Vec2::new(c.x, c.y)).collect(), sizes); 
}

pub fn overlap(
	room_center: &Vec3,
    room_size: &Vec2,
	center_list: &Vec<Vec3>,
    size_list: &Vec<Vec2>,
) -> bool {
    for i in 0..size_list.len() {
        let overlap = collide (
        	*room_center,
        	*room_size,
        	center_list[i],
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