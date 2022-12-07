
use bevy::{prelude::*
};

use crate::GameState;

pub struct MainMenuPlugin;
#[derive(Component)]
pub struct StartMenuObjects;
#[derive(Component)]
pub struct InfoButton;


impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_update(GameState::StartMenu))
        .add_system_set(SystemSet::on_enter(GameState::StartMenu)
            .with_system(render_objects)
        )
        .add_system_set(SystemSet::on_exit(GameState::StartMenu)
            .with_system(derender_objects)
        );
    }
}
//We don't despawn the start menu, which isn't a problem for now, since we don't have
//multiple entities of it.
/*pub fn despawn_start_menu(mut commands: Commands, object_query: Query<Entity, With<StartMenuObjects>>) {
    for ent in object_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}*/

fn derender_objects(
    mut screen: Query<(&mut Visibility, Entity), With<StartMenuObjects>>,
){
    for (mut v, _e) in screen.iter_mut() {
        v.is_visible = false;
        //info!("rendering start menu");
    }
}

fn render_objects(
    mut screen: Query<(&mut Visibility, Entity), With<StartMenuObjects>>,
){
    for (mut v, _e) in screen.iter_mut() {
        v.is_visible = true;
        //info!("rendering start menu");
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){   
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
			translation: Vec3::new(-350., 200., 100.),
			..default()
		},
        texture: asset_server.load("NameoftheGame.png"),
        ..Default::default()
    }).insert(StartMenuObjects);
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-350., -200., 100.),
            ..default()
        },
    texture: asset_server.load("StartButton.png"),
    ..Default::default()
    }).insert(StartMenuObjects);
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-350., -0., 100.),
            ..default()
        },
    texture: asset_server.load("tutorial.png"),
    ..Default::default()
    }).insert(StartMenuObjects);
}  

//Code for a button sprite if we want to change to buttons
//Needs despawn with it and some adjustments to color
//Also needs a button system to function
/*pub fn spawn_info_button(
	mut commands: Commands,
    asset_server: Res<AssetServer>,
){
	let _button_entity = 
	commands
		.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(100.0)),
				position: UiRect { 
					left: Val::Px(850.0),
					top: Val::Px(40.0), 
					..default()
				},
				position_type: PositionType::Absolute,
				justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
		.with_children(|parent| {
			parent.spawn_bundle(TextBundle::from_section(
				"Press I for Info",
				TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: 40.0,
					color: Color::rgb(0.9, 0.9, 0.9),
				},
			));
		})
        .insert(InfoButton)
		.id();
}*/