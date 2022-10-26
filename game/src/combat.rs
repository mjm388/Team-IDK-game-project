use bevy::{
	prelude::*,
};

use crate::{
	GameState,
};

pub(crate) mod combat_buttons;
pub(crate) mod combat_sprites;
pub(crate) mod combat_structs;


use combat_buttons::*;

use combat_sprites::*;

use combat_structs::{
	CombatLog,
	CombatStats,
};

#[derive(Clone, Copy)]
pub enum EnemyType{
	Square,
}

#[derive(Component, PartialEq, Clone, Copy)]
pub enum CombatOptions{
	Attack,
	Charge,
	Recover,
	Heal,
	Guard,
	AntiMage,
	Double,
	Block,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin{
    fn build(&self, app: &mut App){
        app
		.add_system_set(SystemSet::on_update(GameState::Combat)
			.with_system(button_system)
			.with_system(combat_button_system2)
			.with_system(update_player_text)
			.with_system(update_enemy_text)
		)
		.add_system_set(SystemSet::on_enter(GameState::Combat)
			.with_system(set_combat)
		)
		.add_system_set(SystemSet::on_exit(GameState::Combat)
			.with_system(despawn_button)
			.with_system(despawn_enemy)
			.with_system(despawn_player)
			.with_system(despawn_background)
		);
    }
}




#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Background;

fn spawn_combat_background(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,    
){
    let background_handle = asset_server.load("Background_Combat.png");
    let background_atlas = TextureAtlas::from_grid(background_handle, Vec2 {x:(1280.), y: (720.)}, 1,1);
    let background_atlas_handle = texture_atlases.add(background_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: background_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
			transform: Transform {
				translation: Vec3::new(0., 0., 500.),
				..default()
			},
            ..default()
        })
        .insert(Background);
}

fn set_combat(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,	
){
	spawn_combat_background(&mut commands, &asset_server, &mut texture_atlases);
	let enemy_translation = Vec3::new(-50., 100., 900.);
	let enemy = EnemyType::Square;
	spawn_enemy_sprite(
		&mut commands,
		&asset_server,
		&mut texture_atlases,
		enemy_translation,
		enemy,
	);
	let player_translation = Vec3::new(-450., 100., 900.);
	spawn_player_sprite(
		&mut commands, 
		&asset_server, 
		&mut texture_atlases, 
		player_translation,
	);
	//The code below sets up the button positions using the spawn function
	let mut left = Val::Px(850.0);
	let mut top = Val::Px(80.0);
	let mut combat_opt_txt = "Attack";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::Attack,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(1050.0);
	top = Val::Px(80.0);
	combat_opt_txt = "Charge";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::Charge,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(850.0);
	top = Val::Px(200.0);
	combat_opt_txt = "Recover";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::Recover,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(1050.0);
	top = Val::Px(200.0);
	combat_opt_txt = "Heal";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::Heal,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(850.0);
	top = Val::Px(320.0);
	combat_opt_txt = "Guard";
	spawn_combat_buttons(
	&mut commands,
        &asset_server,
        CombatOptions::Guard,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(1050.0);
	top = Val::Px(320.0);
	combat_opt_txt = "Anti-Mage";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::AntiMage,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(850.0);
	top = Val::Px(440.0);
	combat_opt_txt = "Double";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::Double,
        left,
        top,
        combat_opt_txt,
	);

	left = Val::Px(1050.0);
	top = Val::Px(440.0);
	combat_opt_txt = "Block";
	spawn_combat_buttons(
		&mut commands,
        &asset_server,
        CombatOptions::Block,
        left,
        top,
        combat_opt_txt,
	);
}

fn despawn_background(
	mut commands: Commands, background_query: Query<Entity, With<Background>>
){
	for entity in background_query.iter(){
        commands.entity(entity).despawn_recursive();
    }
}