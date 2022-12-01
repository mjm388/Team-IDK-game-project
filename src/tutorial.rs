
use bevy::{prelude::*
};

use crate::GameState;

pub struct TutorialPlugin;

#[derive(Component)]
struct HelpInfo;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Tutorial)
            .with_system(setup))
        .add_system_set(SystemSet::on_exit(GameState::Tutorial)
            .with_system(derender)
        );
    }
}
fn derender(
    mut commands: Commands, 
    mut screen: Query<Entity, With<HelpInfo>>) 
{
    for e in screen.iter_mut() {
        commands.entity(e).despawn_recursive();
    }
}

fn render(
    mut screen: Query<(&mut Visibility, Entity), With<HelpInfo>>,
){
    for (mut v, _e) in screen.iter_mut() {
        v.is_visible = true;
        info!("Rendering Tutorial");
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){   
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("IDK-Tutorial.png"),
        transform: Transform {
			translation: Vec3::new(-360., 0., 0.),
			..default()
		},
        visibility: Visibility {
            is_visible: true
        },
        ..default()
    })
    .insert(HelpInfo);
}   