use bevy::{
	prelude::*,	
};
use std::convert::From;

use crate::{
	GameState,
};

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
	//commands.spawn_bundle(Camera2dBundle::default());

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
	
	let mut i=0;
	while i < BUTTON_NUM {
		commands
		.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(100.0)),
				position: UiRect { 
					left: (Val::Px(820.+f32::from(200*(i%2)))),
					top: (Val::Px(50.+f32::from(150*(i/2)))), 
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
				"Button",
				TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: 40.0,
					color: Color::rgb(0.9, 0.9, 0.9),
				},
			));
		});

		i += 1;
	}
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
				*color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
				*color = COMBAT_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
				*color = COMBAT_BUTTON.into();
            }
        }
    }
}