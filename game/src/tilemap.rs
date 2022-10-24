use bevy::prelude::*;

use crate::{
	GameState,
    room_generator::Room,
};

pub const TILE_SIZE: f32 = 3.;

#[derive(Component)]
pub struct WallTile;

#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
struct FloorTile;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(GameState::Overworld)
		)
		.add_system_set(SystemSet::on_enter(GameState::Overworld)
            .with_system(create_random_room)
		)
		.add_system_set(SystemSet::on_exit(GameState::Overworld)
        );
    }
}
fn create_random_room(
    mut commands: Commands, 
    rooms: Query<&Room>,
    room_tfs: Query<&Transform, With<Room>>
) {
    for unzip in rooms.iter().zip(room_tfs.iter()) {
        let (room, room_tf) = unzip;
        let room_size = room.size;
        let room_coord = room_tf.translation;

        commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(room_size.x * TILE_SIZE, room_size.y * TILE_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(room_coord.x * TILE_SIZE, room_coord.y * TILE_SIZE, room_coord.z),
                ..default()
            },
            ..default()
        })
        .insert(FloorTile);
        //.insert(TileCollider); // for testing ???
    }
}

/*
fn create_random_room(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    room: Query<>
) {
    let wall_handle = asset_server.load("BrickWall2.png");
	let wall_atlas = TextureAtlas::from_grid(wall_handle, Vec2::splat(TILE_SIZE), 1, 1);
	let wall_atlas_len = wall_atlas.textures.len();
	let wall_atlas_handle = texture_atlases.add(wall_atlas);

    let floor_handle = asset_server.load("WoodFloors2.png");
	let floor_atlas = TextureAtlas::from_grid(floor_handle, Vec2::splat(TILE_SIZE), 2, 1);
	let floor_atlas_len = floor_atlas.textures.len();
	let floor_atlas_handle = texture_atlases.add(floor_atlas);

    for 

    // Randomly generate dimensions of the room
    let x_len = rng.gen_range(size_lower_bound..size_upper_bound);
    let y_len = rng.gen_range(size_lower_bound..size_upper_bound);

    // Randomly generate location of the room
    let mut x = rng.gen_range(-x_bound..x_bound);
    let mut y = rng.gen_range(-y_bound..y_bound);

   // Draws bottom wall
   for i in 0..x_len {
        let t = Vec3::new(
            x,
            y,
            900.,
        );
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: wall_atlas_handle.clone(),
                transform: Transform {
                    translation: t,
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: i % wall_atlas_len,
                    ..default()
                },
                ..default()
            })
            .insert(WallTile)
            .insert(TileCollider);
        x = x + TILE_SIZE;
    }
    // Draws right wall
    for i in 0..y_len {
        let t = Vec3::new(
            x,
            y,
            900.,
        );
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: wall_atlas_handle.clone(),
                transform: Transform {
                    translation: t,
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: i % wall_atlas_len,
                    ..default()
                },
                ..default()
            })
            .insert(WallTile)
            .insert(TileCollider);
        y = y + TILE_SIZE;
    }
    // Draws top wall
    for i in 0..x_len {
        let t = Vec3::new(
            x,
            y,
            900.,
        );
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: wall_atlas_handle.clone(),
                transform: Transform {
                    translation: t,
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: i % wall_atlas_len,
                    ..default()
                },
                ..default()
            })
            .insert(WallTile)
            .insert(TileCollider);
        x = x - TILE_SIZE;
    }
    // Draws left wall
    for i in 0..y_len {
        let t = Vec3::new(
            x,
            y,
            900.,
        );
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: wall_atlas_handle.clone(),
                transform: Transform {
                    translation: t,
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: i % wall_atlas_len,
                    ..default()
                },
                ..default()
            })
            .insert(WallTile)
            .insert(TileCollider);
        y = y - TILE_SIZE;
    }
    x = x + TILE_SIZE;
    y = y + TILE_SIZE;
    let x_start = x;
    for i in 0..y_len-1 {
        x = x_start;
        for i in 0..x_len-1 {
            let t = Vec3::new(
                x,
                y,
                900.,
            );
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: floor_atlas_handle.clone(),
                    transform: Transform {
                        translation: t,
                        ..default()
                    },
                    sprite: TextureAtlasSprite {
                        index: i % floor_atlas_len,
                        ..default()
                    },
                    ..default()
                })
                .insert(FloorTile);
            x = x + TILE_SIZE;
        }
        y = y + TILE_SIZE;
    }
}
*/