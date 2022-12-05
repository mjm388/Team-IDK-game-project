use bevy::{prelude::*};

use crate::{
    GameState,
    map_gen::{
        Room,
        BlockPath,
        StandPath,
        random_objs::Decor,
        random_objs::DecorType,
    }, movement::OverworldPlayer
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

#[derive(Component)]
pub struct Fog;

#[derive(Component)]
pub struct ViewShed {
    pub range: f32,
}
pub struct RoomRendPlugin;

impl Plugin for RoomRendPlugin {
    fn build(&self, app: &mut App) {
        app
        //.add_system(check_field_of_view)
        .add_startup_system(create_fog)
        .add_system_set(SystemSet::on_update(GameState::Overworld)
		)
		.add_system_set(SystemSet::on_enter(GameState::Overworld)
            .with_system(create_random_room)
            .with_system(render_objects)
            .with_system(render_fog)
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
    block_path: Query<&BlockPath>,
    stand_path: Query<&StandPath>,
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
        // println!("Room {},{}", x, y);
        
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
                            visibility: Visibility {
                                is_visible: true,
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
                        visibility: Visibility {
                            is_visible: true,
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
        if room.id == 1 {
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: door_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x as f32 + (x_size as f32 - 1.) * TILE_SIZE / 2., y as f32 + (y_size as f32 - 1.) * TILE_SIZE / 2., z + 1.),
                    ..default()
                },
                visibility: Visibility {
                    is_visible: true,
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
        if room.id == 2 {
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: key_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x as f32 + (x_size as f32 - 1.) * TILE_SIZE / 2., y as f32 + (y_size as f32 - 1.) * TILE_SIZE / 2., z + 1.),
                    ..default()
                },
                visibility: Visibility {
                    is_visible: true,
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
    // fill door holes
    for hole in block_path.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: wall_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new((hole.0.x) * TILE_SIZE, (hole.0.y) * TILE_SIZE, 0.),
                    ..default()
                },
                visibility: Visibility {
                    is_visible: true,
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                ..default()
            })
            .insert(WallTile)
            .insert(TileCollider);
    }
    for hole in stand_path.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: floor_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3::new((hole.0.x) * TILE_SIZE, (hole.0.y) * TILE_SIZE, 0.),
                    ..default()
                },
                visibility: Visibility {
                    is_visible: true,
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                ..default()
            })
            .insert(FloorTile);
    }

}

fn render_objects(
    mut commands: Commands,
    mut decor: Query<&Decor, With<Decor>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    //load all assets in first
    let wall_handle = asset_server.load("BrickWall2.png");
    let wall_atlas = TextureAtlas::from_grid(wall_handle, Vec2::splat(TILE_SIZE), 1, 1);
    //let wall_atlas_len = wall_atlas.textures.len();
    let wall_atlas_handle = texture_atlases.add(wall_atlas);

    let plant_handle = asset_server.load("Plant.png");
    let plant_atlas = TextureAtlas::from_grid(plant_handle, Vec2::new(TILE_SIZE,TILE_SIZE*2.), 1, 1);
    //let plant_atlas_len = plant_atlas.textures.len();
    let plant_atlas_handle = texture_atlases.add(plant_atlas);

    let book_handle = asset_server.load("Bookshelf.png");
    let book_atlas = TextureAtlas::from_grid(book_handle, Vec2::new(TILE_SIZE,TILE_SIZE*2.), 1, 1);
    //let plant_atlas_len = plant_atlas.textures.len();
    let book_atlas_handle = texture_atlases.add(book_atlas);

    let lamp_handle = asset_server.load("Lamp.png");
    let lamp_atlas = TextureAtlas::from_grid(lamp_handle, Vec2::new(TILE_SIZE,TILE_SIZE*2.), 1, 1);
    //let plant_atlas_len = plant_atlas.textures.len();
    let lamp_atlas_handle = texture_atlases.add(lamp_atlas);

    let chair_handle = asset_server.load("Bad_Chair.png");
    let chair_atlas = TextureAtlas::from_grid(chair_handle, Vec2::new(TILE_SIZE,TILE_SIZE*2.), 1, 1);
    //let plant_atlas_len = plant_atlas.textures.len();
    let chair_atlas_handle = texture_atlases.add(chair_atlas);

    let sofa_handle = asset_server.load("Sofa.png");
    let sofa_atlas = TextureAtlas::from_grid(sofa_handle, Vec2::new(TILE_SIZE,TILE_SIZE*2.), 1, 1);
    //let plant_atlas_len = plant_atlas.textures.len();
    let sofa_atlas_handle = texture_atlases.add(sofa_atlas);

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
				        translation: Vec3::new(d.location.x * TILE_SIZE,(d.location.y+0.5) * TILE_SIZE, 1.),
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
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: plant_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
				        custom_size: Some(Vec2::new(TILE_SIZE*1.5,TILE_SIZE*4.)),
				        ..default()
			        },
			        transform: Transform {
				        translation: Vec3::new(d.location.x * TILE_SIZE,(d.location.y+0.5) * TILE_SIZE, 1.),
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
            //sofa
	        DecorType::Sofa => {
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: sofa_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
				        custom_size: Some(Vec2::new(TILE_SIZE*1.5,TILE_SIZE*4.)),
				        ..default()
			        },
			        transform: Transform {
				        translation: Vec3::new(d.location.x * TILE_SIZE,(d.location.y+0.5) * TILE_SIZE, 1.),
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
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: chair_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
				        custom_size: Some(Vec2::new(TILE_SIZE*1.5,TILE_SIZE*4.)),
				        ..default()
			        },
			        transform: Transform {
				        translation: Vec3::new(d.location.x * TILE_SIZE,(d.location.y+0.5) * TILE_SIZE, 1.),
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
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: lamp_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
				        custom_size: Some(Vec2::new(TILE_SIZE*1.5,TILE_SIZE*4.)),
				        ..default()
			        },
			        transform: Transform {
				        translation: Vec3::new(d.location.x * TILE_SIZE,(d.location.y+0.5) * TILE_SIZE, 1.),
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
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: wall_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
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
            //bookshelf
	        DecorType::Bookshelf => {
                commands.spawn_bundle(SpriteSheetBundle{
                    texture_atlas: book_atlas_handle.clone(),
                    sprite: TextureAtlasSprite {
				        custom_size: Some(Vec2::new(TILE_SIZE*1.5,TILE_SIZE*4.)),
				        ..default()
			        },
			        transform: Transform {
				        translation: Vec3::new(d.location.x * TILE_SIZE,(d.location.y+0.5) * TILE_SIZE, 1.),
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

fn check_field_of_view(
	mut floors: Query<(Entity, &mut TextureAtlasSprite,&Transform,&mut Visibility), (With<FloorTile>)>,
	mut walls: Query<(Entity, &mut TextureAtlasSprite,&Transform,&mut Visibility), (With<WallTile>,Without<FloorTile>)>,
    
    mut decor: Query<(Entity, &mut Sprite,&Transform,&mut Visibility), (With<DecorTile>,Without<WallTile>,Without<FloorTile>)>,
    mut doors: Query<(Entity, &mut TextureAtlasSprite,&Transform,&mut Visibility), (With<DoorTile>, Without<WallTile>,Without<DecorTile>,Without<FloorTile>,Without<BlockPath>,Without<StandPath>)>,
    mut keys: Query<(Entity, &mut TextureAtlasSprite,&Transform,&mut Visibility), (With<KeyObject>, Without<WallTile>,Without<DecorTile>,Without<DoorTile>,Without<FloorTile>,Without<BlockPath>,Without<StandPath>)>,
    mut rooms: Query<(Entity, &mut Sprite,&Transform,&mut Visibility), (With<Room>, Without<WallTile>,Without<DecorTile>,Without<DoorTile>,Without<KeyObject>,Without<FloorTile>,Without<BlockPath>,Without<StandPath>)>,
    mut block_path: Query<(Entity, &mut Sprite,&Transform,&mut Visibility), (With<BlockPath>, Without<WallTile>,Without<DecorTile>,Without<FloorTile>)>,
    mut stand_path: Query<(Entity, &mut Sprite,&Transform,&mut Visibility), (With<StandPath>, Without<WallTile>,Without<DecorTile>,Without<BlockPath>,Without<FloorTile>)>,
    mut player : Query<(Entity, &Transform, &mut ViewShed), (With<OverworldPlayer>, Without<TileCollider>)>,
){
    let(_,player_transform,view_shed)=player.single_mut();
    
    for (_,floors_sprite,floors_transform,mut floors_visibility) in floors.iter_mut(){
        
        if (floors_transform.translation-player_transform.translation).length() < view_shed.range {

            floors_visibility.is_visible=true ;    
            } 
    }
    for (_,walls_sprite,walls_transform,mut walls_visibilityy) in walls.iter_mut(){
       
        if (walls_transform.translation-player_transform.translation).length() < view_shed.range {
            walls_visibilityy.is_visible=true;
              
          } 
    }
    for (_,decor_sprite,decors_transformm,mut decors_visibility) in decor.iter_mut(){
       ;
        if (decors_transformm.translation-player_transform.translation).length() < view_shed.range {
            decors_visibility.is_visible=true 
            
          } 
    }
    for (_,doors_sprite ,doors_transformm,mut doors_visibility) in doors.iter_mut(){
        
        if (doors_transformm.translation-player_transform.translation).length() < view_shed.range {

            doors_visibility.is_visible=true ;
          } 
    }
    for (_,keys_sprite,keys_transform,mut keys_visibility) in keys.iter_mut(){
       
        if (keys_transform.translation-player_transform.translation).length() < view_shed.range {

            keys_visibility.is_visible=true ;
          } 

    }
    for (_,rooms_sprite, rooms_transform,mut rooms_visibility) in rooms.iter_mut(){
        
        if (rooms_transform.translation-player_transform.translation).length() < view_shed.range {
            rooms_visibility.is_visible=true ;
           
          } 
    }
    for (_,block_sprite, block_transform,mut block_visibilityy) in block_path.iter_mut(){
        
        if (block_transform.translation-player_transform.translation).length() < view_shed.range {
            block_visibilityy.is_visible=true ;
            
          } 
    }
    for (_,stand_sprite,stand_transform,mut stand_visibility) in stand_path.iter_mut(){
       
        if (stand_transform.translation-player_transform.translation).length() < view_shed.range {

            stand_visibility.is_visible=true ;
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
    mut fog: Query<(&mut Visibility, Entity), With<Fog>>,
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
    for (mut v, _e) in fog.iter_mut() {
        v.is_visible = false;
    }
}

fn render_fog(
    mut fog: Query<(&mut Visibility, Entity), With<Fog>>,
){
    for (mut v, _e) in fog.iter_mut() {
        v.is_visible = true;
    }
}

fn create_fog (
    mut commands: Commands,
) {
    for x in -60..60 {
        for y in -60..60 {
            commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::DARK_GRAY,
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 2.),
                    ..default()
                },
                visibility: Visibility {
                    is_visible: false
                },
                ..default()
            })
            .insert(Fog);
        }
    }
}