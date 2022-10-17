use bevy::{
	prelude::*,
};
use std::convert::From;

use crate::{
	GameState,
};

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
		)
		.add_system_set(SystemSet::on_enter(GameState::Combat)
			.with_system(set_combat)

		)
		.add_system_set(SystemSet::on_exit(GameState::Combat)
			.with_system(despawn_button)
		);
    }
}


const BUTTON_NUM: u16 = 8;
const COMBAT_BUTTON: Color = Color::rgb(0.15, 0.15, 0.235);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Background;



fn set_combat(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,	
){
	let player_handle = asset_server.load("Player_Combat.png");
	let player_atlas = TextureAtlas::from_grid(player_handle, Vec2 { x: (300.), y: (500.) }, 1, 1);
	let player_atlas_handle = texture_atlases.add(player_atlas);
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: player_atlas_handle.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform {
				translation: Vec3::new(-450., 100., 900.),
				..default()
			},
			..default()
		});

	let enemy_handle = asset_server.load("Enemy_Combat.png");
	let enemy_atlas = TextureAtlas::from_grid(enemy_handle, Vec2 { x: (300.), y: (500.) }, 1, 1);
	let enemy_atlas_handle = texture_atlases.add(enemy_atlas);
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: enemy_atlas_handle.clone(),
			sprite: TextureAtlasSprite {
				index: 0,
				..default()
			},
			transform: Transform {
				translation: Vec3::new(-50., 100., 900.),
				..default()
			},
			..default()
		});
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

fn spawn_combat_buttons(
	commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    id: CombatOptions,
    left_val: Val,
    top_val: Val,
    text: &str,
){
	let button_entity = 
	commands
		.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(100.0)),
				position: UiRect { 
					left: left_val,
					top: top_val, 
					..default()
				},
				position_type: PositionType::Absolute,
				justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: COMBAT_BUTTON.into(),
            ..default()
        })
		.with_children(|parent| {
			parent.spawn_bundle(TextBundle::from_section(
				text,
				TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: 40.0,
					color: Color::rgb(0.9, 0.9, 0.9),
				},
			));
		})
		.insert(id)
		.id();
}

fn despawn_button(mut commands: Commands, button_query: Query<Entity, With<CombatOptions>>){
    for button in button_query.iter(){
        commands.entity(button).despawn_recursive();
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
				*color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
				*color = COMBAT_BUTTON.into();
            }
            Interaction::None => {
				*color = COMBAT_BUTTON.into();
            }
        }
    }
}
//Can probably put this with the other button system
//This checks which button was clicked
fn combat_button_system2(
    mut commands: Commands,
    query: Query<(&Interaction, &CombatOptions), (Changed<Interaction>, With<Button>)>,
    //mut state: ResMut<State<GameState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Clicked{
            match button{
				//TODO: Add Combat functions
                CombatOptions::Attack => {
                    
                }
                CombatOptions::Charge => {

                }
				CombatOptions::Recover => {

                }
				CombatOptions::Heal => {

                }
				CombatOptions::Guard => {

                }
				CombatOptions::AntiMage => {

                }
				CombatOptions::Double => {

                }
				CombatOptions::Block=> {

                }
            }
        }
    }
}
