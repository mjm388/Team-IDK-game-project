use bevy::prelude::*;

use crate::{
	GameState,
    room_generator::Room,
    movement::OverworldPlayer,
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
    player: Query<&Transform, With<OverworldPlayer>>,
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
    mut player: Query<Entity, With<MiniRoom>>,
){
	for e in rooms.iter_mut(){
		commands.entity(e).despawn_recursive();
	}
}