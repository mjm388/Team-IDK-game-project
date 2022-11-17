use bevy::prelude::*;

use crate::{
	GameState,
    map_gen::Room,
};

pub const TILE_SIZE: f32 = 40.;

#[derive(Component)]
pub struct WallTile;

#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
struct FloorTile;

#[derive(Component)]
pub struct KeyObject;

#[derive(Component)]
pub struct DoorTile;

pub struct RoomRendPlugin;

impl Plugin for RoomRendPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(GameState::Overworld)
		)
		.add_system_set(SystemSet::on_enter(GameState::Overworld)
            .with_system(create_random_room)
		)
		.add_system_set(SystemSet::on_exit(GameState::Overworld)
			.with_system(derender_all_rooms)
        );
    }
}
fn create_random_room(
    mut commands: Commands,
    rooms: Query<&Room>,
    asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let wall_handle = asset_server.load("BrickWall2.png");
    let wall_atlas = TextureAtlas::from_grid(wall_handle, Vec2::splat(TILE_SIZE), 1, 1);
    let wall_atlas_len = wall_atlas.textures.len();
    let wall_atlas_handle = texture_atlases.add(wall_atlas);

    let floor_handle = asset_server.load("WoodFloors2.png");
    let floor_atlas = TextureAtlas::from_grid(floor_handle, Vec2::splat(TILE_SIZE), 2, 1);
    let floor_atlas_len = floor_atlas.textures.len();
    let floor_atlas_handle = texture_atlases.add(floor_atlas);

    let door_handle = asset_server.load("Door.png");
    let door_atlas = TextureAtlas::from_grid(door_handle, Vec2::splat(TILE_SIZE * 2.), 1, 1);
    //  let door_atlas_len = door_atlas.textures.len();
    let door_atlas_handle = texture_atlases.add(door_atlas);

    let key_handle = asset_server.load("Key.png");
    let key_atlas = TextureAtlas::from_grid(key_handle, Vec2::splat(TILE_SIZE), 1, 1);
    //  let key_atlas_len = key_atlas.textures.len();
    let key_atlas_handle = texture_atlases.add(key_atlas);
    
    for room in rooms.iter() {
        let x = (room.center.x-(room.size.x-1.)/2.) * TILE_SIZE;
        let y = (room.center.y-(room.size.y-1.)/2.) * TILE_SIZE;
        let z = 0.;
        
        let x_size = room.size.x as usize;
        let y_size = room.size.y as usize;

        //floor
        for i in 0..x_size {
            for j in 0..y_size {
                if i == 0 || j == 0 || i == x_size-1 || j == y_size-1 {
                    // doors
                    if i == x_size/2 || j == y_size/2 {
                        
                    }
                    // walls
                    else {
                        commands
                        .spawn_bundle(SpriteSheetBundle {
                            texture_atlas: wall_atlas_handle.clone(),
                            transform: Transform {
                                translation: Vec3::new(x+i as f32 * TILE_SIZE, y+j as f32 * TILE_SIZE, z),
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
                    }
                }
                // floors
                else {
                    commands
                    .spawn_bundle(SpriteSheetBundle {
                        texture_atlas: floor_atlas_handle.clone(),
                        transform: Transform {
                            translation: Vec3::new(x+i as f32 * TILE_SIZE, y+j as f32 * TILE_SIZE, z),
                            ..default()
                        },
                        sprite: TextureAtlasSprite {
                            index: i % floor_atlas_len,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(FloorTile);
                }
                
            }
        }
        if room.id == 13 {
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: door_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x as f32 + (x_size as f32 - 1.) * TILE_SIZE / 2., y as f32 + (y_size as f32 - 1.) * TILE_SIZE / 2., z + 1.),
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                ..default()
            })
            .insert(DoorTile);
            info!("Door added: {},{}", x / TILE_SIZE, y / TILE_SIZE);
        }
        if room.id == 14 {
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: key_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x as f32 + (x_size as f32 - 1.) * TILE_SIZE / 2., y as f32 + (y_size as f32 - 1.) * TILE_SIZE / 2., z + 1.),
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                ..default()
            })
            .insert(KeyObject);
            info!("Key added: {},{}", x / TILE_SIZE, y / TILE_SIZE);
        }
    }
}

fn derender_all_rooms(
	mut commands: Commands,
	mut floors: Query<Entity, With<FloorTile>>,
	mut walls: Query<Entity, With<WallTile>>,
    mut doors: Query<Entity, With<DoorTile>>,
    mut keys: Query<Entity, With<KeyObject>>,
){
	for e in floors.iter_mut(){
		commands.entity(e).despawn_recursive();
	}
	for e in walls.iter_mut(){
		commands.entity(e).despawn_recursive();
	}
    for e in doors.iter_mut(){
        commands.entity(e).despawn_recursive();
    }
    for e in keys.iter_mut(){
        commands.entity(e).despawn_recursive();
    }
}