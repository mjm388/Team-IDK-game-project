
use bevy::{prelude::*
};

use crate::GameState;

pub struct TutorialPlugin;

#[derive(Component)]
pub struct HelpInfo;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup_help_info)
        .add_system_set(SystemSet::on_enter(GameState::Tutorial)
            .with_system(render)
        )
        .add_system_set(SystemSet::on_exit(GameState::Tutorial)
            .with_system(derender)
        );
    }
}
//We don't despawn the help info which isn't a problem for now
//but if we do spawn it multiple times we will have issues, maybe
/*pub fn despawn_help_info(mut commands: Commands, object_query: Query<Entity, With<HelpInfo>>) {
    for ent in object_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}*/

fn derender(
    mut screen: Query<(&mut Visibility, Entity), With<HelpInfo>>,
){
    for (mut v, _e) in screen.iter_mut() {
        v.is_visible = false;
        //info!("Derendering Tutorial");
    }
}

fn render(
    mut screen: Query<(&mut Visibility, Entity), With<HelpInfo>>,
){
    for (mut v, _e) in screen.iter_mut() {
        v.is_visible = true;
        //info!("Rendering Tutorial");
    }
}

fn setup_help_info(mut commands: Commands, asset_server: Res<AssetServer>){   
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("IDK-Tutorial.png"),
        transform: Transform {
			translation: Vec3::new(-360., 0., 0.),
			..default()
		},
        visibility: Visibility {
            is_visible: false
        },
        ..default()
    })
    .insert(HelpInfo);
}   