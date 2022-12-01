//use bevy::{prelude::*, render::render_graph::Edge};
use bevy::{prelude::*,};

use crate::{
	GameState,
    map_gen::{
        Room,
        StandPath,
    }
};

pub const M_TILE_SIZE: f32 = 6.;

#[derive(Component)]
struct MiniMapSprite;

pub struct MiniMapPlugin;

impl Plugin for MiniMapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(GameState::Map)
		)
		.add_system_set(SystemSet::on_enter(GameState::Map)
            .with_system(create_random_room)
            //.with_system(bresenhams)
            .with_system(minipaths)
		)
		.add_system_set(SystemSet::on_exit(GameState::Map)
			.with_system(despawn_map)
        );
    }
}
fn create_random_room(
    mut commands: Commands,
    rooms: Query<&Room>,
) {
    for room in rooms.iter() {
        commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(room.size.x as f32 * M_TILE_SIZE, room.size.y as f32 * M_TILE_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(room.center.x * M_TILE_SIZE, room.center.y * M_TILE_SIZE, room.center.z),
                ..default()
            },
            ..default()
        })
		.insert(MiniMapSprite);
    }
}

fn despawn_map(
	mut commands: Commands,
	mut rooms: Query<Entity, With<MiniMapSprite>>,
){
	for e in rooms.iter_mut(){
		commands.entity(e).despawn_recursive();
	}
}

fn minipaths (
    mut commands: Commands,
    paths: Query<&StandPath>
) {
    for path in paths.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(5., 5.)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(path.0.x * M_TILE_SIZE, path.0.y * M_TILE_SIZE, 0.),
                    ..default()
                },
                ..default()
            })
            .insert(MiniMapSprite);
    }
}

// fn bresenhams(
//     mut commands: Commands,
//     edges: Query<&Edge>,
// ) {
//     for line in edges.iter() {
//         let p1 = line.0;
//         let p2 = line.1;
//         let dx = (p2.x - p1.x).abs();
//         let sx = if p1.x < p2.x {
//             1
//         }
//         else {
//             -1
//         };
//         let dy = -(p2.y - p1.y).abs();
//         let sy = if p1.y < p2.y {
//             1
//         }
//         else {
//             -1
//         };
//         let mut error = dx + dy;

//         let mut x0 = p1.x;
//         let mut y0 = p1.y;
//         let x1 = p2.x;
//         let y1 = p2.y;

//         loop {
//             commands
//             .spawn_bundle(SpriteBundle {
//                 sprite: Sprite {
//                     color: Color::BLUE,
//                     custom_size: Some(Vec2::new(5., 5.)),
//                     ..default()
//                 },
//                 transform: Transform {
//                     translation: Vec3::new(x0 as f32 * 6., y0 as f32 * 6., 0.),
//                     ..default()
//                 },
//                 ..default()
//             })
//             .insert(MiniMapSprite);

//             if x0 == x1 && y0 == y1 { break; }
//             let e2 = 2. * error;
//             if e2 >= dy {
//                 if x0 == x1 { break; }
//                 error = error + dy;
//                 x0 = x0 + sx as f32;
//             }
//             if e2 <= dx {
//                 if y0 == y1 { break; }
//                 error = error + dx;
//                 y0 = y0 + sy as f32;
//             }
//         }
//     }
// }