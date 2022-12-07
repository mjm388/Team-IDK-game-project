
use bevy::{prelude::*
};

use crate::GameState;

pub struct EndPlugin;

#[derive(Component)]
pub struct End;

impl Plugin for EndPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup_end)
        .add_system_set(SystemSet::on_enter(GameState::End)
            .with_system(render)
        )
        .add_system_set(SystemSet::on_exit(GameState::End)
            .with_system(derender)
        );
    }
}

fn derender(
    mut screen: Query<(&mut Visibility, Entity), With<End>>,
){
    for (mut v, _e) in screen.iter_mut() {
        v.is_visible = false;
    }
}

fn render(
    mut screen: Query<(&mut Visibility, Entity), With<End>>,
){
    for (mut v, _e) in screen.iter_mut() {
        v.is_visible = true;
    }
}

fn setup_end(mut commands: Commands, asset_server: Res<AssetServer>,){   
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("End_Scene.png"),
        transform: Transform {
			translation: Vec3::new(0., 0., 900.),
			..default()
		},
        visibility: Visibility {
            is_visible: false
        },
        ..default()
    })
    .insert(End);
}   