use bevy::prelude::*;

use crate::{
	GameState,
    map_generator::Room,
    map_generator::Line,
};

pub const M_TILE_SIZE: f32 = 6.;

#[derive(Component)]
pub struct WallTile;

#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
struct FloorTile;

#[derive(Component)]
struct MiniRoom;

pub struct MiniMapPlugin;

impl Plugin for MiniMapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(GameState::Map)
		)
		.add_system_set(SystemSet::on_enter(GameState::Map)
            .with_system(create_random_room)
            .with_system(bresenhams)
		)
		.add_system_set(SystemSet::on_exit(GameState::Map)
			.with_system(despawn_map)
        );
    }
}
fn create_random_room(
    mut commands: Commands,
    rooms: Query<&Room>,
    room_tfs: Query<&Transform, With<Room>>,
) {
    for unzip in rooms.iter().zip(room_tfs.iter()) {
        let (room, room_tf) = unzip;
        let room_size = room.size;
        let room_coord = room_tf.translation;

        commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(room_size.x as f32 * M_TILE_SIZE, room_size.y as f32 * M_TILE_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(room_coord.x * M_TILE_SIZE, room_coord.y * M_TILE_SIZE, room_coord.z),
                ..default()
            },
            ..default()
        })
        .insert(FloorTile)
		.insert(MiniRoom);
    }
}

fn despawn_map(
	mut commands: Commands,
	mut rooms: Query<Entity, With<MiniRoom>>,
){
	for e in rooms.iter_mut(){
		commands.entity(e).despawn_recursive();
	}
}

fn bresenhams(
    mut commands: Commands,
    lines: Query<&Line>,
) {
    for line in lines.iter() {
        let p1 = line.p1;
        let p2 = line.p2;
        let dx = (p2.x - p1.x).abs();
        let sx = if p1.x < p2.x {
            1
        }
        else {
            -1
        };
        let dy = -(p2.y - p1.y).abs();
        let sy = if p1.y < p2.y {
            1
        }
        else {
            -1
        };
        let mut error = dx + dy;

        let mut x0 = p1.x;
        let mut y0 = p1.y;
        let x1 = p2.x;
        let y1 = p2.y;

        loop {
            commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(5., 5.)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x0 as f32 * 6., y0 as f32 * 6., 0.),
                    ..default()
                },
                ..default()
            })
            .insert(MiniRoom);

            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2. * error;
            if e2 >= dy {
                if x0 == x1 { break; }
                error = error + dy;
                x0 = x0 + sx as f32;
            }
            if e2 <= dx {
                if y0 == y1 { break; }
                error = error + dx;
                y0 = y0 + sy as f32;
            }
        }
    }
}