use bevy::{prelude::*, utils::HashMap};

use crate::{
    map_gen::{random_objs::Decor, random_objs::DecorType, BlockPath, Room, StandPath},
    movement::OverworldPlayer,
    GameState,
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

/*#[derive(Component)]
pub struct Fog;*/

#[derive(Component)]
pub struct RoomWasCreated(pub bool);

#[derive(Component)]
pub struct ViewShed {
    pub range: f32,
    pub viewed_tiles: HashMap<Entity, Color>,
}
pub struct RoomRendPlugin;

impl Plugin for RoomRendPlugin {
    fn build(&self, app: &mut App) {
        app
            /* .add_startup_system(
                render_objects.after("room_render")
                )
            .add_startup_system_set(
                SystemSet::new()
                .after("map_gen")
                .with_system(create_random_room)
                .with_system(render_objects),
                )*/
            //.add_startup_system(create_random_room)
            //.add_startup_system(render_objects)
            //.add_system(check_field_of_view)
            //.add_startup_system(create_fog)
            .add_system_set(
                SystemSet::on_update(GameState::Overworld).with_system(check_field_of_view), //.with_system(create_random_room)
                                                                                             //.with_system(render_objects)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Overworld)
                    .with_system(create_random_room)
                    .with_system(render_objects), //.with_system(render_fog)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Overworld).with_system(derender_all_rooms),
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
    mut room_was_created: ResMut<RoomWasCreated>,
) {
    if room_was_created.0 {
        return;
    }
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
    let key_atlas_len = key_atlas.textures.len();
    let key_atlas_handle = texture_atlases.add(key_atlas);

    for room in rooms.iter() {
        let x = (room.center.x - (room.size.x - 1.) / 2.) * TILE_SIZE;
        let y = (room.center.y - (room.size.y - 1.) / 2.) * TILE_SIZE;
        let z = 0.;
        // println!("Room {},{}", x, y);

        let x_size = room.size.x as usize;
        let y_size = room.size.y as usize;

        //floor
        for i in 0..x_size {
            for j in 0..y_size {
                if i == 0 || j == 0 || i == x_size - 1 || j == y_size - 1 {
                    // doors
                    if i == x_size / 2 || j == y_size / 2 {
                    }
                    // walls
                    else {
                        commands
                            .spawn_bundle(SpriteSheetBundle {
                                texture_atlas: wall_atlas_handle.clone(),
                                transform: Transform {
                                    translation: Vec3::new(
                                        x + i as f32 * TILE_SIZE,
                                        y + j as f32 * TILE_SIZE,
                                        z,
                                    ),
                                    ..default()
                                },
                                visibility: Visibility {
                                    is_visible: false,
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
                                translation: Vec3::new(
                                    x + i as f32 * TILE_SIZE,
                                    y + j as f32 * TILE_SIZE,
                                    z,
                                ),
                                ..default()
                            },
                            visibility: Visibility {
                                is_visible: false,
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
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: door_atlas_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            x as f32 + (x_size as f32 - 1.) * TILE_SIZE / 2.,
                            y as f32 + (y_size as f32 - 1.) * TILE_SIZE / 2.,
                            z + 1.,
                        ),
                        ..default()
                    },
                    visibility: Visibility {
                        is_visible: false,
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
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: key_atlas_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            x as f32 + (x_size as f32 - 1.) * TILE_SIZE / 2.,
                            y as f32 + (y_size as f32 - 1.) * TILE_SIZE / 2.,
                            z + 1.,
                        ),
                        ..default()
                    },
                    visibility: Visibility {
                        is_visible: false,
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
                    is_visible: false,
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
                    is_visible: false,
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
        room_was_created.0 = true;
}

fn render_objects(mut commands: Commands, mut decor: Query<&Decor, With<Decor>>) {
    for d in decor.iter_mut() {
        //render decor based on type
        match d.decor_type {
            //statue
            DecorType::Statue => {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GRAY,
                            custom_size: Some(Vec2::splat(TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(
                                d.location.x * TILE_SIZE,
                                d.location.y * TILE_SIZE,
                                1.,
                            ),
                            ..default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..default()
                    })
                    .insert(TileCollider)
                    .insert(DecorTile);
            }
            //plant
            DecorType::Plant => {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::DARK_GREEN,
                            custom_size: Some(Vec2::splat(TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(
                                d.location.x * TILE_SIZE,
                                d.location.y * TILE_SIZE,
                                1.,
                            ),
                            ..default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..default()
                    })
                    .insert(TileCollider)
                    .insert(DecorTile);
                println!("{},{}", d.location.x, d.location.y);
            }
            //sofa
            DecorType::Sofa => {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::PURPLE,
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(
                                d.location.x * TILE_SIZE,
                                d.location.y * TILE_SIZE,
                                1.,
                            ),
                            ..default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..default()
                    })
                    .insert(TileCollider)
                    .insert(DecorTile);
            }
            //chair
            DecorType::Chair => {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::TEAL,
                            custom_size: Some(Vec2::splat(TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(
                                d.location.x * TILE_SIZE,
                                d.location.y * TILE_SIZE,
                                1.,
                            ),
                            ..default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..default()
                    })
                    .insert(TileCollider)
                    .insert(DecorTile);
            }
            //lamp
            DecorType::Lamp => {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::GOLD,
                            custom_size: Some(Vec2::splat(TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(
                                d.location.x * TILE_SIZE,
                                d.location.y * TILE_SIZE,
                                1.,
                            ),
                            ..default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..default()
                    })
                    .insert(TileCollider)
                    .insert(DecorTile);
            }
            //lamp
            DecorType::Pillar => {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLACK,
                            custom_size: Some(Vec2::splat(TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(
                                d.location.x * TILE_SIZE,
                                d.location.y * TILE_SIZE,
                                1.,
                            ),
                            ..default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..default()
                    })
                    .insert(TileCollider)
                    .insert(DecorTile);
            }
            DecorType::Bookshelf => {
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::OLIVE,
                            custom_size: Some(Vec2::splat(TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(
                                d.location.x * TILE_SIZE,
                                d.location.y * TILE_SIZE,
                                1.,
                            ),
                            ..default()
                        },
                        visibility: Visibility { is_visible: false },
                        ..default()
                    })
                    .insert(TileCollider)
                    .insert(DecorTile);
            }
        }
    }
}

fn check_field_of_view(
    mut floors: Query<
        (Entity, &mut TextureAtlasSprite, &Transform, &mut Visibility),
        (With<FloorTile>),
    >,
    mut walls: Query<
        (Entity, &mut TextureAtlasSprite, &Transform, &mut Visibility),
        (With<WallTile>, Without<FloorTile>),
    >,

    mut decor: Query<
        (Entity, &mut Sprite, &Transform, &mut Visibility),
        (With<DecorTile>, Without<WallTile>, Without<FloorTile>),
    >,
    mut doors: Query<
        (Entity, &mut TextureAtlasSprite, &Transform, &mut Visibility),
        (
            With<DoorTile>,
            Without<WallTile>,
            Without<DecorTile>,
            Without<FloorTile>,
            Without<BlockPath>,
            Without<StandPath>,
        ),
    >,
    mut keys: Query<
        (Entity, &mut TextureAtlasSprite, &Transform, &mut Visibility),
        (
            With<KeyObject>,
            Without<WallTile>,
            Without<DecorTile>,
            Without<DoorTile>,
            Without<FloorTile>,
            Without<BlockPath>,
            Without<StandPath>,
        ),
    >,
    mut rooms: Query<
        (Entity, &mut Sprite, &Transform, &mut Visibility),
        (
            With<Room>,
            Without<WallTile>,
            Without<DecorTile>,
            Without<DoorTile>,
            Without<KeyObject>,
            Without<FloorTile>,
            Without<BlockPath>,
            Without<StandPath>,
        ),
    >,
    mut block_path: Query<
        (Entity, &mut Sprite, &Transform, &mut Visibility),
        (
            With<BlockPath>,
            Without<WallTile>,
            Without<DecorTile>,
            Without<FloorTile>,
        ),
    >,
    mut stand_path: Query<
        (Entity, &mut Sprite, &Transform, &mut Visibility),
        (
            With<StandPath>,
            Without<WallTile>,
            Without<DecorTile>,
            Without<BlockPath>,
            Without<FloorTile>,
        ),
    >,
    mut player: Query<
        (Entity, &Transform, &mut ViewShed),
        (With<OverworldPlayer>, Without<TileCollider>),
    >,
) {
    let (_, player_transform, mut view_shed) = player.single_mut();

    for (Entity, mut floors_sprite, floors_transform, mut floors_visibility) in floors.iter_mut() {
        let search_res = view_shed.viewed_tiles.get(&Entity);
        let is_inside_field_of_view = (floors_transform.translation - player_transform.translation)
            .length()
            < view_shed.range;
        if is_inside_field_of_view {
            if search_res.is_none() {
                view_shed.viewed_tiles.insert(Entity, floors_sprite.color);
            } else {
                floors_sprite.color = *search_res.unwrap();
            }
            floors_visibility.is_visible = is_inside_field_of_view;
        } else {
            if search_res.is_some() {
                floors_visibility.is_visible = true;
                floors_sprite.color = Color::GRAY;
            }
        }
    }
    for (Entity, mut walls_sprite, walls_transform, mut walls_visibilityy) in walls.iter_mut() {
        let search_res = view_shed.viewed_tiles.get(&Entity);
        let is_inside_field_of_view =
            (walls_transform.translation - player_transform.translation).length() < view_shed.range;
        if is_inside_field_of_view {
            if search_res.is_none() {
                view_shed.viewed_tiles.insert(Entity, walls_sprite.color);
            } else {
                walls_sprite.color = *search_res.unwrap();
            }
            walls_visibilityy.is_visible = is_inside_field_of_view;
        } else {
            if search_res.is_some() {
                walls_visibilityy.is_visible = true;
                walls_sprite.color = Color::GRAY;
            }
        }
    }
    for (Entity, mut decor_sprite, decors_transformm, mut decors_visibility) in decor.iter_mut() {
        let search_res = view_shed.viewed_tiles.get(&Entity);
        let is_inside_field_of_view =
            (decors_transformm.translation - player_transform.translation).length()
                < view_shed.range;
        if is_inside_field_of_view {
            if search_res.is_none() {
                view_shed.viewed_tiles.insert(Entity, decor_sprite.color);
            } else {
                decor_sprite.color = *search_res.unwrap();
            }
            decors_visibility.is_visible = is_inside_field_of_view;
        } else {
            if search_res.is_some() {
            decors_visibility.is_visible = true;
            decor_sprite.color = Color::GRAY;
            }
        }
    }
    for (Entity, mut doors_sprite, doors_transformm, mut doors_visibility) in doors.iter_mut() {
        let search_res = view_shed.viewed_tiles.get(&Entity);
        let is_inside_field_of_view = (doors_transformm.translation - player_transform.translation)
            .length()
            < view_shed.range;
        if is_inside_field_of_view {
            if search_res.is_none() {
                view_shed.viewed_tiles.insert(Entity, doors_sprite.color);
            } else {
                doors_sprite.color = *search_res.unwrap();
            }
            doors_visibility.is_visible = is_inside_field_of_view;
        } else {
            if search_res.is_some() {
            doors_visibility.is_visible = true;
            doors_sprite.color = Color::GRAY;
            }
        }
    }
    for (Entity, mut keys_sprite, keys_transform, mut keys_visibility) in keys.iter_mut() {
        let search_res = view_shed.viewed_tiles.get(&Entity);
        let is_inside_field_of_view =
            (keys_transform.translation - player_transform.translation).length() < view_shed.range;
        if is_inside_field_of_view {
            if search_res.is_none() {
                view_shed.viewed_tiles.insert(Entity, keys_sprite.color);
            } else {
                keys_sprite.color = *search_res.unwrap();
            }
            keys_visibility.is_visible = is_inside_field_of_view;
        } else {
            if search_res.is_some() {
            keys_visibility.is_visible = true;
            keys_sprite.color = Color::GRAY;
            }
        }
    }
    for (Entity, mut rooms_sprite, rooms_transform, mut rooms_visibility) in rooms.iter_mut() {
        let search_res = view_shed.viewed_tiles.get(&Entity);
        let is_inside_field_of_view =
            (rooms_transform.translation - player_transform.translation).length() < view_shed.range;
        if is_inside_field_of_view {
            if search_res.is_none() {
                view_shed.viewed_tiles.insert(Entity, rooms_sprite.color);
            } else {
                rooms_sprite.color = *search_res.unwrap();
            }
            rooms_visibility.is_visible = is_inside_field_of_view;
        } else {
            if search_res.is_some() {
            rooms_visibility.is_visible = true;
            rooms_sprite.color = Color::GRAY;
            }
        }
    }
    for (Entity, mut block_sprite, block_transform, mut block_visibilityy) in block_path.iter_mut()
    {
        let search_res = view_shed.viewed_tiles.get(&Entity);
        let is_inside_field_of_view =
            (block_transform.translation - player_transform.translation).length() < view_shed.range;
        if is_inside_field_of_view {
            if search_res.is_none() {
                view_shed.viewed_tiles.insert(Entity, block_sprite.color);
            } else {
                block_sprite.color = *search_res.unwrap();
            }
            block_visibilityy.is_visible = is_inside_field_of_view;
        } else {
            if search_res.is_some() {
            block_visibilityy.is_visible = true;
            block_sprite.color = Color::GRAY;
            }
        }
    }
    for (Entity, mut stand_sprite, stand_transform, mut stand_visibility) in stand_path.iter_mut() {
        let search_res = view_shed.viewed_tiles.get(&Entity);
        let is_inside_field_of_view =
            (stand_transform.translation - player_transform.translation).length() < view_shed.range;
        if is_inside_field_of_view {
            if search_res.is_none() {
                view_shed.viewed_tiles.insert(Entity, stand_sprite.color);
            } else {
                stand_sprite.color = *search_res.unwrap();
            }
            stand_visibility.is_visible = is_inside_field_of_view;
        } else {
            if search_res.is_some() {
            stand_visibility.is_visible = true;
            stand_sprite.color = Color::GRAY;
            }
        }
    }
}

fn derender_all_rooms(
    mut commands: Commands,
    mut floors: Query<
        (Entity, &mut Visibility),
        (
            With<FloorTile>,
            Without<WallTile>,
            Without<DoorTile>,
            Without<KeyObject>,
            Without<DecorTile>,
        ),
    >,
    mut walls: Query<
        (Entity, &mut Visibility),
        (
            With<WallTile>,
            Without<FloorTile>,
            Without<DoorTile>,
            Without<KeyObject>,
            Without<DecorTile>,
        ),
    >,
    mut doors: Query<
        (Entity, &mut Visibility),
        (
            With<DoorTile>,
            Without<WallTile>,
            Without<FloorTile>,
            Without<KeyObject>,
            Without<DecorTile>,
        ),
    >,
    mut keys: Query<
        (Entity, &mut Visibility),
        (
            With<KeyObject>,
            Without<WallTile>,
            Without<DoorTile>,
            Without<FloorTile>,
            Without<DecorTile>,
        ),
    >,
    mut decor: Query<
        (Entity, &mut Visibility),
        (
            With<DecorTile>,
            Without<WallTile>,
            Without<DoorTile>,
            Without<KeyObject>,
            Without<FloorTile>,
        ),
    >,
    //mut fog: Query<(&mut Visibility, Entity), With<Fog>>,
) {
    for (floors_entity, mut floors_visibility) in floors.iter_mut() {
        floors_visibility.is_visible = false;
    }
    for (walls_entity, mut walls_visibility) in walls.iter_mut() {
        walls_visibility.is_visible = false;
    }
    for (doors_entity, mut doors_visibility) in doors.iter_mut() {
        doors_visibility.is_visible = false;
    }
    for (keys_entity, mut keys_visibility) in keys.iter_mut() {
        keys_visibility.is_visible = false;
    }
    for (decor_entity, mut decor_visibility) in decor.iter_mut() {
        decor_visibility.is_visible = false;
    }
    /*for (mut v, _e) in fog.iter_mut() {
        v.is_visible = false;
    }*/
}

/*fn render_fog(
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
}*/