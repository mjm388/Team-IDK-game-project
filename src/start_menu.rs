
use bevy::{prelude::*
};

use crate::GameState;

pub struct MainMenuPlugin;
#[derive(Component)]
pub struct ButtonActive(bool);


impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
        .add_system_set(SystemSet::on_update(GameState::StartMenu))
        .add_system_set(SystemSet::on_enter(GameState::StartMenu))
        .add_system_set(SystemSet::on_exit(GameState::StartMenu)
            .with_system(despawn_button)
            .with_system(despawn_name))
        ;
    }
}
fn despawn_name(mut commands: Commands, button_query: Query<Entity, With<Button>>){
    for ent in button_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
fn despawn_button(mut commands: Commands, button_query: Query<Entity, With<Button>>) {
    for ent in button_query.iter() {
        commands.entity(ent).despawn_recursive();
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
}).insert(Button);
commands.spawn_bundle(SpriteBundle {
    transform: Transform {
        translation: Vec3::new(-350., -200., 100.),
        ..default()
    },
texture: asset_server.load("StartButton.png"),
..Default::default()
}).insert(Button);
}