use bevy::prelude::*;

use crate::{
	GameState,
    map_gen::Room,
    map_gen::random_objs::Decor,
    map_gen::random_objs::DecorType,
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

#[derive(Component)]
pub struct DecorTile;

pub struct RoomRendPlugin;

impl Plugin for RoomRendPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(GameState::Overworld)
		)
		.add_system_set(SystemSet::on_enter(GameState::Overworld)
            .with_system(create_random_room)
            .with_system(render_objects)
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
        println!("Room {},{}", x, y);
        
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

fn render_objects(
    mut commands: Commands,
    mut decor: Query<&Decor, With<Decor>>,
){
    for d in decor.iter_mut(){
        //render decor based on type
        match d.decor_type{
            //statue
            DecorType::Statue => {
                commands.spawn_bundle(SpriteBundle{
                    sprite: Sprite {
				        color: Color::GRAY,
				        custom_size: Some(Vec2::splat(TILE_SIZE)),
				        ..default()
			        },
			        transform: Transform {
				        translation: Vec3::new(d.location.x * TILE_SIZE,d.location.y * TILE_SIZE, 1.),
				        ..default()
			        },
			        visibility: Visibility {
				        is_visible: true
			        },
			        ..default()
                })
                .insert(TileCollider)
                .insert(DecorTile);
            },
            //plant
	        DecorType::Plant => {
                commands.spawn_bundle(SpriteBundle{
                    sprite: Sprite {
				        color: Color::DARK_GREEN,
				        custom_size: Some(Vec2::splat(TILE_SIZE)),
				        ..default()
			        },
			        transform: Transform {
				        translation: Vec3::new(d.location.x * TILE_SIZE,d.location.y * TILE_SIZE, 100.),
				        ..default()
			        },
			        visibility: Visibility {
				        is_visible: true
			        },
			        ..default()
                })
                .insert(TileCollider)
                .insert(DecorTile);
                println!("{},{}",d.location.x,d.location.y);
            },
            //sofa
	        DecorType::Sofa => {
                commands.spawn_bundle(SpriteBundle{
                    sprite: Sprite {
				        color: Color::PURPLE,
				        custom_size: Some(Vec2::new(TILE_SIZE*2.0, TILE_SIZE)),
				        ..default()
			        },
			        transform: Transform {
				        translation: Vec3::new(d.location.x * TILE_SIZE,d.location.y * TILE_SIZE, 1.),
				        ..default()
			        },
			        visibility: Visibility {
				        is_visible: true
			        },
			        ..default()
                })
                .insert(TileCollider)
                .insert(DecorTile);
            },
            //chair
	        DecorType::Chair => {
                commands.spawn_bundle(SpriteBundle{
                    sprite: Sprite {
				        color: Color::TEAL,
				        custom_size: Some(Vec2::splat(TILE_SIZE)),
				        ..default()
			        },
			        transform: Transform {
				        translation: Vec3::new(d.location.x * TILE_SIZE,d.location.y * TILE_SIZE, 1.),
				        ..default()
			        },
			        visibility: Visibility {
				        is_visible: true
			        },
			        ..default()
                })
                .insert(TileCollider)
                .insert(DecorTile);
            },
            //lamp
	        DecorType::Lamp => {
                commands.spawn_bundle(SpriteBundle{
                    sprite: Sprite {
				        color: Color::GOLD,
				        custom_size: Some(Vec2::splat(TILE_SIZE)),
				        ..default()
			        },
			        transform: Transform {
				        translation: Vec3::new(d.location.x * TILE_SIZE,d.location.y * TILE_SIZE, 1.),
				        ..default()
			        },
			        visibility: Visibility {
				        is_visible: true
			        },
			        ..default()
                })
                .insert(TileCollider)
                .insert(DecorTile);
            },
            //lamp
	        DecorType::Pillar => {
                commands.spawn_bundle(SpriteBundle{
                    sprite: Sprite {
				        color: Color::BLACK,
				        custom_size: Some(Vec2::splat(TILE_SIZE)),
				        ..default()
			        },
			        transform: Transform {
				        translation: Vec3::new(d.location.x * TILE_SIZE,d.location.y * TILE_SIZE, 1.),
				        ..default()
			        },
			        visibility: Visibility {
				        is_visible: true
			        },
			        ..default()
                })
                .insert(TileCollider)
                .insert(DecorTile);
            },
        }
    }
}

fn derender_all_rooms(
	mut commands: Commands,
	mut floors: Query<Entity, With<FloorTile>>,
	mut walls: Query<Entity, With<WallTile>>,
    mut doors: Query<Entity, With<DoorTile>>,
    mut keys: Query<Entity, With<KeyObject>>,
    mut decor: Query<Entity, With<DecorTile>>,
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
    for d in decor.iter_mut(){
        commands.entity(d).despawn_recursive();
    }
}