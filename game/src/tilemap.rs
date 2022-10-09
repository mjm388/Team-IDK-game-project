use bevy::prelude::*;
use rand::Rng;

const TITLE: &str = "Tiling";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const TILE_SIZE: f32 = 45.;

#[derive(Component)]
struct WallTile;

#[derive(Component)]
struct FloorTile;


pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_random_room);
    }
}

fn create_random_room(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let wall_handle = asset_server.load("WallTiles.png");
	let wall_atlas = TextureAtlas::from_grid(wall_handle, Vec2::splat(TILE_SIZE), 1, 1);
	let wall_atlas_len = wall_atlas.textures.len();
	let wall_atlas_handle = texture_atlases.add(wall_atlas);

    let floor_handle = asset_server.load("WoodFloors.png");
	let floor_atlas = TextureAtlas::from_grid(floor_handle, Vec2::splat(TILE_SIZE), 2, 1);
	let floor_atlas_len = floor_atlas.textures.len();
	let floor_atlas_handle = texture_atlases.add(floor_atlas);

    let mut rng = rand::thread_rng();
    let x_bound = WIN_W/2. - TILE_SIZE/2.;  // Create bounds on where to put in window
	let y_bound = WIN_H/2. - TILE_SIZE/2.;
    let size_lower_bound = 6;       // Create bounds on size of room
    let size_upper_bound = 10;     

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
            .insert(WallTile);
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
            .insert(WallTile);
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
            .insert(WallTile);
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
            .insert(WallTile);
        y = y - TILE_SIZE;
    }
}